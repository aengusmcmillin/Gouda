#![cfg(target_os = "windows")]

use std::mem;
use winapi::shared::dxgi::*;
use winapi::shared::dxgiformat::*;
use winapi::shared::dxgitype::*;
use winapi::um::d3d11::*;
use winapi::Interface;
use winapi::_core::ptr::null_mut;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::FAILED;
use winapi::um::d3dcommon::*;

use self::buffers::PlatformIndexBuffer;

pub mod buffers;
pub mod shader;
pub mod texture;

pub struct PlatformRenderer {
    swap_chain: Box<IDXGISwapChain>,
    device: *mut ID3D11Device,
    device_context: *mut ID3D11DeviceContext,
    render_target: *mut ID3D11RenderTargetView,
}

impl PlatformRenderer {
    pub fn new(hwnd: HWND) -> Result<PlatformRenderer, String> {
        unsafe {
            let factory: *mut IDXGIFactory = null_mut();
            let result = CreateDXGIFactory(&IDXGIFactory::uuidof(), mem::transmute(&factory));
            if FAILED(result) {
                return Err("Failed to create".to_string());
            }

            let mut adapter: *mut IDXGIAdapter = null_mut();
            (*factory).EnumAdapters(0, &mut adapter);

            let mut adapter_output: *mut IDXGIOutput = null_mut();
            (*adapter).EnumOutputs(0, &mut adapter_output);

            let mut num_modes = 0;
            (*adapter_output).GetDisplayModeList(
                DXGI_FORMAT_R8G8B8A8_UNORM,
                DXGI_ENUM_MODES_INTERLACED,
                &mut num_modes,
                null_mut(),
            );

            let mut modes: Vec<DXGI_MODE_DESC> = Vec::new();
            for _i in 0..(num_modes) {
                modes.push(DXGI_MODE_DESC {
                    Width: 0,
                    Height: 0,
                    RefreshRate: DXGI_RATIONAL {
                        Numerator: 0,
                        Denominator: 0,
                    },
                    Format: 0,
                    ScanlineOrdering: 0,
                    Scaling: 0,
                });
            }
            (*adapter_output).GetDisplayModeList(
                DXGI_FORMAT_R8G8B8A8_UNORM,
                DXGI_ENUM_MODES_INTERLACED,
                &mut num_modes,
                modes.as_mut_ptr(),
            );

            // let mut numerator = 0;
            // let mut denominator = 0;
            // for i in 0..(num_modes) as usize {
            //     if modes[i].Width == 900 {
            //         if modes[i].Height == 900 {
            //             numerator = modes[i].RefreshRate.Numerator;
            //             denominator = modes[i].RefreshRate.Denominator;
            //         }
            //     }
            // }

            let mut adapter_desc: DXGI_ADAPTER_DESC = mem::zeroed();
            (*adapter).GetDesc(&mut adapter_desc);

            // let video_memory = adapter_desc.DedicatedVideoMemory / 1024 / 1024;
            // let desc = adapter_desc.Description;

            (*adapter_output).Release();
            (*adapter).Release();
            (*factory).Release();

            let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
                BufferDesc: DXGI_MODE_DESC {
                    Width: 0,
                    Height: 0,
                    RefreshRate: DXGI_RATIONAL {
                        Numerator: 0,
                        Denominator: 0,
                    },
                    Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                    Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
                },
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                BufferCount: 1,
                OutputWindow: hwnd,
                Windowed: 1,
                SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
                Flags: 0,
            };

            let swap_chain: Box<IDXGISwapChain> = Box::new(mem::zeroed());
            let mut swap_chain_ptr: *mut IDXGISwapChain = Box::into_raw(swap_chain);
            let device: Box<ID3D11Device> = Box::new(mem::zeroed());
            let mut device_ptr: *mut ID3D11Device = Box::into_raw(device);
            let device_context: Box<ID3D11DeviceContext> = Box::new(mem::zeroed());
            let mut device_context_ptr: *mut ID3D11DeviceContext = Box::into_raw(device_context);

            D3D11CreateDeviceAndSwapChain(
                null_mut(),
                D3D_DRIVER_TYPE_HARDWARE,
                null_mut(),
                D3D11_CREATE_DEVICE_DEBUG,
                null_mut(),
                0,
                D3D11_SDK_VERSION,
                &swap_chain_desc,
                &mut swap_chain_ptr,
                &mut device_ptr,
                null_mut(),
                &mut device_context_ptr,
            );

            let swap_chain = Box::from_raw(swap_chain_ptr);
            let device = Box::from_raw(device_ptr);
            let device_context = Box::from_raw(device_context_ptr);

            let mut blend_state_desc: D3D11_BLEND_DESC = mem::zeroed();
            blend_state_desc.RenderTarget[0].BlendEnable = 1;
            blend_state_desc.RenderTarget[0].SrcBlend = D3D11_BLEND_SRC_ALPHA;
            blend_state_desc.RenderTarget[0].DestBlend = D3D11_BLEND_INV_SRC_ALPHA;
            blend_state_desc.RenderTarget[0].BlendOp = D3D11_BLEND_OP_ADD;
            blend_state_desc.RenderTarget[0].SrcBlendAlpha = D3D11_BLEND_ONE;
            blend_state_desc.RenderTarget[0].DestBlendAlpha = D3D11_BLEND_ZERO;
            blend_state_desc.RenderTarget[0].BlendOpAlpha = D3D11_BLEND_OP_ADD;
            blend_state_desc.RenderTarget[0].RenderTargetWriteMask = 0x0f;
            let blend_state: Box<ID3D11BlendState> = Box::new(mem::zeroed());
            let mut blend_state_ptr: *mut ID3D11BlendState = Box::into_raw(blend_state);
            let result = (*device).CreateBlendState(
                mem::transmute(&blend_state_desc),
                mem::transmute(&mut blend_state_ptr),
            );
            if FAILED(result) {
                println!("FAILED {:x}", result);
            }

            let blend_factor = [1.; 4];
            (*device_context).OMSetBlendState(blend_state_ptr, &blend_factor, 0xFFFFFFFF);

            let back_buffer: Box<ID3D11Resource> = Box::new(mem::zeroed());
            let mut back_buffer_ptr: *mut ID3D11Resource = Box::into_raw(back_buffer);
            (*swap_chain).GetBuffer(
                0,
                &ID3D11Resource::uuidof(),
                mem::transmute(&mut back_buffer_ptr),
            );

            let mut render_target: Box<ID3D11RenderTargetView> = Box::new(mem::zeroed());
            let mut render_target_ptr: *mut ID3D11RenderTargetView = Box::into_raw(render_target);
            (*device).CreateRenderTargetView(
                back_buffer_ptr,
                null_mut(),
                mem::transmute(&mut render_target_ptr),
            );

            // back_buffer = Box::from_raw(back_buffer_ptr);
            render_target = Box::from_raw(render_target_ptr);

            let res = PlatformRenderer {
                swap_chain,
                device: Box::into_raw(device),
                device_context: Box::into_raw(device_context),
                render_target: Box::into_raw(render_target),
            };
            return Ok(res);
        }
    }

    pub fn begin_scene(&self) -> Option<PlatformScene> {
        let scene = PlatformScene {
            device: self.device,
            device_context: self.device_context,
            render_target: self.render_target,
            swap_chain: &self.swap_chain,
            renderer: self,
        };

        unsafe {
            (*scene.device_context)
                .ClearRenderTargetView(scene.render_target, &[0.43, 0.73, 0.36, 1.0]);
            (*scene.device_context).OMSetRenderTargets(1, &self.render_target, null_mut());
            let viewport = D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: 900.0,
                Height: 900.0,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            };
            (*scene.device_context).RSSetViewports(1, &viewport);
        }
        return Some(scene);
    }
}

pub struct PlatformScene<'a> {
    pub device: *mut ID3D11Device,
    pub device_context: *mut ID3D11DeviceContext,
    pub render_target: *mut ID3D11RenderTargetView,
    pub swap_chain: &'a Box<IDXGISwapChain>,
    pub renderer: &'a PlatformRenderer,
}

impl PlatformScene<'_> {
    pub fn end(self) {
        unsafe {
            let result = self.swap_chain.Present(1, 0);
            if FAILED(result) {
                panic!("Failed to present swap chain {:x}", result);
            }
        }
    }
    pub fn draw_indexed(&self, num_indices: u64, _index_buffer: &PlatformIndexBuffer) {
        unsafe {
            (*self.device_context).IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            (*self.device_context).DrawIndexed(num_indices as u32, 0, 0);
        }
    }

    pub fn draw_indexed_tris(&self, num_indices: u64, _index_buffer: &PlatformIndexBuffer) {
        unsafe {
            (*self.device_context).IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
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
