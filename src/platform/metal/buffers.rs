use crate::platform::metal::{Renderer, Scene};
use std::mem;
use metal::*;

pub struct IndexBuffer {
    pub data: Buffer,
}

impl IndexBuffer {
    pub fn new(renderer: &mut Renderer, indices: Vec<i16>) -> IndexBuffer {
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

pub struct VertexBuffer {
    data: Buffer,
    offset: u64,
}

impl VertexBuffer {
    pub fn new(renderer: &mut Renderer, offset: u64, position_data: Vec<f32>) -> VertexBuffer {
        let data = renderer.device.new_buffer_with_data(
            unsafe { mem::transmute(position_data.as_ptr()) },
            (position_data.len() * mem::size_of::<f32>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        );
        return VertexBuffer {
            offset,
            data,
        }
    }

    pub fn bind(&self, scene: &Scene) {
        scene.encoder.set_vertex_buffer(self.offset, Some(&self.data), 0);
    }

    pub fn update_data(&self, mut data: Vec<f32>) {
        unsafe {
            std::ptr::copy(data.as_mut_ptr(), mem::transmute(self.data.contents()), data.len());
        };
    }
}
