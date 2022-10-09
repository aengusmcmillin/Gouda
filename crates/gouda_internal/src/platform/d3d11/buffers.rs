use crate::rendering::buffers2::{BufferLayout};
pub use crate::rendering::{Renderer, Scene};
use winapi::um::d3d11::*;
use std::mem;
use std::ptr::null_mut;
use winapi::shared::winerror::FAILED;
use std::mem::size_of;
use winapi::shared::dxgiformat::*;

#[derive(Debug)]
pub struct VertexBuffer {
    buffer: *mut ID3D11Buffer,
    offset: u32,
    layout: BufferLayout,
}

impl VertexBuffer {
    pub fn new<T>(renderer: &Renderer, layout: BufferLayout, offset: u32, data: Vec<T>) -> VertexBuffer {
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
                SysMemSlicePitch: 0
            };
            let buffer: Box<ID3D11Buffer> = Box::new(mem::zeroed());
            let mut buffer_ptr: *mut ID3D11Buffer = Box::into_raw(buffer);
            let result = (*renderer.device).CreateBuffer(&vertex_buffer_desc, &subresource_data, &mut buffer_ptr);
            if FAILED(result) {
                panic!("Failed to create vertex buffer");
            }
            VertexBuffer {buffer: buffer_ptr, offset, layout: layout}
        }
    }

    pub fn bind(&self, scene: &Scene) {
        self.bind_to_offset(scene, self.offset);
    }

    pub fn bind_to_offset(&self, scene: &Scene, offset: u32) {
        unsafe {
            (*scene.device_context).IASetVertexBuffers(0, 1, &self.buffer, &(self.layout.stride), &offset);
        }
    }
}

#[derive(Debug)]
struct ConstantBuffer {
    buffer: *mut ID3D11Buffer,
}

impl ConstantBuffer {
    pub fn new<T>(renderer: &Renderer, data: Vec<T>) -> ConstantBuffer {
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
                SysMemSlicePitch: 0
            };
            let buffer: Box<ID3D11Buffer> = Box::new(mem::zeroed());
            let mut buffer_ptr: *mut ID3D11Buffer = Box::into_raw(buffer);
            let result = (*renderer.device).CreateBuffer(&constant_buffer_desc, &subresource_data, &mut buffer_ptr);
            if FAILED(result) {
                panic!("Failed to create constant buffer {:x}", result);
            }
            return ConstantBuffer {
                buffer: buffer_ptr,
            }
        }
    }

    pub fn update_data<T>(&self, renderer: &Renderer, data: Vec<T>) {
        let mut msr = D3D11_MAPPED_SUBRESOURCE {
            pData: null_mut(),
            RowPitch: 0,
            DepthPitch: 0
        };
        unsafe {
            let result = (*renderer.device_context).Map(mem::transmute(self.buffer), 0, D3D11_MAP_WRITE_DISCARD, 0, &mut msr);
            if FAILED(result) {
                panic!("failed to map buffer");
            }
            std::ptr::copy(data.as_ptr(), mem::transmute(msr.pData), data.len());
            (*renderer.device_context).Unmap(mem::transmute(self.buffer), 0);
        }
    }
}

#[derive(Debug)]
pub struct VertexConstantBuffer {
    buffer: ConstantBuffer,
    offset: u32,
}

impl VertexConstantBuffer {
    pub fn new<T>(renderer: &Renderer, offset: u32, data: Vec<T>) -> VertexConstantBuffer {
        VertexConstantBuffer {buffer: ConstantBuffer::new(renderer, data), offset}
    }

    pub fn update_data<T>(&self, renderer: &Renderer, data: Vec<T>) {
        self.buffer.update_data(renderer, data);
    }

    pub fn bind_to_offset(&self, scene: &Scene, offset: u32) {
        unsafe {
            (*scene.device_context).VSSetConstantBuffers(offset, 1, &self.buffer.buffer);
        }
    }

    pub fn bind(&self, scene: &Scene) {
        self.bind_to_offset(scene, self.offset);
    }
}

#[derive(Debug)]
pub struct FragmentConstantBuffer {
    buffer: ConstantBuffer,
    offset: u32,
}

impl FragmentConstantBuffer {
    pub fn new<T>(renderer: &Renderer, offset: u32, data: Vec<T>) -> FragmentConstantBuffer {
        FragmentConstantBuffer {
            buffer: ConstantBuffer::new(renderer, data),
            offset,
        }
    }

    pub fn update_data<T>(&mut self, renderer: &Renderer, data: Vec<T>) {
        self.buffer.update_data(renderer, data);
    }

    pub fn bind(&self, scene: &Scene) {
        unsafe {
            (*scene.device_context).PSSetConstantBuffers(self.offset, 1, &self.buffer.buffer);
        }
    }
}

#[derive(Debug)]
pub struct IndexBuffer {
    pub buffer: *mut ID3D11Buffer,
    pub num_indices: u64,
}

impl IndexBuffer {
    pub fn new(renderer: &Renderer, indices: Vec<u16>) -> IndexBuffer {
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
                SysMemSlicePitch: 0
            };
            let buffer: Box<ID3D11Buffer> = Box::new(mem::zeroed());
            let mut buffer_ptr: *mut ID3D11Buffer = Box::into_raw(buffer);
            let result = (*renderer.device).CreateBuffer(&index_buffer_desc, &subresource_data, &mut buffer_ptr);
            if FAILED(result) {
                panic!("Failed to create index buffer {:x}", result);
            }
            IndexBuffer {buffer: buffer_ptr, num_indices: indices.len() as u64}
        }
    }

    pub fn bind(&self, scene: &Scene) {
        unsafe {
            (*scene.device_context).IASetIndexBuffer(self.buffer, DXGI_FORMAT_R16_UINT, 0);
        }
    }

    pub fn bind_with_offset(&self, scene: &Scene, offset: u32) {
        unsafe {
            (*scene.device_context).IASetIndexBuffer(self.buffer, DXGI_FORMAT_R16_UINT, offset * 2);
        }
    }
}
