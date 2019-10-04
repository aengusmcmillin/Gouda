use crate::platform::metal::{Renderer, Scene};
use std::mem;
use metal::*;


#[derive(Debug)]
pub struct IndexBuffer {
    pub data: Buffer,
}

impl IndexBuffer {
    pub fn new(renderer: &Renderer, indices: Vec<i16>) -> IndexBuffer {
        let data = renderer.device.new_buffer_with_data(
            unsafe { mem::transmute(indices.as_ptr()) },
            (indices.len() * mem::size_of::<i16>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        );
        return IndexBuffer {
            data
        };
    }
}

#[derive(Debug)]
pub struct FragmentBuffer {
    data: Buffer,
    offset: u64,
}

fn create_buffer(renderer: &Renderer, data: Vec<f32>) -> Buffer {
    let buffer = renderer.device.new_buffer_with_data(
        unsafe { mem::transmute(data.as_ptr()) },
        (data.len() * mem::size_of::<f32>()) as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache,
    );
    return buffer;
}

fn update_buffer(buffer: &Buffer, mut data: Vec<f32>) {
    unsafe {
        std::ptr::copy(data.as_mut_ptr(), mem::transmute(buffer.contents()), data.len());
    };
}

impl FragmentBuffer {
    pub fn new(renderer: &Renderer, offset: u64, data: Vec<f32>) -> FragmentBuffer {
        return FragmentBuffer {
            offset,
            data: create_buffer(renderer, data),
        }
    }

    pub fn bind(&self, scene: &Scene) {
        scene.encoder.set_fragment_buffer(self.offset, Some(&self.data), 0);
    }

    pub fn update_data(&self, mut data: Vec<f32>) {
        update_buffer(&self.data, data);
    }
}

#[derive(Debug)]
pub struct VertexBuffer {
    data: Buffer,
    offset: u64,
}

impl VertexBuffer {
    pub fn new(renderer: &Renderer, offset: u64, position_data: Vec<f32>) -> VertexBuffer {
        return VertexBuffer {
            offset,
            data: create_buffer(renderer, position_data),
        }
    }

    pub fn bind_to_offset(&self, scene: &Scene, offset: u64) {
        scene.encoder.set_vertex_buffer(offset, Some(&self.data), 0);
    }

    pub fn bind(&self, scene: &Scene) {
        scene.encoder.set_vertex_buffer(self.offset, Some(&self.data), 0);
    }

    pub fn update_data(&self, mut data: Vec<f32>) {
        update_buffer(&self.data, data);
    }
}
