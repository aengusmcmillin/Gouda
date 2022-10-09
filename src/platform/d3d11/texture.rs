use crate::images::Image;
pub use crate::platform::d3d11::{Renderer, Scene};
pub use crate::png::PNG;
pub use crate::bmp::Bitmap;
use winapi::ctypes::c_void;
use winapi::um::d3d11::{D3D11_TEXTURE2D_DESC};
use winapi::shared::dxgiformat::*;
use winapi::shared::dxgitype::*;
use winapi::um::d3d11::*;
use std::mem;
use winapi::shared::winerror::FAILED;
use winapi::um::d3dcommon::{D3D11_SRV_DIMENSION_TEXTURE2D};

#[derive(Debug)]
pub struct RenderableTexture {
    texture: *mut ID3D11ShaderResourceView,
    sampler: *mut ID3D11SamplerState,
}

impl RenderableTexture {
    pub fn new(renderer: &Renderer, image: &Image, no_mip: bool) -> RenderableTexture {
        let texture_desc = D3D11_TEXTURE2D_DESC {
            Width: image.width as u32,
            Height: image.height as u32,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
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
            let pixels = image.raw_pixels();
            let data: *const c_void = mem::transmute(pixels.as_ptr());
            let subresource_data = D3D11_SUBRESOURCE_DATA {
                pSysMem: data,
                SysMemPitch: (image.width * 4) as u32,
                SysMemSlicePitch: 0
            };

            let texture: Box<ID3D11Texture2D> = Box::new(mem::zeroed());
            let mut texture_ptr: *mut ID3D11Texture2D = Box::into_raw(texture);
            let result = (*renderer.device).CreateTexture2D(&texture_desc, &subresource_data, &mut texture_ptr);
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
            let result = (*renderer.device).CreateShaderResourceView(mem::transmute(texture_ptr), &resource_view_desc, &mut resource_view_ptr);
            if FAILED(result) {
                panic!("Failed to create shader resource view {:x}", result);
            }

            let sampler_desc = D3D11_SAMPLER_DESC {
                Filter: if no_mip { D3D11_FILTER_MIN_MAG_LINEAR_MIP_POINT } else { D3D11_FILTER_MIN_MAG_MIP_LINEAR },
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
            let sampler: Box<ID3D11SamplerState> = Box::new(mem::zeroed());
            let mut sampler_ptr: *mut ID3D11SamplerState = Box::into_raw(sampler);
            (*renderer.device).CreateSamplerState(&sampler_desc, &mut sampler_ptr);

            RenderableTexture {texture: resource_view_ptr, sampler: sampler_ptr}
        }
    }

    pub fn new_from_png(renderer: &Renderer, png: PNG) -> RenderableTexture {
        let texture_desc = D3D11_TEXTURE2D_DESC {
            Width: png.header_chunk.width,
            Height: png.header_chunk.height,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
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

            let texture: Box<ID3D11Texture2D> = Box::new(mem::zeroed());
            let mut texture_ptr: *mut ID3D11Texture2D = Box::into_raw(texture);
            let result = (*renderer.device).CreateTexture2D(&texture_desc, &subresource_data, &mut texture_ptr);
            if FAILED(result) {
                panic!("Failed to create texture");
            }

            let resource_view: Box<ID3D11ShaderResourceView> = Box::new(mem::zeroed());
            let mut resource_view_ptr: *mut ID3D11ShaderResourceView = Box::into_raw(resource_view);
            let mut shader_desc: D3D11_SHADER_RESOURCE_VIEW_DESC_u = mem::zeroed();
            shader_desc.Texture2D_mut().MipLevels = 1;
            let resource_view_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
                u: shader_desc,
            };
            let result = (*renderer.device).CreateShaderResourceView(mem::transmute(texture_ptr), &resource_view_desc, &mut resource_view_ptr);
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
                MaxLOD: 0.0
            };
            let sampler: Box<ID3D11SamplerState> = Box::new(mem::zeroed());
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