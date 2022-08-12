#![cfg(target_os = "macos")]

use cgmath::Matrix4;
use metal::*;
use core_graphics::geometry::CGSize;
use crate::camera::{CameraT};
use crate::rendering::shapes::ShapeLibrary;
use crate::shader_lib::ShaderLibrary;
use crate::window::{GameWindowImpl};
use crate::platform::osx::osx_window::OsxWindow;
use std::f32;
use std::f32::consts::PI;
use std::mem::size_of;
use crate::platform::metal::buffers::{IndexBuffer};

use self::buffers::{VertexBuffer};
use self::shader::Shader;

pub mod drawable;
pub mod shader;
pub mod buffers;
pub mod texture;

pub struct Scene<'a> {
    encoder: &'a RenderCommandEncoderRef,
    command_buffer: &'a CommandBufferRef,
    drawable: &'a DrawableRef,
    pub renderer: &'a Renderer,
    pub camera_view_projection_matrix: Matrix4<f32>
}

pub trait Renderable {
    fn bind(&self, scene: &Scene);
    fn num_indices(&self) -> u64;
    fn index_buffer(&self) -> &IndexBuffer;
}

impl Scene<'_> {
    pub fn end(self) {
        self.encoder.end_encoding();

        self.command_buffer.present_drawable(&self.drawable);
        self.command_buffer.commit();
    }

    pub fn submit(&self, shader: &Shader, renderable: &impl Renderable, transform: Matrix4<f32>, color: [f32; 4]) {
        shader.bind(self);
        shader.upload_vertex_uniform_mat4(self, 0, self.camera_view_projection_matrix);
        shader.upload_vertex_uniform_mat4(self, 1, transform);
        shader.upload_fragment_uniform_float4(self, 0, color);

        renderable.bind(self);

        self.draw_indexed(renderable.num_indices(), renderable.index_buffer());
    }

    pub fn submit_shape_by_name(&self, shader_name: &str, shape_name: &str, transform: Matrix4<f32>, color: [f32; 4]) {
        let shader = self.renderer.shader_lib.as_ref().unwrap().get(shader_name.to_string()).unwrap();
        let shape = self.renderer.shape_lib.as_ref().unwrap().get(shape_name.to_string()).unwrap();
        self.submit(shader, shape, transform, color);
    }

    pub fn draw_tri_strip(&self, num_verts: u64) {
        self.encoder.draw_primitives(
            MTLPrimitiveType::TriangleStrip,
            0,
            num_verts,
        );
    }

    pub fn draw_triangles(&self, num_verts: u64) {
        self.encoder.draw_primitives(
            MTLPrimitiveType::Triangle,
            0,
            num_verts,
        );
    }

    pub fn draw_indexed(&self, index_count: u64, index_buffer: &IndexBuffer) {
        self.encoder.draw_indexed_primitives(
            MTLPrimitiveType::TriangleStrip,
            index_count,
            MTLIndexType::UInt16,
            &index_buffer.data,
            0
        );
    }

    pub fn draw_line(&self, index_count: u64, index_buffer: &IndexBuffer) {
        self.encoder.draw_indexed_primitives(
            MTLPrimitiveType::LineStrip,
            index_count,
            MTLIndexType::UInt16,
            &index_buffer.data,
            0
        );
    }

    pub fn bind_shader(&self, shader: String) {
        self.renderer.shader_lib.as_ref().unwrap().bind_shader(self, shader);
    }
}

fn prepare_render_pass_descriptor(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();

    color_attachment.set_texture(Some(texture));
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_clear_color(MTLClearColor::new(0.43, 0.73, 0.36, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}

pub struct Renderer {
    device: Device,
    layer: CoreAnimationLayer,
    command_queue: CommandQueue,
    width: usize,
    height: usize,
    pub shader_lib: Option<ShaderLibrary>,
    pub shape_lib: Option<ShapeLibrary>,
}

impl Renderer {
    pub fn new(window: &mut OsxWindow) -> Self {
        let device = Device::system_default().unwrap();
        let layer = CoreAnimationLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);

        let width = window.get_width();
        let height = window.get_height();
        layer.set_drawable_size(CGSize::new(
            width as f64,
            height as f64,
        ));

        let command_queue = device.new_command_queue();


        let mut res = Renderer {
            device,
            layer,
            command_queue,
            width,
            height,
            shader_lib: None,
            shape_lib: None,
        };

        let shader_lib = ShaderLibrary::construct(&res);
        res.shader_lib = Some(shader_lib);

        let shape_lib = ShapeLibrary::construct(&res);
        res.shape_lib = Some(shape_lib);
        return res;
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width as usize;
        self.height = height as usize;
        println!("Resizing {} {}", width, height);
        self.layer.set_drawable_size(CGSize::new(
            width as f64,
            height as f64,
        ));
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn begin_scene(&self, camera: Box<dyn CameraT>) -> Option<Scene> {
        if let Some(drawable) = self.layer.next_drawable() {
            let render_pass_descriptor = RenderPassDescriptor::new();
            prepare_render_pass_descriptor(&render_pass_descriptor, drawable.texture());

            let command_buffer = self.command_queue.new_command_buffer();

            let encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);
            let scene = Scene {encoder, command_buffer, drawable, renderer: self, camera_view_projection_matrix: camera.get_view_projection_matrix()};
            return Some(scene);
        } else {
            return None;
        }
    }

    pub fn end_scene(&self, scene: Scene) {
        scene.end();
    }

    pub fn get_layer(&self) -> &CoreAnimationLayerRef {
        return self.layer.as_ref();
    }
}

pub trait Sizeable<T> {
    fn size() -> usize {
        return size_of::<T>()
    }
    fn stride() -> usize {
        return size_of::<T>()
    }
}

pub struct Vertex {
    _pos: Float3,
    _color: Float4,
}

impl Vertex {
    pub fn new(pos: Float3, color: Float4) -> Vertex { return Vertex { _pos: pos, _color: color} }
}

impl Sizeable<Vertex> for Vertex {}

pub type Float2 = [f32; 2];
impl Sizeable<Float2> for Float2 {}
pub type Float3 = [f32; 3];
pub type Float4 = [f32; 4];
