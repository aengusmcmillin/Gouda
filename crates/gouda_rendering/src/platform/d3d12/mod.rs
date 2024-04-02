#[cfg(all(target_os = "windows", feature="use_d3d12"))]

use std::mem;
use std::ptr::null_mut;

use winapi::shared::dxgi::*;
use winapi::shared::dxgi1_4::*;
use winapi::shared::dxgiformat::*;
use winapi::shared::dxgitype::*;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d3d12::*;
use winapi::um::d3dcommon::D3D_FEATURE_LEVEL_11_0;
use winapi::Interface;

pub mod buffers;
pub mod shader;
pub mod texture;

pub struct PlatformRenderer {
    swap_chain: Box<IDXGISwapChain3>,
}

struct D3D12Pipeline {
    device_context: *mut ID3D12Device,
    render_target: *mut ID3D12Resource,
}

impl PlatformRenderer {
    pub fn new() {
        unsafe {
            let device: Box<ID3D12Device> = Box::new(mem::zeroed());
            let mut device_ptr: *mut ID3D12Device = Box::into_raw(device);

            D3D12CreateDevice(null_mut(), D3D_FEATURE_LEVEL_11_0, &mut ID3D12Device::uuidof(), mem::transmute(&mut device_ptr));
        }
    }
}

pub struct Texture {
    
}

fn create_texture() {

}