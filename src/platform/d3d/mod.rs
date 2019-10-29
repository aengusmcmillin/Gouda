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

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { x, y, z }
    }
}

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
                if modes[i].Width == 900 {
                    if modes[i].Height == 900 {
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
                    Width: 0,
                    Height: 0,
                    RefreshRate: DXGI_RATIONAL { Numerator: 0, Denominator: 0 },
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

            D3D11CreateDeviceAndSwapChain(null_mut(), D3D_DRIVER_TYPE_HARDWARE, null_mut(), D3D11_CREATE_DEVICE_DEBUG, null_mut(), 0,
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
            (*scene.device_context).ClearRenderTargetView(scene.render_target, &[0.0, 0.0, 0., 1.]);
            (*scene.device_context).OMSetRenderTargets(1, &self.render_target, null_mut());
            let viewport = D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: 900.0,
                Height: 900.0,
                MinDepth: 0.0,
                MaxDepth: 1.0
            };
            (*scene.device_context).RSSetViewports(1, &viewport);
        }
        return Some(scene);
    }

    pub fn end_scene(&self, scene: Scene) {
        unsafe {
            let result = self.swap_chain.Present(1, 0);
            if FAILED(result) {
                panic!("Failed to present swap chain {:x}", result);
            }
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
        unsafe {
            (*self.device_context).IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            (*self.device_context).DrawIndexed(num_indices as u32, 0, 0);
        }
    }

    pub fn draw_tri_strip(&self, num_verts: u64) {
        unsafe {
            (*self.device_context).IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            (*self.device_context).Draw(num_verts as u32, 0);
        }
    }

    pub fn draw_triangles(&self, num_verts: u64) {
        unsafe {
            (*self.device_context).IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            (*self.device_context).Draw(num_verts as u32, 0);
        }
    }
}

pub mod buffers;

pub mod shader;

pub mod texture {
    pub use crate::platform::d3d::{Renderer, Scene};
    pub use crate::png::PNG;
    pub use crate::bmp::Bitmap;
    use winapi::um::d3d11::{ID3D11VertexShader, ID3D11PixelShader, D3D11_INPUT_ELEMENT_DESC, D3D11_INPUT_PER_VERTEX_DATA, ID3D11InputLayout, D3D11_TEXTURE2D_DESC};
    use winapi::shared::dxgi::*;
    use winapi::shared::dxgiformat::*;
    use winapi::shared::dxgitype::*;
    use winapi::um::d3d11::*;
    use std::mem;
    use winapi::_core::ptr::null_mut;
    use winapi::shared::winerror::FAILED;
    use winapi::um::d3dcommon::{D3D11_SRV_DIMENSION_BUFFER, D3D11_SRV_DIMENSION_TEXTURE2D};

    #[derive(Debug)]
    pub struct RenderableTexture {
        texture: *mut ID3D11ShaderResourceView,
        sampler: *mut ID3D11SamplerState,
    }

    impl RenderableTexture {
        pub fn new(renderer: &Renderer, bmp: Bitmap) -> RenderableTexture {
            let texture_desc = D3D11_TEXTURE2D_DESC {
                Width: bmp.header.width,
                Height: bmp.header.height,
                MipLevels: 1,
                ArraySize: 1,
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0
                },
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: D3D11_BIND_SHADER_RESOURCE,
                CPUAccessFlags: 0,
                MiscFlags: 0
            };
            unsafe {
                let contents = Box::new(bmp.raw_contents());
                let subresource_data = D3D11_SUBRESOURCE_DATA {
                    pSysMem: mem::transmute(contents.as_ptr()),
                    SysMemPitch: bmp.header.width * 4,
                    SysMemSlicePitch: 0
                };

                let mut texture: Box<ID3D11Texture2D> = Box::new(mem::zeroed());
                let mut texture_ptr: *mut ID3D11Texture2D = Box::into_raw(texture);
                let result = (*renderer.device).CreateTexture2D(&texture_desc, &subresource_data, &mut texture_ptr);
                if FAILED(result) {
                    panic!("Failed to create texture {:x}", result);
                }

                let mut resource_view: Box<ID3D11ShaderResourceView> = Box::new(mem::zeroed());
                let mut resource_view_ptr: *mut ID3D11ShaderResourceView = Box::into_raw(resource_view);
                let mut shader_desc: D3D11_SHADER_RESOURCE_VIEW_DESC_u = mem::zeroed();
                shader_desc.Texture2D_mut().MipLevels = 1;
                let mut resource_view_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
                    Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
                    u: shader_desc,
                };
                let result = (*renderer.device).CreateShaderResourceView(mem::transmute(texture_ptr), &resource_view_desc, &mut resource_view_ptr);
                if FAILED(result) {
                    panic!("Failed to create shader resource view {:x}", result);
                }

                let mut sampler_desc = D3D11_SAMPLER_DESC {
                    Filter: D3D11_FILTER_MIN_MAG_MIP_LINEAR,
                    AddressU: D3D11_TEXTURE_ADDRESS_WRAP,
                    AddressV: D3D11_TEXTURE_ADDRESS_WRAP,
                    AddressW: D3D11_TEXTURE_ADDRESS_WRAP,
                    MipLODBias: 0.0,
                    MaxAnisotropy: 0,
                    ComparisonFunc: 0,
                    BorderColor: [0., 0., 0., 0.],
                    MinLOD: 0.0,
                    MaxLOD: 0.0
                };
                let mut sampler: Box<ID3D11SamplerState> = Box::new(mem::zeroed());
                let mut sampler_ptr: *mut ID3D11SamplerState = Box::into_raw(sampler);
                let result = (*renderer.device).CreateSamplerState(&sampler_desc, &mut sampler_ptr);

                RenderableTexture {texture: resource_view_ptr, sampler: sampler_ptr}
            }
        }

        pub fn new_from_png(renderer: &Renderer, png: PNG) -> RenderableTexture {
            let texture_desc = D3D11_TEXTURE2D_DESC {
                Width: png.header_chunk.width,
                Height: png.header_chunk.height,
                MipLevels: 1,
                ArraySize: 1,
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0
                },
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: D3D11_BIND_SHADER_RESOURCE,
                CPUAccessFlags: 0,
                MiscFlags: 0
            };
            unsafe {
                let subresource_data = D3D11_SUBRESOURCE_DATA {
                    pSysMem: mem::transmute(png.data.as_ptr()),
                    SysMemPitch: png.header_chunk.width * 4,
                    SysMemSlicePitch: 0
                };

                let mut texture: Box<ID3D11Texture2D> = Box::new(mem::zeroed());
                let mut texture_ptr: *mut ID3D11Texture2D = Box::into_raw(texture);
                let result = (*renderer.device).CreateTexture2D(&texture_desc, &subresource_data, &mut texture_ptr);
                if FAILED(result) {
                    panic!("Failed to create texture");
                }

                let mut resource_view: Box<ID3D11ShaderResourceView> = Box::new(mem::zeroed());
                let mut resource_view_ptr: *mut ID3D11ShaderResourceView = Box::into_raw(resource_view);
                let mut shader_desc: D3D11_SHADER_RESOURCE_VIEW_DESC_u = mem::zeroed();
                shader_desc.Texture2D_mut().MipLevels = 1;
                let mut resource_view_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
                    Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
                    u: shader_desc,
                };
                let result = (*renderer.device).CreateShaderResourceView(mem::transmute(texture_ptr), &resource_view_desc, &mut resource_view_ptr);
                if FAILED(result) {
                    panic!("Failed to create shader resource view {:x}", result);
                }

                let mut sampler_desc = D3D11_SAMPLER_DESC {
                    Filter: D3D11_FILTER_MIN_MAG_MIP_LINEAR,
                    AddressU: D3D11_TEXTURE_ADDRESS_WRAP,
                    AddressV: D3D11_TEXTURE_ADDRESS_WRAP,
                    AddressW: D3D11_TEXTURE_ADDRESS_WRAP,
                    MipLODBias: 0.0,
                    MaxAnisotropy: 0,
                    ComparisonFunc: 0,
                    BorderColor: [0., 0., 0., 0.],
                    MinLOD: 0.0,
                    MaxLOD: 0.0
                };
                let mut sampler: Box<ID3D11SamplerState> = Box::new(mem::zeroed());
                let mut sampler_ptr: *mut ID3D11SamplerState = Box::into_raw(sampler);
                let result = (*renderer.device).CreateSamplerState(&sampler_desc, &mut sampler_ptr);
                if FAILED(result) {
                    panic!("Failed to create sampler state");
                }

                RenderableTexture {texture: resource_view_ptr, sampler: sampler_ptr}
            }
        }

        pub fn bind(&self, scene: &Scene) {
            unsafe {
                (*scene.device_context).PSSetShaderResources(0, 1, &self.texture);
                (*scene.device_context).PSSetSamplers(0, 1, &self.sampler);
            }
        }
    }
}
