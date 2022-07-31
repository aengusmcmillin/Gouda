use crate::platform::metal::{Renderer, Scene};
use std::mem;
use metal::*;
use std::marker::PhantomData;

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
        std::ptr::copy(data.as_mut_ptr(), mem::transmute(buffer.contents()), data.len());
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

    pub fn bind(&self, _scene: &Scene) {

    }
}

#[derive(Debug)]
pub struct FragmentConstantBuffer<T> {
    data: Buffer,
    offset: u64,
    phantom: PhantomData<T>,
}

impl <T> FragmentConstantBuffer<T> {
    pub fn new(renderer: &Renderer, offset: u64, data: Vec<T>) -> FragmentConstantBuffer<T> {
        return FragmentConstantBuffer {
            offset,
            data: create_buffer(renderer, data),
            phantom: PhantomData,
        }
    }

    pub fn bind(&self, scene: &Scene) {
        scene.encoder.set_fragment_buffer(self.offset, Some(&self.data), 0);
    }

    pub fn update_data(&self, _renderer: &Renderer, data: Vec<T>) {
        update_buffer(&self.data, data);
    }
}

#[derive(Debug)]
pub struct VertexConstantBuffer<T> {
    data: Buffer,
    offset: u64,
    phantom: PhantomData<T>,
}

impl <T> VertexConstantBuffer<T> {
    pub fn new(renderer: &Renderer, offset: u64, data: Vec<T>) -> VertexConstantBuffer<T> {
        return VertexConstantBuffer {
            offset,
            data: create_buffer(renderer, data),
            phantom: PhantomData,
        }
    }

    pub fn bind_to_offset(&self, scene: &Scene, offset: u64) {
        scene.encoder.set_vertex_buffer(offset + 1, Some(&self.data), 0);
    }

    pub fn bind(&self, scene: &Scene) {
        scene.encoder.set_vertex_buffer(self.offset + 1, Some(&self.data), 0);
    }

    pub fn update_data(&self, _renderer: &Renderer, data: Vec<T>) {
        update_buffer(&self.data, data);
    }
}

#[derive(Debug)]
pub struct VertexBuffer<T> {
    data: Buffer,
    offset: u64,
    phantom: PhantomData<T>,
}

impl <T> VertexBuffer <T> {
    pub fn new(renderer: &Renderer, offset: u64, position_data: Vec<T>) -> VertexBuffer<T> {
        return VertexBuffer {
            offset,
            data: create_buffer(renderer, position_data),
            phantom: PhantomData,
        }
    }

    pub fn bind_to_offset(&self, scene: &Scene, offset: u64) {
        scene.encoder.set_vertex_buffer(offset, Some(&self.data), 0);
    }

    pub fn bind(&self, scene: &Scene) {
        scene.encoder.set_vertex_buffer(self.offset, Some(&self.data), 0);
    }

    pub fn update_data(&self, _renderer: &Renderer, data: Vec<T>) {
        update_buffer(&self.data, data);
    }
}
