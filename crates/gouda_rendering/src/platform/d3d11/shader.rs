use crate::buffers::{BufferLayout, ShaderDataType};
pub use crate::platform::d3d11::PlatformScene;
use std::ffi::OsStr;
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use winapi::_core::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::shared::dxgiformat::{DXGI_FORMAT_R32G32_FLOAT, *};
use winapi::shared::winerror::FAILED;
use winapi::um::d3d11::{
    ID3D11InputLayout, ID3D11PixelShader, ID3D11VertexShader, D3D11_APPEND_ALIGNED_ELEMENT,
    D3D11_INPUT_ELEMENT_DESC, D3D11_INPUT_PER_VERTEX_DATA,
};
use winapi::um::d3dcommon::ID3DBlob;
use winapi::um::d3dcompiler::{
    D3DCompile, D3DCOMPILE_DEBUG, D3DCOMPILE_ENABLE_STRICTNESS, D3D_COMPILE_STANDARD_FILE_INCLUDE,
};

use super::PlatformRenderer;

impl ShaderDataType {
    pub fn to_d3d11(&self) -> DXGI_FORMAT {
        match self {
            ShaderDataType::Float => DXGI_FORMAT_R32_FLOAT,
            ShaderDataType::Float2 => DXGI_FORMAT_R32G32_FLOAT,
            ShaderDataType::Float3 => DXGI_FORMAT_R32G32B32_FLOAT,
            ShaderDataType::Float4 => DXGI_FORMAT_R32G32B32A32_FLOAT,
            ShaderDataType::Int => DXGI_FORMAT_R32_SINT,
            ShaderDataType::Int2 => DXGI_FORMAT_R32G32_SINT,
            ShaderDataType::Int3 => DXGI_FORMAT_R32G32B32_SINT,
            ShaderDataType::Int4 => DXGI_FORMAT_R32G32B32A32_SINT,
        }
    }
}

pub struct PlatformVertexShader {
    vertex_shader: *mut ID3D11VertexShader,
    input_layout: *mut ID3D11InputLayout,
}

impl PlatformVertexShader {
    pub fn new(
        renderer: &PlatformRenderer,
        layout: BufferLayout,
        vertex_src: &str,
    ) -> PlatformVertexShader {
        unsafe {
            let vs_buffer: Box<ID3DBlob> = Box::new(mem::zeroed());
            let mut vs_buffer_ptr: *mut ID3DBlob = Box::into_raw(vs_buffer);
            let error_buffer: Box<ID3DBlob> = Box::new(mem::zeroed());
            let mut error_buffer_ptr: *mut ID3DBlob = Box::into_raw(error_buffer);
            let short_string = win32_string_short(vertex_src);
            let data: *const c_void = mem::transmute(short_string.as_ptr());
            let result = D3DCompile(
                data,
                vertex_src.len(),
                null_mut(),
                null_mut(),
                D3D_COMPILE_STANDARD_FILE_INCLUDE,
                win32_string_short("VSMain").as_ptr() as *const i8,
                win32_string_short("vs_5_0").as_ptr() as *const i8,
                D3DCOMPILE_DEBUG | D3DCOMPILE_ENABLE_STRICTNESS,
                0,
                &mut vs_buffer_ptr,
                &mut error_buffer_ptr,
            );
            let error_buffer: Box<ID3DBlob> = Box::from_raw(error_buffer_ptr);
            if FAILED(result) {
                let len = error_buffer.GetBufferSize();
                let chars = error_buffer.GetBufferPointer() as *const u8;
                for i in 0..len {
                    print!("{}", *chars.offset(i as isize) as char);
                }
                println!("");
                panic!("Failed to compile vertex shader {:?}", vertex_src.chars());
            }
            let vs_buffer = Box::from_raw(vs_buffer_ptr);

            let vertex_shader: Box<ID3D11VertexShader> = Box::new(mem::zeroed());
            let mut vertex_shader_ptr: *mut ID3D11VertexShader = Box::into_raw(vertex_shader);
            let result = (*renderer.device).CreateVertexShader(
                vs_buffer.GetBufferPointer(),
                vs_buffer.GetBufferSize(),
                null_mut(),
                &mut vertex_shader_ptr,
            );
            if FAILED(result) {
                panic!("Failed to create vertex shader {:x}", result);
            }

            let mut first = true;
            let mut names = vec![];
            let input_desc: Vec<D3D11_INPUT_ELEMENT_DESC> = layout
                .elements
                .iter()
                .map(|element| {
                    let aligned_byte_offset: u32 = if first {
                        0
                    } else {
                        D3D11_APPEND_ALIGNED_ELEMENT
                    };
                    first = false;

                    let name = win32_string_short(element.name);
                    names.push(name);
                    D3D11_INPUT_ELEMENT_DESC {
                        SemanticName: names.last().unwrap().as_ptr() as *const i8,
                        SemanticIndex: 0,
                        Format: element.data_type.to_d3d11(),
                        InputSlot: 0,
                        AlignedByteOffset: aligned_byte_offset,
                        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                        InstanceDataStepRate: 0,
                    }
                })
                .collect();
            let input_layout: Box<ID3D11InputLayout> = Box::new(mem::zeroed());
            let mut input_layout_ptr: *mut ID3D11InputLayout = Box::into_raw(input_layout);
            let result = (*renderer.device).CreateInputLayout(
                input_desc.as_ptr(),
                layout.elements.len() as u32,
                vs_buffer.GetBufferPointer(),
                vs_buffer.GetBufferSize(),
                &mut input_layout_ptr,
            );
            if FAILED(result) {
                panic!(
                    "Failed to create input layout {:?} {:x}",
                    vertex_src.chars(),
                    result
                );
            }
            return PlatformVertexShader {
                vertex_shader: vertex_shader_ptr,
                input_layout: input_layout_ptr,
            };
        }
    }

    pub fn bind(&self, scene: &PlatformScene) {
        unsafe {
            (*scene.device_context).VSSetShader(
                mem::transmute(self.vertex_shader.as_ref()),
                null_mut(),
                0,
            );
            (*scene.device_context).IASetInputLayout(mem::transmute(self.input_layout.as_ref()));
        }
    }
}

pub struct PlatformFragmentShader {
    fragment_shader: *mut ID3D11PixelShader,
}

impl PlatformFragmentShader {
    pub fn new(renderer: &PlatformRenderer, fragment_src: &str) -> PlatformFragmentShader {
        unsafe {
            let fs_buffer: Box<ID3DBlob> = Box::new(mem::zeroed());
            let mut fs_buffer_ptr: *mut ID3DBlob = Box::into_raw(fs_buffer);
            let error_buffer: Box<ID3DBlob> = Box::new(mem::zeroed());
            let mut error_buffer_ptr: *mut ID3DBlob = Box::into_raw(error_buffer);
            let short_string = win32_string_short(fragment_src);
            let data: *const c_void = mem::transmute(short_string.as_ptr());
            let result = D3DCompile(
                data,
                fragment_src.len(),
                null_mut(),
                null_mut(),
                D3D_COMPILE_STANDARD_FILE_INCLUDE,
                win32_string_short("PSMain").as_ptr() as *const i8,
                win32_string_short("ps_5_0").as_ptr() as *const i8,
                D3DCOMPILE_DEBUG | D3DCOMPILE_ENABLE_STRICTNESS,
                0,
                &mut fs_buffer_ptr,
                &mut error_buffer_ptr,
            );
            let error_buffer: Box<ID3DBlob> = Box::from_raw(error_buffer_ptr);
            if FAILED(result) {
                let len = error_buffer.GetBufferSize();
                let chars = error_buffer.GetBufferPointer() as *const u8;
                for i in 0..len {
                    print!("{}", *chars.offset(i as isize) as char);
                }
                println!("");
                panic!("Failed to compile fragment shader {}", fragment_src);
            }
            let fs_buffer = Box::from_raw(fs_buffer_ptr);

            let fragment_shader: Box<ID3D11PixelShader> = Box::new(mem::zeroed());
            let mut fragment_shader_ptr: *mut ID3D11PixelShader = Box::into_raw(fragment_shader);
            let result = (*renderer.device).CreatePixelShader(
                fs_buffer.GetBufferPointer(),
                fs_buffer.GetBufferSize(),
                null_mut(),
                &mut fragment_shader_ptr,
            );
            if FAILED(result) {
                panic!("Failed to create fragment shader {:x}", result);
            }
            PlatformFragmentShader {
                fragment_shader: fragment_shader_ptr,
            }
        }
    }

    pub fn bind(&self, scene: &PlatformScene) {
        unsafe {
            (*scene.device_context).PSSetShader(
                mem::transmute(self.fragment_shader.as_ref()),
                null_mut(),
                0,
            );
        }
    }
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
