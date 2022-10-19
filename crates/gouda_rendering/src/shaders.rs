use std::fmt::{Error, Formatter};

use crate::buffers::{BufferLayout, FragmentConstantBuffer, VertexConstantBuffer};
use crate::camera::matrix_to_vec;
use crate::rendering_platform::shader::PlatformShader;
use crate::{Renderer, Scene};
use cgmath::Matrix4;

pub struct Shader {
    platform_shader: PlatformShader,
}

#[derive(Clone, Copy)]
pub enum ShaderUniform {
    Mat4(Matrix4<f32>),
    Float4([f32; 4]),
    Float3([f32; 3]),
    Float2([f32; 2]),
    Float(f32),
}

impl Shader {
    pub fn new(
        renderer: &Renderer,
        layout: BufferLayout,
        vertex_src: &str,
        fragment_src: &str,
    ) -> Shader {
        return Shader {
            platform_shader: PlatformShader::new(
                &renderer.platform_renderer,
                layout,
                vertex_src,
                fragment_src,
            ),
        };
    }

    pub fn bind(&self, scene: &Scene) {
        self.platform_shader.bind(scene);
    }

    pub fn upload_vertex_uniform(&self, scene: &Scene, offset: u32, uniform: ShaderUniform) {
        match uniform {
            ShaderUniform::Mat4(m) => self.upload_vertex_uniform_mat4(scene, offset, m),
            ShaderUniform::Float4(f) => {}
            ShaderUniform::Float3(f) => {}
            ShaderUniform::Float2(f) => {}
            ShaderUniform::Float(f) => self.upload_vertex_uniform_float(scene, offset, f),
        }
    }

    pub fn upload_fragment_uniform(&self, scene: &Scene, offset: u32, uniform: ShaderUniform) {
        match uniform {
            ShaderUniform::Mat4(m) => {}
            ShaderUniform::Float4(f) => self.upload_fragment_uniform_float4(scene, offset, f),
            ShaderUniform::Float3(f) => self.upload_fragment_uniform_float3(scene, offset, f),
            ShaderUniform::Float2(f) => self.upload_fragment_uniform_float2(scene, offset, f),
            ShaderUniform::Float(f) => self.upload_fragment_uniform_float(scene, offset, f),
        }
    }

    pub fn upload_vertex_uniform_mat4(&self, scene: &Scene, offset: u32, matrix: Matrix4<f32>) {
        let buffer = VertexConstantBuffer::new(scene.renderer, offset, matrix_to_vec(matrix));
        buffer.bind(scene);
    }

    pub fn upload_vertex_uniform_float(&self, scene: &Scene, offset: u32, float: f32) {
        let buffer = VertexConstantBuffer::new(scene.renderer, offset, vec![float]);
        buffer.bind(scene);
    }

    pub fn upload_fragment_uniform_float4(&self, scene: &Scene, offset: u32, uniform: [f32; 4]) {
        let buffer = FragmentConstantBuffer::new(scene.renderer, offset, uniform.to_vec());
        buffer.bind(scene);
    }

    pub fn upload_fragment_uniform_float3(&self, scene: &Scene, offset: u32, uniform: [f32; 3]) {
        let buffer = FragmentConstantBuffer::new(
            scene.renderer,
            offset,
            [uniform[0], uniform[1], uniform[2], 0.].to_vec(),
        );
        buffer.bind(scene);
    }

    pub fn upload_fragment_uniform_float2(&self, scene: &Scene, offset: u32, uniform: [f32; 2]) {
        let buffer = FragmentConstantBuffer::new(
            scene.renderer,
            offset,
            [uniform[0], uniform[1], 0., 0.].to_vec(),
        );
        buffer.bind(scene);
    }

    pub fn upload_fragment_uniform_float(&self, scene: &Scene, offset: u32, uniform: f32) {
        let buffer = FragmentConstantBuffer::new(scene.renderer, offset, [uniform].to_vec());
        buffer.bind(scene);
    }
}

impl std::fmt::Debug for Shader {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), Error> {
        return Ok(());
    }
}
