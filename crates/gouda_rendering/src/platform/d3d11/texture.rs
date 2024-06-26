pub use crate::platform::d3d11::PlatformScene;
use crate::Renderer;
use crate::TextureDesc;
pub use gouda_images::bmp::Bitmap;
pub use gouda_images::png::PNG;
use gouda_images::Image;
use std::fmt::Debug;
use std::mem;
use winapi::ctypes::c_void;
use winapi::shared::dxgiformat::*;
use winapi::shared::dxgitype::*;
use winapi::shared::winerror::FAILED;
use winapi::um::d3d11::{D3D11_TEXTURE2D_DESC, *};
use winapi::um::d3dcommon::D3D11_SRV_DIMENSION_TEXTURE2D;

use super::PlatformRenderer;
use super::RenderDevice;

#[derive(Debug)]
pub struct Texture {
    pub texture: *mut ID3D11ShaderResourceView,
    pub sampler: *mut ID3D11SamplerState,
}

impl From<TextureDesc> for D3D11_TEXTURE2D_DESC {
    fn from(desc: TextureDesc) -> D3D11_TEXTURE2D_DESC {
        D3D11_TEXTURE2D_DESC {
            Width: desc.width,
            Height: desc.height,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_SHADER_RESOURCE,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        }
    }
}

impl RenderDevice {
    pub fn create_texture(&self, texture_desc: TextureDesc, image: &Image) -> Texture {
        let texture_desc: D3D11_TEXTURE2D_DESC = texture_desc.into();
        unsafe {
            let pixels = image.raw_pixels();
            let data: *const c_void = mem::transmute(pixels.as_ptr());
            let subresource_data = D3D11_SUBRESOURCE_DATA {
                pSysMem: data,
                SysMemPitch: (image.width * 4) as u32,
                SysMemSlicePitch: 0,
            };

            let texture: Box<ID3D11Texture2D> = Box::new(mem::zeroed());
            let mut texture_ptr: *mut ID3D11Texture2D = Box::into_raw(texture);
            let result =
                (*self.device).CreateTexture2D(&texture_desc, &subresource_data, &mut texture_ptr);
            if FAILED(result) {
                panic!("Failed to create texture {:x}", result);
            }

            let resource_view: Box<ID3D11ShaderResourceView> = Box::new(mem::zeroed());
            let mut resource_view_ptr: *mut ID3D11ShaderResourceView = Box::into_raw(resource_view);
            let mut shader_desc: D3D11_SHADER_RESOURCE_VIEW_DESC_u = mem::zeroed();
            shader_desc.Texture2D_mut().MipLevels = 1;
            let resource_view_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
                u: shader_desc,
            };
            let result = (*self.device).CreateShaderResourceView(
                mem::transmute(texture_ptr),
                &resource_view_desc,
                &mut resource_view_ptr,
            );
            if FAILED(result) {
                panic!("Failed to create shader resource view {:x}", result);
            }

            let sampler_desc = D3D11_SAMPLER_DESC {
                Filter: D3D11_FILTER_MIN_MAG_MIP_LINEAR,
                AddressU: D3D11_TEXTURE_ADDRESS_WRAP,
                AddressV: D3D11_TEXTURE_ADDRESS_WRAP,
                AddressW: D3D11_TEXTURE_ADDRESS_WRAP,
                MipLODBias: 0.0,
                MaxAnisotropy: 0,
                ComparisonFunc: 0,
                BorderColor: [0., 0., 0., 0.],
                MinLOD: 0.0,
                MaxLOD: 0.0,
            };
            let sampler: Box<ID3D11SamplerState> = Box::new(mem::zeroed());
            let mut sampler_ptr: *mut ID3D11SamplerState = Box::into_raw(sampler);
            (*self.device).CreateSamplerState(&sampler_desc, &mut sampler_ptr);

            Texture {
                texture: resource_view_ptr,
                sampler: sampler_ptr,
            }
        }
    }
}

impl Texture {
    pub fn bind(&self, scene: &PlatformScene) {
        unsafe {
            (*scene.device_context).PSSetShaderResources(0, 1, &self.texture);
            (*scene.device_context).PSSetSamplers(0, 1, &self.sampler);
        }
    }
}
