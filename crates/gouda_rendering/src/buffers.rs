use crate::rendering_platform::buffers::{
    PlatformFragmentConstantBuffer, PlatformIndexBuffer, PlatformVertexBuffer,
    PlatformVertexConstantBuffer,
};
use crate::{Renderer, Scene};

#[derive(Debug)]
pub enum ShaderDataType {
    Float = 0,
    Float2,
    Float3,
    Float4,
    Int,
    Int2,
    Int3,
    Int4,
}

impl ShaderDataType {
    pub fn size(&self) -> u32 {
        match self {
            ShaderDataType::Float => 4,
            ShaderDataType::Float2 => 4 * 2,
            ShaderDataType::Float3 => 4 * 3,
            ShaderDataType::Float4 => 4 * 4,
            ShaderDataType::Int => 4,
            ShaderDataType::Int2 => 4 * 2,
            ShaderDataType::Int3 => 4 * 3,
            ShaderDataType::Int4 => 4 * 4,
        }
    }
}

#[derive(Debug)]
pub struct BufferLayout {
    pub elements: Vec<BufferElement>,
    pub stride: u32,
}

impl BufferLayout {
    pub fn new(elements: Vec<BufferElement>) -> BufferLayout {
        let mut res = BufferLayout {
            elements,
            stride: 0,
        };
        res.calculate_offsets_and_stride();
        return res;
    }

    fn calculate_offsets_and_stride(&mut self) {
        let mut offset = 0;
        self.stride = 0;
        for element in self.elements.iter_mut() {
            element.offset = offset;
            offset += element.size;
            self.stride += element.size;
        }
    }
}

#[derive(Debug)]
pub struct BufferElement {
    pub name: &'static str,
    pub data_type: ShaderDataType,
    pub offset: u32,
    pub size: u32,
    pub normalized: bool,
}

impl BufferElement {
    pub fn new_normalized(name: &'static str, data_type: ShaderDataType) -> BufferElement {
        let size = data_type.size();
        return BufferElement {
            name,
            data_type,
            offset: 0,
            size: size,
            normalized: true,
        };
    }

    pub fn new(name: &'static str, data_type: ShaderDataType) -> BufferElement {
        let size = data_type.size();
        return BufferElement {
            name,
            data_type,
            offset: 0,
            size: size,
            normalized: false,
        };
    }
}

#[derive(Debug)]
pub struct IndexBuffer {
    pub platform_buffer: PlatformIndexBuffer,
    pub num_indices: u64,
}

impl IndexBuffer {
    pub fn new(renderer: &Renderer, indices: Vec<u16>) -> IndexBuffer {
        let num_indices = indices.len() as u64;
        return IndexBuffer {
            platform_buffer: PlatformIndexBuffer::new(&renderer.platform_renderer, indices),
            num_indices,
        };
    }

    pub fn bind(&self, scene: &Scene) {
        self.bind_with_offset(scene, 0);
    }

    pub fn bind_with_offset(&self, scene: &Scene, offset: u32) {
        self.platform_buffer
            .bind_with_offset(&scene.platform_scene, offset);
    }
}

#[derive(Debug)]
pub struct VertexBuffer {
    pub platform_vertex_buffer: PlatformVertexBuffer,
}

impl VertexBuffer {
    pub fn new<T>(
        renderer: &Renderer,
        layout: BufferLayout,
        offset: u32,
        data: Vec<T>,
    ) -> VertexBuffer {
        return VertexBuffer {
            platform_vertex_buffer: PlatformVertexBuffer::new(
                &renderer.platform_renderer,
                layout,
                offset,
                data,
            ),
        };
    }

    pub fn bind(&self, scene: &Scene) {
        self.platform_vertex_buffer.bind(&scene.platform_scene);
    }

    pub fn bind_to_offset(&self, scene: &Scene, offset: u32) {
        self.platform_vertex_buffer
            .bind_to_offset(&scene.platform_scene, offset);
    }
}

#[derive(Debug)]
pub struct VertexConstantBuffer {
    pub platform_buffer: PlatformVertexConstantBuffer,
}

impl VertexConstantBuffer {
    pub fn new<T>(renderer: &Renderer, offset: u32, data: Vec<T>) -> VertexConstantBuffer {
        return VertexConstantBuffer {
            platform_buffer: PlatformVertexConstantBuffer::new(
                &renderer.platform_renderer,
                offset,
                data,
            ),
        };
    }

    pub fn update_data<T>(&self, renderer: &Renderer, data: Vec<T>) {
        self.platform_buffer
            .update_data(&renderer.platform_renderer, data);
    }

    pub fn bind_to_offset(&self, scene: &Scene, offset: u32) {
        self.platform_buffer
            .bind_to_offset(&scene.platform_scene, offset);
    }

    pub fn bind(&self, scene: &Scene) {
        self.platform_buffer.bind(&scene.platform_scene);
    }
}

#[derive(Debug)]
pub struct FragmentConstantBuffer {
    pub platform_buffer: PlatformFragmentConstantBuffer,
}

impl FragmentConstantBuffer {
    pub fn new<T>(renderer: &Renderer, offset: u32, data: Vec<T>) -> FragmentConstantBuffer {
        return FragmentConstantBuffer {
            platform_buffer: PlatformFragmentConstantBuffer::new(
                &renderer.platform_renderer,
                offset,
                data,
            ),
        };
    }

    pub fn update_data<T>(&mut self, renderer: &Renderer, data: Vec<T>) {
        self.platform_buffer
            .update_data(&renderer.platform_renderer, data);
    }

    pub fn bind(&self, scene: &Scene) {
        self.platform_buffer.bind(&scene.platform_scene);
    }
}
