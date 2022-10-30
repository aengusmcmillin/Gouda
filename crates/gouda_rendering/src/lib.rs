pub mod buffers;
pub mod camera;
pub mod drawable;
pub mod font;
pub mod font_library;
pub mod material_library;
pub mod model;
pub mod obj;
pub mod platform;
pub mod shader_lib;
pub mod shaders;
pub mod shapes;
pub mod sprites;
pub mod texture_library;

use buffers::{IndexBuffer, VertexBuffer};
use camera::Camera;
use cgmath::{Matrix4, SquareMatrix};
use font::Font;
use font_library::FontLibrary;
use gouda_images::Image;
use material_library::Material;
use model::Model;
use rendering_platform::texture::PlatformTexture;
use rendering_platform::{PlatformRenderer, PlatformScene};
use shader_lib::imgui_shader::imgui_shader_layout;
use shader_lib::ShaderLibrary;
use shaders::{Shader, ShaderUniform};
use shapes::{Shape2d, ShapeLibrary};
use winapi::shared::windef::HWND;

#[cfg(target_os = "macos")]
pub use crate::platform::metal as rendering_platform;

#[cfg(target_os = "windows")]
pub use crate::platform::d3d11 as rendering_platform;

pub trait Renderable {
    fn bind(&self, scene: &Scene);
    fn num_indices(&self) -> u64;
    fn index_buffer(&self) -> &IndexBuffer;
}

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { x, y, z }
    }
}

#[derive(Debug)]
pub struct Texture {
    platform_texture: PlatformTexture,
}

impl Texture {
    pub fn new(renderer: &Renderer, image: &Image) -> Texture {
        return Texture {
            platform_texture: PlatformTexture::new(&renderer.platform_renderer, image),
        };
    }

    pub fn bind(&self, scene: &Scene) {
        return self.platform_texture.bind(&scene.platform_scene);
    }
}

pub struct Renderer {
    pub platform_renderer: PlatformRenderer,
    pub shader_lib: ShaderLibrary,
    pub shape_lib: ShapeLibrary,
    pub font_lib: FontLibrary,
}

impl Renderer {
    pub fn new(hwnd: HWND) -> Result<Renderer, String> {
        let platform_renderer = PlatformRenderer::new(hwnd);

        match platform_renderer {
            Ok(platform_renderer) => {
                let mut renderer = Renderer {
                    platform_renderer,
                    shader_lib: ShaderLibrary::new(),
                    shape_lib: ShapeLibrary::new(),
                    font_lib: FontLibrary::new(),
                };
                renderer.initialize_libs();
                return Ok(renderer);
            }
            Err(e) => return Err(e),
        }
    }

    fn initialize_libs(&mut self) {
        let mut shader_lib = ShaderLibrary::new();
        shader_lib.initialize(self);
        self.shader_lib = shader_lib;
        let mut shape_lib = ShapeLibrary::new();
        shape_lib.initialize(self);
        self.shape_lib = shape_lib;
        let mut font_lib = FontLibrary::new();
        font_lib.initialize(self);
        self.font_lib = font_lib;
    }

    pub fn get_shader(&self, name: &'static str) -> &Shader {
        return self.shader_lib.get(name).unwrap();
    }

    pub fn get_shape(&self, name: &'static str) -> &Shape2d {
        return self.shape_lib.get(name).unwrap();
    }

    pub fn get_font(&self, name: &'static str) -> &Font {
        return self.font_lib.get(name).unwrap();
    }

    pub fn end_scene(&self, scene: PlatformScene) {
        scene.end();
    }

    pub fn get_width(&self) -> usize {
        return 900;
    }

    pub fn get_height(&self) -> usize {
        return 900;
    }

    pub fn begin_scene(&self) -> Option<Scene> {
        let scene = Scene {
            renderer: self,
            camera_view_projection_matrix: Matrix4::identity(),
            platform_scene: self.platform_renderer.begin_scene().unwrap(),
        };

        return Some(scene);
    }
}

pub struct Scene<'a> {
    pub renderer: &'a Renderer,
    pub camera_view_projection_matrix: Matrix4<f32>,
    pub platform_scene: PlatformScene<'a>,
}

impl Scene<'_> {
    pub fn bind_camera(&mut self, camera: &dyn Camera) {
        self.camera_view_projection_matrix = camera.get_view_projection_matrix();
    }

    pub fn unbind_camera(&mut self) {
        self.camera_view_projection_matrix = Matrix4::identity();
    }

    pub fn end(self) {
        self.platform_scene.end();
    }

    fn submit_impl(
        &self,
        shader: &Shader,
        renderable: &impl Renderable,
        transform: Matrix4<f32>,
        projection: Matrix4<f32>,
        color: [f32; 4],
    ) {
        shader.bind(&self);
        shader.upload_vertex_uniform_mat4(&self, 0, projection);
        shader.upload_vertex_uniform_mat4(&self, 1, transform);
        shader.upload_fragment_uniform_float4(&self, 0, color);

        renderable.bind(&self);

        self.draw_indexed(renderable.index_buffer());
    }

    pub fn submit(
        &self,
        shader: &Shader,
        renderable: &impl Renderable,
        transform: Matrix4<f32>,
        color: [f32; 4],
    ) {
        self.submit_impl(
            shader,
            renderable,
            transform,
            self.camera_view_projection_matrix,
            color,
        );
    }

    pub fn submit_model(&self, model: Model) {
        model
            .mesh
            .submeshes
            .iter()
            .enumerate()
            .for_each(|(index, submesh)| {
                let mat = model.materials.get(index).unwrap();
            })
    }

    pub fn bind_material(&self, material: &Material) {}

    // pub fn submit_obj(&self, obj_model: &ObjModel, transform: Matrix4<f32>) {
    //     let shader = self.renderer.get_shader("obj_model");
    //     shader.bind(&self);
    //     shader.upload_vertex_uniform_mat4(&self, 0, self.camera_view_projection_matrix);
    //     shader.upload_vertex_uniform_mat4(&self, 1, transform);

    //     obj_model.vertex_buffer.bind(&self);

    //     if let Some(no_mat_ibuf) = &obj_model.no_material_index_buffer {
    //         no_mat_ibuf.bind(&self);
    //         self.draw_indexed_tris(no_mat_ibuf.num_indices, &no_mat_ibuf);
    //     }

    //     obj_model.submeshes.iter().for_each(|submesh| {
    //         shader.upload_fragment_uniform_float3(&self, 0, submesh.ambient);
    //         shader.upload_fragment_uniform_float3(&self, 1, submesh.diffuse);
    //         shader.upload_fragment_uniform_float3(&self, 2, [3., 0.5, -2.5]);

    //         submesh.index_buffer.bind(&self);
    //         self.draw_indexed_tris(submesh.index_buffer.num_indices, &submesh.index_buffer);
    //     });
    // }

    pub fn submit_shape_gui(
        &self,
        shader_name: &'static str,
        shape_name: &'static str,
        transform: Matrix4<f32>,
        color: [f32; 4],
    ) {
        let shader = self.renderer.get_shader(shader_name);
        let shape = self.renderer.get_shape(shape_name);

        self.submit_impl(
            shader,
            shape,
            transform,
            self.camera_view_projection_matrix,
            color,
        );
    }

    pub fn submit_shape_by_name(
        &self,
        shader_name: &'static str,
        shape_name: &'static str,
        transform: Matrix4<f32>,
        color: [f32; 4],
    ) {
        let shader = self.renderer.get_shader(shader_name);
        let shape = self.renderer.get_shape(shape_name);

        self.submit_impl(
            shader,
            shape,
            transform,
            self.camera_view_projection_matrix,
            color,
        );
    }

    fn submit_texture_with_projection(
        &self,
        texture: &Texture,
        transform: Matrix4<f32>,
        projection: Matrix4<f32>,
    ) {
        let shader = self.renderer.get_shader("texture");
        let shape = self.renderer.get_shape("texture");
        texture.bind(&self);
        shader.bind(&self);
        shader.upload_vertex_uniform_mat4(&self, 0, projection);
        shader.upload_vertex_uniform_mat4(&self, 1, transform);

        shape.bind(&self);

        self.draw_indexed(shape.index_buffer());
    }

    pub fn submit_gui_texture(&self, texture: &Texture, transform: Matrix4<f32>) {
        self.submit_texture_with_projection(texture, transform, Matrix4::identity())
    }

    pub fn submit_texture(&self, texture: &Texture, transform: Matrix4<f32>) {
        self.submit_texture_with_projection(&texture, transform, self.camera_view_projection_matrix)
    }

    pub fn bind_shader(&self, shader: &'static str) {
        self.renderer.shader_lib.bind_shader(&self, shader);
    }

    pub fn bind_shader_with_uniforms(
        &self,
        shader: &'static str,
        vertex_uniforms: Vec<ShaderUniform>,
        fragment_uniforms: Vec<ShaderUniform>,
    ) {
        let shader = self.renderer.shader_lib.get(shader);
        if let Some(shader) = shader {
            for (i, uniform) in vertex_uniforms.iter().enumerate() {
                shader.upload_vertex_uniform(&self, i as u32, *uniform);
            }
            for (i, uniform) in fragment_uniforms.iter().enumerate() {
                shader.upload_fragment_uniform(&self, i as u32, *uniform);
            }
            shader.bind(&self);
        }
    }

    pub fn draw_shape(&self, shape: &'static str) {
        self.renderer.shape_lib.bind_shape(&self, shape);
        let shape = self.renderer.shape_lib.get(shape).unwrap();

        shape.bind(&self);

        self.draw_indexed(shape.index_buffer());
    }

    pub fn bind_font(&self, font: &'static str) {
        self.renderer.get_font(font).texture.bind(&self);
    }

    pub fn submit_imgui(
        &self,
        vbuf: &Vec<[f32; 8]>,
        ibuf: &[u16],
        count: usize,
        _vtx_offset: usize,
        idx_offset: usize,
        texture: &Texture,
        matrix: Matrix4<f32>,
    ) {
        self.bind_shader_with_uniforms("imgui", vec![ShaderUniform::Mat4(matrix)], vec![]);
        let vertex_buffer =
            VertexBuffer::new::<[f32; 8]>(&self.renderer, imgui_shader_layout(), 0, vbuf.clone());
        vertex_buffer.bind(&self);
        let index_buffer = IndexBuffer::new(self.renderer, ibuf.to_vec());
        index_buffer.bind_with_offset(&self, idx_offset as u32);
        texture.bind(&self);

        self.draw_indexed_tris(count as u64, &index_buffer);
    }

    pub fn draw_indexed(&self, index_buffer: &buffers::IndexBuffer) {
        self.platform_scene
            .draw_indexed(index_buffer.num_indices, &index_buffer.platform_buffer);
    }

    pub fn draw_indexed_tris(&self, num_indices: u64, index_buffer: &buffers::IndexBuffer) {
        self.platform_scene
            .draw_indexed_tris(num_indices, &index_buffer.platform_buffer);
    }

    pub fn draw_tri_strip(&self, num_verts: u64) {
        self.platform_scene.draw_tri_strip(num_verts);
    }

    pub fn draw_triangles(&self, num_verts: u64) {
        self.platform_scene.draw_triangles(num_verts);
    }
}
