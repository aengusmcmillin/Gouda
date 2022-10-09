use crate::platform::buffers2::ShaderDataType;
use crate::platform::metal::{Renderer, Scene};
use metal::*;
use std::mem;

impl ShaderDataType {
    pub fn to_metal(&self) -> MTLVertexFormat {
        match self {
            ShaderDataType::Float => MTLVertexFormat::Float,
            ShaderDataType::Float2 => MTLVertexFormat::Float2,
            ShaderDataType::Float3 => MTLVertexFormat::Float3,
            ShaderDataType::Float4 => MTLVertexFormat::Float4,
            ShaderDataType::Int => MTLVertexFormat::Int,
            ShaderDataType::Int2 => MTLVertexFormat::Int2,
            ShaderDataType::Int3 => MTLVertexFormat::Int3,
            ShaderDataType::Int4 => MTLVertexFormat::Int4,
        }
    }
}

fn create_buffer<T>(renderer: &Renderer, data: Vec<T>) -> Buffer {
    let buffer = renderer.device.new_buffer_with_data(
        unsafe { mem::transmute(data.as_ptr()) },
        (data.len() * mem::size_of::<T>()) as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache,
    );
    return buffer;
}

fn update_buffer<T>(buffer: &Buffer, mut data: Vec<T>) {
    unsafe {
        std::ptr::copy(
            data.as_mut_ptr(),
            mem::transmute(buffer.contents()),
            data.len(),
        );
    };
}

#[derive(Debug)]
pub struct IndexBuffer {
    pub data: Buffer,
}

impl IndexBuffer {
    pub fn new(renderer: &Renderer, indices: Vec<i16>) -> IndexBuffer {
        return IndexBuffer {
            data: create_buffer(renderer, indices),
        };
    }

    pub fn bind(&self, _scene: &Scene) {}
}

#[derive(Debug)]
pub struct FragmentConstantBuffer {
    data: Buffer,
    offset: u64,
}

impl FragmentConstantBuffer {
    pub fn new<T>(renderer: &Renderer, offset: u64, data: Vec<T>) -> FragmentConstantBuffer {
        return FragmentConstantBuffer {
            offset,
            data: create_buffer(renderer, data),
        };
    }

    pub fn bind(&self, scene: &Scene) {
        scene
            .encoder
            .set_fragment_buffer(self.offset, Some(&self.data), 0);
    }

    pub fn update_data<T>(&self, data: Vec<T>) {
        update_buffer(&self.data, data);
    }
}

#[derive(Debug)]
pub struct VertexConstantBuffer {
    data: Buffer,
    offset: u64,
}

impl VertexConstantBuffer {
    pub fn new<T>(renderer: &Renderer, offset: u64, data: Vec<T>) -> VertexConstantBuffer {
        return VertexConstantBuffer {
            offset,
            data: create_buffer(renderer, data),
        };
    }

    pub fn bind_to_offset(&self, scene: &Scene, offset: u64) {
        scene
            .encoder
            .set_vertex_buffer(offset + 1, Some(&self.data), 0);
    }

    pub fn bind(&self, scene: &Scene) {
        scene
            .encoder
            .set_vertex_buffer(self.offset + 1, Some(&self.data), 0);
    }

    pub fn update_data<T>(&self, data: Vec<T>) {
        update_buffer(&self.data, data);
    }
}

#[derive(Debug)]
pub struct VertexBuffer {
    data: Buffer,
    offset: u64,
}

impl VertexBuffer {
    pub fn new<T>(renderer: &Renderer, offset: u64, position_data: Vec<T>) -> VertexBuffer {
        return VertexBuffer {
            offset,
            data: create_buffer(renderer, position_data),
        };
    }

    pub fn bind_to_offset(&self, scene: &Scene, offset: u64) {
        scene.encoder.set_vertex_buffer(offset, Some(&self.data), 0);
    }

    pub fn bind(&self, scene: &Scene) {
        scene
            .encoder
            .set_vertex_buffer(self.offset, Some(&self.data), 0);
    }

    pub fn update_data<T>(&self, _renderer: &Renderer, data: Vec<T>) {
        update_buffer(&self.data, data);
    }
}
