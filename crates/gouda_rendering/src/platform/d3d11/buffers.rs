use crate::buffers::BufferLayout;
pub use crate::Renderer;
use std::mem;
use std::mem::size_of;
use std::ptr::null_mut;
use winapi::shared::dxgiformat::*;
use winapi::shared::winerror::FAILED;
use winapi::um::d3d11::*;

use super::{PlatformRenderer, PlatformScene};

#[derive(Debug)]
pub struct PlatformVertexBuffer {
    buffer: *mut ID3D11Buffer,
    offset: u32,
    layout: BufferLayout,
}

impl PlatformVertexBuffer {
    pub fn new<T>(
        renderer: &PlatformRenderer,
        layout: BufferLayout,
        offset: u32,
        data: Vec<T>,
    ) -> PlatformVertexBuffer {
        unsafe {
            let vertex_buffer_desc = D3D11_BUFFER_DESC {
                ByteWidth: (size_of::<T>() * data.len()) as u32,
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: D3D11_BIND_VERTEX_BUFFER,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: (size_of::<T>()) as u32,
            };
            let subresource_data = D3D11_SUBRESOURCE_DATA {
                pSysMem: mem::transmute(data.as_ptr()),
                SysMemPitch: 0,
                SysMemSlicePitch: 0,
            };
            let buffer: Box<ID3D11Buffer> = Box::new(mem::zeroed());
            let mut buffer_ptr: *mut ID3D11Buffer = Box::into_raw(buffer);
            let result = (*renderer.device).CreateBuffer(
                &vertex_buffer_desc,
                &subresource_data,
                &mut buffer_ptr,
            );
            if FAILED(result) {
                panic!("Failed to create vertex buffer");
            }
            PlatformVertexBuffer {
                buffer: buffer_ptr,
                offset,
                layout: layout,
            }
        }
    }

    pub fn bind(&self, scene: &PlatformScene) {
        self.bind_to_offset(scene, self.offset);
    }

    pub fn bind_to_offset(&self, scene: &PlatformScene, offset: u32) {
        unsafe {
            (*scene.device_context).IASetVertexBuffers(
                0,
                1,
                &self.buffer,
                &(self.layout.stride),
                &offset,
            );
        }
    }
}

#[derive(Debug)]
struct PlatformConstantBuffer {
    buffer: *mut ID3D11Buffer,
}

impl PlatformConstantBuffer {
    pub fn new<T>(renderer: &PlatformRenderer, data: Vec<T>) -> PlatformConstantBuffer {
        unsafe {
            let len = size_of::<T>() * data.len();
            let width = ((len / 16) + 1) * 16;
            let constant_buffer_desc = D3D11_BUFFER_DESC {
                ByteWidth: width as u32,
                Usage: D3D11_USAGE_DYNAMIC,
                BindFlags: D3D11_BIND_CONSTANT_BUFFER,
                CPUAccessFlags: D3D11_CPU_ACCESS_WRITE,
                MiscFlags: 0,
                StructureByteStride: 0,
            };
            let subresource_data = D3D11_SUBRESOURCE_DATA {
                pSysMem: mem::transmute(data.as_ptr()),
                SysMemPitch: 0,
                SysMemSlicePitch: 0,
            };
            let buffer: Box<ID3D11Buffer> = Box::new(mem::zeroed());
            let mut buffer_ptr: *mut ID3D11Buffer = Box::into_raw(buffer);
            let result = (*renderer.device).CreateBuffer(
                &constant_buffer_desc,
                &subresource_data,
                &mut buffer_ptr,
            );
            if FAILED(result) {
                panic!("Failed to create constant buffer {:x}", result);
            }
            return PlatformConstantBuffer { buffer: buffer_ptr };
        }
    }

    pub fn update_data<T>(&self, renderer: &PlatformRenderer, data: Vec<T>) {
        let mut msr = D3D11_MAPPED_SUBRESOURCE {
            pData: null_mut(),
            RowPitch: 0,
            DepthPitch: 0,
        };
        unsafe {
            let result = (*renderer.device_context).Map(
                mem::transmute(self.buffer),
                0,
                D3D11_MAP_WRITE_DISCARD,
                0,
                &mut msr,
            );
            if FAILED(result) {
                panic!("failed to map buffer");
            }
            std::ptr::copy(data.as_ptr(), mem::transmute(msr.pData), data.len());
            (*renderer.device_context).Unmap(mem::transmute(self.buffer), 0);
        }
    }
}

#[derive(Debug)]
pub struct PlatformVertexConstantBuffer {
    buffer: PlatformConstantBuffer,
    offset: u32,
}

impl PlatformVertexConstantBuffer {
    pub fn new<T>(
        renderer: &PlatformRenderer,
        offset: u32,
        data: Vec<T>,
    ) -> PlatformVertexConstantBuffer {
        PlatformVertexConstantBuffer {
            buffer: PlatformConstantBuffer::new(renderer, data),
            offset,
        }
    }

    pub fn update_data<T>(&self, renderer: &PlatformRenderer, data: Vec<T>) {
        self.buffer.update_data(renderer, data);
    }

    pub fn bind_to_offset(&self, scene: &PlatformScene, offset: u32) {
        unsafe {
            (*scene.device_context).VSSetConstantBuffers(offset, 1, &self.buffer.buffer);
        }
    }

    pub fn bind(&self, scene: &PlatformScene) {
        self.bind_to_offset(scene, self.offset);
    }
}

#[derive(Debug)]
pub struct PlatformFragmentConstantBuffer {
    buffer: PlatformConstantBuffer,
    offset: u32,
}

impl PlatformFragmentConstantBuffer {
    pub fn new<T>(
        renderer: &PlatformRenderer,
        offset: u64,
        data: Vec<T>,
    ) -> PlatformFragmentConstantBuffer {
        PlatformFragmentConstantBuffer {
            buffer: PlatformConstantBuffer::new(renderer, data),
            offset: offset as u32,
        }
    }

    pub fn update_data<T>(&mut self, renderer: &PlatformRenderer, data: Vec<T>) {
        self.buffer.update_data(renderer, data);
    }

    pub fn bind(&self, scene: &PlatformScene) {
        unsafe {
            (*scene.device_context).PSSetConstantBuffers(self.offset, 1, &self.buffer.buffer);
        }
    }
}

#[derive(Debug)]
pub struct PlatformIndexBuffer {
    pub buffer: *mut ID3D11Buffer,
}

impl PlatformIndexBuffer {
    pub fn new(renderer: &PlatformRenderer, indices: Vec<u16>) -> PlatformIndexBuffer {
        unsafe {
            let index_buffer_desc = D3D11_BUFFER_DESC {
                ByteWidth: (size_of::<u16>() * indices.len()) as u32,
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: D3D11_BIND_INDEX_BUFFER,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: size_of::<u16>() as u32,
            };
            let subresource_data = D3D11_SUBRESOURCE_DATA {
                pSysMem: mem::transmute(indices.as_ptr()),
                SysMemPitch: 0,
                SysMemSlicePitch: 0,
            };
            let buffer: Box<ID3D11Buffer> = Box::new(mem::zeroed());
            let mut buffer_ptr: *mut ID3D11Buffer = Box::into_raw(buffer);
            let result = (*renderer.device).CreateBuffer(
                &index_buffer_desc,
                &subresource_data,
                &mut buffer_ptr,
            );
            if FAILED(result) {
                panic!("Failed to create index buffer {:x}", result);
            }
            PlatformIndexBuffer { buffer: buffer_ptr }
        }
    }

    pub fn bind_with_offset(&self, scene: &PlatformScene, offset: u32) {
        unsafe {
            (*scene.device_context).IASetIndexBuffer(self.buffer, DXGI_FORMAT_R16_UINT, offset * 2);
        }
    }
}
