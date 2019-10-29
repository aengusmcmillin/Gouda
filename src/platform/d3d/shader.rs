pub use crate::platform::d3d::{Renderer, Scene};
use winapi::um::d3dcompiler::{D3DCompileFromFile, D3DReadFileToBlob, D3DCOMPILE_ENABLE_STRICTNESS, D3D_COMPILE_STANDARD_FILE_INCLUDE};
use winapi::_core::ptr::null_mut;
use winapi::um::d3dcommon::ID3DBlob;
use winapi::um::d3d11::{ID3D11VertexShader, ID3D11PixelShader, D3D11_INPUT_ELEMENT_DESC, D3D11_INPUT_PER_VERTEX_DATA, ID3D11InputLayout, D3D11_APPEND_ALIGNED_ELEMENT};
use std::mem;
use std::ffi::{OsStr, CString};
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use winapi::shared::winerror::{FAILED, D3D11_ERROR_FILE_NOT_FOUND, ERROR_MESSAGE_EXCEEDS_MAX_SIZE, E_INVALIDARG};
use winapi::shared::dxgiformat::DXGI_FORMAT_R32G32B32A32_FLOAT;
use winapi::shared::dxgiformat::DXGI_FORMAT_R32G32_FLOAT;
use winapi::um::errhandlingapi::GetLastError;
use winapi::_core::fmt::{Formatter, Error};

pub struct VertexShader {
    vertex_shader: Box<ID3D11VertexShader>,
    input_layout: Box<ID3D11InputLayout>,
}

impl VertexShader {
    pub fn new(renderer: &Renderer, has_textures: bool, vertex_file: &str) -> VertexShader {
        unsafe {
            println!("Creating vertex shader {}", vertex_file);
            let mut vs_buffer: Box<ID3DBlob> = Box::new(mem::zeroed());
            let mut vs_buffer_ptr: *mut ID3DBlob = Box::into_raw(vs_buffer);
            let mut error_buffer: Box<ID3DBlob> = Box::new(mem::zeroed());
            let mut error_buffer_ptr: *mut ID3DBlob = Box::into_raw(error_buffer);
            let result = D3DCompileFromFile(win32_string(format!("{}.d3d", vertex_file).as_str()).as_ptr(), null_mut(),
                                            D3D_COMPILE_STANDARD_FILE_INCLUDE, win32_string_short("main").as_ptr() as *const i8, win32_string_short("vs_4_0_level_9_1").as_ptr() as *const i8, D3DCOMPILE_ENABLE_STRICTNESS, 0, &mut vs_buffer_ptr, &mut error_buffer_ptr);
            let mut error_buffer: Box<ID3DBlob> = Box::from_raw(error_buffer_ptr);
            if FAILED(result) {
                let len = error_buffer.GetBufferSize();
                let chars = error_buffer.GetBufferPointer() as *const u8;
                for i in 0..len {
                    print!("{}", *chars.offset(i as isize) as char);
                }
                println!("");
                panic!("Failed to compile shader {:x}", result);
            }
            let mut vs_buffer = Box::from_raw(vs_buffer_ptr);

            let mut vertex_shader: Box<ID3D11VertexShader> = Box::new(mem::zeroed());
            let mut vertex_shader_ptr: *mut ID3D11VertexShader = Box::into_raw(vertex_shader);
            let result = (*renderer.device).CreateVertexShader(vs_buffer.GetBufferPointer(), vs_buffer.GetBufferSize(), null_mut(), &mut vertex_shader_ptr);
            if FAILED(result) {
                panic!("Failed to create vertex shader {:x}", result);
            }
            let vertex_shader = Box::from_raw(vertex_shader_ptr);

            let input_layout = if has_textures {
                let input_desc = [
                    D3D11_INPUT_ELEMENT_DESC {
                        SemanticName: win32_string_short("Position").as_ptr() as *const i8,
                        SemanticIndex: 0,
                        Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                        InputSlot: 0,
                        AlignedByteOffset: 0,
                        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                        InstanceDataStepRate: 0
                    },
                    D3D11_INPUT_ELEMENT_DESC {
                        SemanticName: win32_string_short("TexCoord").as_ptr() as *const i8,
                        SemanticIndex: 0,
                        Format: DXGI_FORMAT_R32G32_FLOAT,
                        InputSlot: 0,
                        AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
                        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                        InstanceDataStepRate: 0
                    },
                ];
                let mut input_layout: Box<ID3D11InputLayout> = Box::new(mem::zeroed());
                let mut input_layout_ptr: *mut ID3D11InputLayout = Box::into_raw(input_layout);
                let result = (*renderer.device).CreateInputLayout(input_desc.as_ptr(), 2, vs_buffer.GetBufferPointer(), vs_buffer.GetBufferSize(), &mut input_layout_ptr);
                if FAILED(result) {
                    panic!("Failed to create input layout {:x} {}", result, GetLastError());
                }
                Box::from_raw(input_layout_ptr)
            } else {
                let input_desc = [D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: win32_string_short("Position").as_ptr() as *const i8,
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: 0,
                    InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0
                }];
                let mut input_layout: Box<ID3D11InputLayout> = Box::new(mem::zeroed());
                let mut input_layout_ptr: *mut ID3D11InputLayout = Box::into_raw(input_layout);
                let result = (*renderer.device).CreateInputLayout(input_desc.as_ptr(), 1, vs_buffer.GetBufferPointer(), vs_buffer.GetBufferSize(), &mut input_layout_ptr);
                if FAILED(result) {
                    panic!("Failed to create input layout {:x} {}", result, GetLastError());
                }
                Box::from_raw(input_layout_ptr)
            };
            return VertexShader {
                vertex_shader,
                input_layout,
            }
        }
    }

    pub fn bind(&self, scene: &Scene) {
        unsafe {
            (*scene.device_context).VSSetShader(mem::transmute(self.vertex_shader.as_ref()), null_mut(), 0);
            (*scene.device_context).IASetInputLayout(mem::transmute(self.input_layout.as_ref()));
        }
    }
}

pub struct FragmentShader {
    fragment_shader: Box<ID3D11PixelShader>,
}

impl FragmentShader {
    pub fn new(renderer: &Renderer, fragment_file: &str) -> FragmentShader {
        unsafe {
            println!("Creating fragment shader {}", fragment_file);
            let mut fs_buffer: Box<ID3DBlob> = Box::new(mem::zeroed());
            let mut fs_buffer_ptr: *mut ID3DBlob = Box::into_raw(fs_buffer);
            let mut error_buffer: Box<ID3DBlob> = Box::new(mem::zeroed());
            let mut error_buffer_ptr: *mut ID3DBlob = Box::into_raw(error_buffer);
            let result = D3DCompileFromFile(win32_string(format!("{}.d3d", fragment_file).as_str()).as_ptr(), null_mut(),
                                            D3D_COMPILE_STANDARD_FILE_INCLUDE, win32_string_short("main").as_ptr() as *const i8, win32_string_short("ps_4_0_level_9_1").as_ptr() as *const i8, D3DCOMPILE_ENABLE_STRICTNESS, 0, &mut fs_buffer_ptr, &mut error_buffer_ptr);
            let mut error_buffer: Box<ID3DBlob> = Box::from_raw(error_buffer_ptr);
            if FAILED(result) {
                let len = error_buffer.GetBufferSize();
                let chars = error_buffer.GetBufferPointer() as *const u8;
                for i in 0..len {
                    print!("{}", *chars.offset(i as isize) as char);
                }
                println!("");
                panic!("Failed to compile shader {:x}", result);
            }
            let mut fs_buffer = Box::from_raw(fs_buffer_ptr);

            let mut fragment_shader: Box<ID3D11PixelShader> = Box::new(mem::zeroed());
            let mut fragment_shader_ptr: *mut ID3D11PixelShader = Box::into_raw(fragment_shader);
            let result = (*renderer.device).CreatePixelShader(fs_buffer.GetBufferPointer(), fs_buffer.GetBufferSize(), null_mut(), &mut fragment_shader_ptr);
            if FAILED(result) {
                panic!("Failed to create fragment shader {:x}", result);
            }
            let fragment_shader = Box::from_raw(fragment_shader_ptr);
            FragmentShader {fragment_shader}
        }
    }

    pub fn bind(&self, scene: &Scene) {
        unsafe {
            (*scene.device_context).PSSetShader(mem::transmute(self.fragment_shader.as_ref()), null_mut(), 0);
        }
    }
}
pub struct Shader {
    vertex_shader: VertexShader,
    fragment_shader: FragmentShader,
}

fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

fn win32_string_short(value: &str) -> Vec<i8> {
    let wstr = win32_string(value);
    let mut result = vec![];
    for c in wstr {
        result.push(c as i8);
    }
    return result;
}

impl Shader {
    pub fn new(renderer: &Renderer, has_textures: bool, vertex_file: &str, fragment_file: &str) -> Shader {
        return Shader {
            vertex_shader: VertexShader::new(renderer, has_textures, vertex_file),
            fragment_shader: FragmentShader::new(renderer, fragment_file),
        };
    }

    pub fn bind(&self, scene: &Scene) {
        self.vertex_shader.bind(scene);
        self.fragment_shader.bind(scene);
    }
}

impl std::fmt::Debug for Shader {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        return Ok(());
    }
}
