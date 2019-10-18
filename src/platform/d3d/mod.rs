#![cfg(target_os = "windows")]

use winapi::shared::dxgi::*;
use winapi::shared::dxgiformat::*;
use winapi::shared::dxgitype::*;
use winapi::um::d3d11::*;
use winapi::Interface;
use winapi::_core::ptr::null_mut;
use std::mem;
use winapi::shared::minwindef::UINT;
use winapi::shared::windef::{HWND};
use winapi::shared::winerror::FAILED;
use winapi::um::d3dcommon::*;
use winapi::um::d3d11::D3D11_BUFFER_DESC;

pub struct Renderer {
    swap_chain: Box<IDXGISwapChain>,
    device: *mut ID3D11Device,
    device_context: *mut ID3D11DeviceContext,
    back_buffer: Box<ID3D11Resource>,
    render_target: *mut ID3D11RenderTargetView
}

impl Renderer {
    pub fn new(hwnd: HWND) -> Result<Renderer, String> {
        unsafe {
            let mut factory: *mut IDXGIFactory = null_mut();
            let result = CreateDXGIFactory(&IDXGIFactory::uuidof(), mem::transmute(&factory));
            if FAILED(result) {
                return Err("Failed to create".to_string());
            }

            let mut adapter: *mut IDXGIAdapter = null_mut();
            (*factory).EnumAdapters(0, &mut adapter);

            let mut adapter_output: *mut IDXGIOutput = null_mut();
            (*adapter).EnumOutputs(0, &mut adapter_output);

            let mut num_modes = 0;
            (*adapter_output).GetDisplayModeList(DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_ENUM_MODES_INTERLACED, &mut num_modes, null_mut());

            let mut modes: Vec<DXGI_MODE_DESC> = Vec::new();
            for i in 0..(num_modes) {
                modes.push(DXGI_MODE_DESC {
                    Width: 0,
                    Height: 0,
                    RefreshRate: DXGI_RATIONAL { Numerator: 0, Denominator: 0 },
                    Format: 0,
                    ScanlineOrdering: 0,
                    Scaling: 0
                });
            }
            (*adapter_output).GetDisplayModeList(DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_ENUM_MODES_INTERLACED, &mut num_modes, modes.as_mut_ptr());

            let mut numerator = 0;
            let mut denominator = 0;
            for i in 0..(num_modes) as usize {
                if modes[i].Width == 640 {
                    if modes[i].Height == 480 {
                        numerator = modes[i].RefreshRate.Numerator;
                        denominator = modes[i].RefreshRate.Denominator;
                    }
                }
            }

            let mut adapter_desc: DXGI_ADAPTER_DESC = mem::zeroed();
            (*adapter).GetDesc(&mut adapter_desc);

            let video_memory = adapter_desc.DedicatedVideoMemory / 1024 / 1024;
            let desc = adapter_desc.Description;

            (*adapter_output).Release();
            (*adapter).Release();
            (*factory).Release();
            
            let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
                BufferDesc: DXGI_MODE_DESC {
                    Width: 640,
                    Height: 480,
                    RefreshRate: DXGI_RATIONAL { Numerator: numerator, Denominator: denominator },
                    Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                    Scaling: DXGI_MODE_SCALING_UNSPECIFIED
                },
                SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                BufferCount: 1,
                OutputWindow: hwnd,
                Windowed: 1,
                SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
                Flags: 0
            };

            let mut swap_chain: Box<IDXGISwapChain> = Box::new(mem::zeroed());
            let mut swap_chain_ptr: *mut IDXGISwapChain = Box::into_raw(swap_chain);
            let mut device: Box<ID3D11Device> = Box::new(mem::zeroed());
            let mut device_ptr: *mut ID3D11Device = Box::into_raw(device);
            let mut device_context: Box<ID3D11DeviceContext> = Box::new(mem::zeroed());
            let mut device_context_ptr: *mut ID3D11DeviceContext = Box::into_raw(device_context);

            let feature_level = D3D_FEATURE_LEVEL_11_0;
            D3D11CreateDeviceAndSwapChain(null_mut(), D3D_DRIVER_TYPE_HARDWARE, null_mut(), 0, &feature_level, 1,
            D3D11_SDK_VERSION, &swap_chain_desc, &mut swap_chain_ptr, &mut device_ptr, null_mut(), &mut device_context_ptr);

            let swap_chain = Box::from_raw(swap_chain_ptr);
            let device = Box::from_raw(device_ptr);
            let device_context = Box::from_raw(device_context_ptr);

            let mut back_buffer: Box<ID3D11Resource> = Box::new(mem::zeroed());
            let mut back_buffer_ptr: *mut ID3D11Resource = Box::into_raw(back_buffer);
            (*swap_chain).GetBuffer(0, &ID3D11Resource::uuidof(), mem::transmute(&mut back_buffer_ptr));

            let mut render_target: Box<ID3D11RenderTargetView> = Box::new(mem::zeroed());
            let mut render_target_ptr: *mut ID3D11RenderTargetView = Box::into_raw(render_target);
            (*device).CreateRenderTargetView(back_buffer_ptr, null_mut(), mem::transmute(&mut render_target_ptr));

            let back_buffer = Box::from_raw(back_buffer_ptr);
            let render_target = Box::from_raw(render_target_ptr);

            Ok(Renderer {
                swap_chain,
                device: Box::into_raw(device),
                device_context: Box::into_raw(device_context),
                back_buffer,
                render_target: Box::into_raw(render_target)
            })
        }

    }
    pub fn begin_scene(&self) -> Option<Scene> {
        let scene = Scene {
            device: self.device,
            device_context: self.device_context,
            render_target: self.render_target,
        };

        unsafe {
            (*scene.device_context).ClearRenderTargetView(scene.render_target, &[0.3, 0.5, 0., 1.]);
        }
        return Some(scene);
    }

    pub fn end_scene(&self, scene: Scene) {
        unsafe {
            self.swap_chain.Present(1, 0);
        }
    }
}

pub struct Scene {
    pub device: *mut ID3D11Device,
    pub device_context: *mut ID3D11DeviceContext,
    pub render_target: *mut ID3D11RenderTargetView,
}

impl Scene {
    pub fn draw_indexed(&self, num_indices: u64, index_buffer: &buffers::IndexBuffer) {
    }

    pub fn draw_tri_strip(&self, num_verts: u64) {

    }

    pub fn draw_triangles(&self, num_verts: u64) {

    }
}

pub mod buffers {
    pub use crate::platform::d3d::{Renderer, Scene};
    use winapi::um::d3d11::*;
    use std::mem;
    use std::ptr::null_mut;

    #[derive(Debug)]
    pub struct VertexBuffer {

    }

    impl VertexBuffer {
        pub fn new(renderer: &Renderer, offset: u32, data: Vec<f32>) -> VertexBuffer {

            unsafe {
                let vertex_buffer_desc = D3D11_BUFFER_DESC {
                    ByteWidth: 3 * data.len() as u32,
                    Usage: D3D11_USAGE_DEFAULT,
                    BindFlags: D3D11_BIND_VERTEX_BUFFER,
                    CPUAccessFlags: 0,
                    MiscFlags: 0,
                    StructureByteStride: 0
                };
                let mut buffer: ID3D11Buffer = mem::zeroed();
                let mut buffer_ptr: *mut ID3D11Buffer = &mut buffer;
                (*renderer.device).CreateBuffer(&vertex_buffer_desc, null_mut(), &mut buffer_ptr);
            }
            VertexBuffer {}
        }

        pub fn update_data(&self, data: Vec<f32>) {

        }

        pub fn bind(&self, scene: &Scene) {

        }

        pub fn bind_to_offset(&self, scene: &Scene, offset: u32) {

        }
    }

    #[derive(Debug)]
    pub struct FragmentBuffer {

    }

    impl FragmentBuffer {
        pub fn new(renderer: &Renderer, offset: u32, data: Vec<f32>) -> FragmentBuffer {
            FragmentBuffer {}
        }

        pub fn update_data(&mut self, data: Vec<f32>) {

        }

        pub fn bind(&self, scene: &Scene) {

        }
    }

    #[derive(Debug)]
    pub struct IndexBuffer {

    }

    impl IndexBuffer {
        pub fn new(renderer: &Renderer, data: Vec<i32>) -> IndexBuffer {
            IndexBuffer {}
        }
    }
}

pub mod shader {
    pub use crate::platform::d3d::{Renderer, Scene};

    #[derive(Debug)]
    pub struct Shader {

    }

    impl Shader {
        pub fn new(renderer: &Renderer, vertex_file: &str, fragment_file: &str) -> Shader {
            Shader {}
        }

        pub fn bind(&self, scene: &Scene) {

        }
    }
}

pub mod texture {
    pub use crate::platform::d3d::{Renderer, Scene};
    pub use crate::png::PNG;
    pub use crate::bmp::Bitmap;

    #[derive(Debug)]
    pub struct RenderableTexture {

    }

    impl RenderableTexture {
        pub fn new(renderer: &Renderer, bmp: Bitmap) -> RenderableTexture {
            RenderableTexture {}
        }

        pub fn new_from_png(renderer: &Renderer, png: PNG) -> RenderableTexture {
            RenderableTexture {}
        }

        pub fn bind(&self, scene: &Scene) {

        }
    }
}
