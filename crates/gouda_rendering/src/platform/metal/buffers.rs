use crate::{
    buffers::{BufferLayout, ShaderDataType},
    platform::metal::{PlatformRenderer, PlatformScene},
};
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

fn create_buffer<T>(renderer: &PlatformRenderer, data: Vec<T>) -> Buffer {
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
pub struct PlatformIndexBuffer {
    pub data: Buffer,
}

impl PlatformIndexBuffer {
    pub fn new(renderer: &PlatformRenderer, indices: Vec<u16>) -> PlatformIndexBuffer {
        return PlatformIndexBuffer {
            data: create_buffer(renderer, indices),
        };
    }

    pub fn bind(&self, _scene: &PlatformScene) {}

    pub fn bind_with_offset(&self, _scene: &PlatformScene, offset: u32) {}
}

#[derive(Debug)]
pub struct PlatformFragmentConstantBuffer {
    data: Buffer,
    offset: u64,
}

impl PlatformFragmentConstantBuffer {
    pub fn new<T>(
        renderer: &PlatformRenderer,
        offset: u64,
        data: Vec<T>,
    ) -> PlatformFragmentConstantBuffer {
        return PlatformFragmentConstantBuffer {
            offset,
            data: create_buffer(renderer, data),
        };
    }

    pub fn bind(&self, scene: &PlatformScene) {
        scene
            .encoder
            .set_fragment_buffer(self.offset, Some(&self.data), 0);
    }

    pub fn update_data<T>(&self, renderer: &PlatformRenderer, data: Vec<T>) {
        update_buffer(&self.data, data);
    }
}

#[derive(Debug)]
pub struct PlatformVertexConstantBuffer {
    data: Buffer,
    offset: u64,
}

impl PlatformVertexConstantBuffer {
    pub fn new<T>(
        renderer: &PlatformRenderer,
        offset: u64,
        data: Vec<T>,
    ) -> PlatformVertexConstantBuffer {
        return PlatformVertexConstantBuffer {
            offset,
            data: create_buffer(renderer, data),
        };
    }

    pub fn bind_to_offset(&self, scene: &PlatformScene, offset: u64) {
        scene
            .encoder
            .set_vertex_buffer(offset + 1, Some(&self.data), 0);
    }

    pub fn bind(&self, scene: &PlatformScene) {
        scene
            .encoder
            .set_vertex_buffer(self.offset + 1, Some(&self.data), 0);
    }

    pub fn update_data<T>(&self, renderer: &PlatformRenderer, data: Vec<T>) {
        update_buffer(&self.data, data);
    }
}

#[derive(Debug)]
pub struct PlatformVertexBuffer {
    data: Buffer,
    offset: u32,
}

impl PlatformVertexBuffer {
    pub fn new<T>(
        renderer: &PlatformRenderer,
        layout: &BufferLayout,
        offset: u32,
        position_data: Vec<T>,
    ) -> PlatformVertexBuffer {
        return PlatformVertexBuffer {
            offset,
            data: create_buffer(renderer, position_data),
        };
    }

    pub fn bind_to_offset(&self, scene: &PlatformScene, offset: u64) {
        scene.encoder.set_vertex_buffer(offset, Some(&self.data), 0);
    }

    pub fn bind(&self, scene: &PlatformScene) {
        scene
            .encoder
            .set_vertex_buffer(self.offset as u64, Some(&self.data), 0);
    }

    pub fn update_data<T>(&self, _renderer: &PlatformRenderer, data: Vec<T>) {
        update_buffer(&self.data, data);
    }
}
