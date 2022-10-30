use std::fmt::{Error, Formatter};

use crate::buffers::{BufferLayout, FragmentConstantBuffer, VertexConstantBuffer};
use crate::camera::matrix_to_vec;
use crate::rendering_platform::shader::{
    PlatformFragmentShader, PlatformShader, PlatformVertexShader,
};
use crate::{Renderer, Scene};
use cgmath::Matrix4;

pub enum UniformType {
    Mat4,
    Float4,
    Float3,
    Float2,
    Float,
}

pub struct UniformSpec {
    name: &'static str,
    uniform_type: UniformType,
}

pub struct ShaderUniformSpec {
    pub uniforms: Vec<UniformSpec>,
}

pub struct VertexShader {
    platform_vertex_shader: PlatformVertexShader,
    layout: BufferLayout,
    uniform_spec: ShaderUniformSpec,
    uniform_buffers: Vec<VertexConstantBuffer>,
}

impl VertexShader {
    pub fn new(
        renderer: &Renderer,
        layout: BufferLayout,
        uniform_spec: ShaderUniformSpec,
        vertex_src: &str,
    ) -> VertexShader {
        let uniform_buffers = uniform_spec
            .uniforms
            .iter()
            .enumerate()
            .map(|(index, spec)| {
                return match spec.uniform_type {
                    UniformType::Float => {
                        VertexConstantBuffer::new(renderer, index as u32, vec![0.])
                    }
                    UniformType::Float2 => {
                        VertexConstantBuffer::new(renderer, index as u32, vec![0.; 2])
                    }
                    UniformType::Float3 => {
                        VertexConstantBuffer::new(renderer, index as u32, vec![0.; 3])
                    }
                    UniformType::Float4 => {
                        VertexConstantBuffer::new(renderer, index as u32, vec![0.; 4])
                    }
                    UniformType::Mat4 => {
                        VertexConstantBuffer::new(renderer, index as u32, vec![0.; 16])
                    }
                };
            })
            .collect();
        return VertexShader {
            platform_vertex_shader: PlatformVertexShader::new(
                &renderer.platform_renderer,
                &layout,
                vertex_src,
            ),
            layout,
            uniform_spec,
            uniform_buffers,
        };
    }

    pub fn bind(&self, scene: &Scene) {
        self.uniform_buffers
            .iter()
            .for_each(|buffer| buffer.bind(scene));
        self.platform_vertex_shader.bind(&scene.platform_scene);
    }
}

pub struct FragmentShader {
    platform_fragment_shader: PlatformFragmentShader,
    uniform_spec: ShaderUniformSpec,
    uniform_buffers: Vec<FragmentConstantBuffer>,
}

impl FragmentShader {
    pub fn new(
        renderer: &Renderer,
        uniform_spec: ShaderUniformSpec,
        fragment_src: &str,
    ) -> FragmentShader {
        let uniform_buffers = uniform_spec
            .uniforms
            .iter()
            .enumerate()
            .map(|(index, spec)| {
                return match spec.uniform_type {
                    UniformType::Float => {
                        FragmentConstantBuffer::new(renderer, index as u32, vec![0.])
                    }
                    UniformType::Float2 => {
                        FragmentConstantBuffer::new(renderer, index as u32, vec![0.; 2])
                    }
                    UniformType::Float3 => {
                        FragmentConstantBuffer::new(renderer, index as u32, vec![0.; 3])
                    }
                    UniformType::Float4 => {
                        FragmentConstantBuffer::new(renderer, index as u32, vec![0.; 4])
                    }
                    UniformType::Mat4 => {
                        FragmentConstantBuffer::new(renderer, index as u32, vec![0.; 16])
                    }
                };
            })
            .collect();
        return FragmentShader {
            platform_fragment_shader: PlatformFragmentShader::new(
                &renderer.platform_renderer,
                fragment_src,
            ),
            uniform_spec,
            uniform_buffers,
        };
    }

    pub fn bind(&self, scene: &Scene) {
        self.platform_fragment_shader.bind(&scene.platform_scene);
    }
}

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
        vertex_uniform_spec: ShaderUniformSpec,
        fragment_uniform_spec: ShaderUniformSpec,
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
