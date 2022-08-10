#![cfg(target_os = "macos")]

use metal::*;
use core_graphics::geometry::CGSize;
use crate::shader_lib::ShaderLibrary;
use crate::window::{GameWindowImpl};
use crate::platform::osx::osx_window::OsxWindow;
use std::f32;
use std::f32::consts::PI;
use std::mem::size_of;
use crate::platform::metal::buffers::{IndexBuffer};

use self::buffers::{VertexBuffer};

pub mod drawable;
pub mod shader;
pub mod buffers;
pub mod texture;

pub struct Scene<'a> {
    encoder: &'a RenderCommandEncoderRef,
    command_buffer: &'a CommandBufferRef,
    drawable: &'a DrawableRef,
    pub renderer: &'a Renderer,
}

impl Scene<'_> {
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
        };

        let shader_lib = ShaderLibrary::construct(&res);
        res.shader_lib = Some(shader_lib);
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

    pub fn begin_scene(&self) -> Option<Scene> {
        if let Some(drawable) = self.layer.next_drawable() {
            let render_pass_descriptor = RenderPassDescriptor::new();
            prepare_render_pass_descriptor(&render_pass_descriptor, drawable.texture());

            let command_buffer = self.command_queue.new_command_buffer();

            let encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);
            let scene = Scene {encoder, command_buffer, drawable, renderer: self};
            return Some(scene);
        } else {
            return None;
        }
    }

    pub fn end_scene(&self, scene: Scene) {
        // let cx = 0.;
        // let cy = 0.;
        // let size = 0.08;
        // let dw = 2. * size * 0.75;
        // let dh = (3. as f32).sqrt() * size;

        // for q in 0..10 {
        //     for r in 0..10 {
        //         if q % 2 == 0 {
        //             self.draw_hex(&scene, size, [cx + q as f32 * dw, cy + (r as f32 + 0.5) * dh]);
        //         } else {
        //             self.draw_hex(&scene, size, [cx + q as f32 * dw, cy + r as f32 * dh]);
        //         }
        //     }
        // }

        scene.encoder.end_encoding();

        scene.command_buffer.present_drawable(&scene.drawable);
        scene.command_buffer.commit();
    }

    pub fn draw_triangle(&self, scene: &Scene) {
        let verts: Vec<Vertex> = vec![
                Vertex::new([0., 1., 0.], [1., 0., 0., 1.]),
                Vertex::new([-1., -1., 0.], [0., 1., 0., 1.]),
                Vertex::new([1., -1., 0.], [0., 0., 1., 1.]),
            ];
        let num_verts = verts.len() as u64;
        let vb = VertexBuffer::new(self, 0, verts);
        scene.bind_shader("basic".to_string());
        vb.bind(scene);
        scene.draw_triangles(num_verts);
    }

    pub fn draw_hex(&self, scene: &Scene, size: f32, center: [f32; 2]) {
        let verts = vec![
            center,
            flat_hex_corner(0., center, size),
            flat_hex_corner(1., center, size),
            flat_hex_corner(2., center, size),
            flat_hex_corner(3., center, size),
            flat_hex_corner(4., center, size),
            flat_hex_corner(5., center, size),
        ];
        let indices = vec![
            0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1
        ];
        let num_indices = indices.len() as u64;
        let index_buffer = IndexBuffer::new(self, indices);
        let vb = VertexBuffer::new(self, 0, verts);
        scene.bind_shader("hex".to_string());
        vb.bind(scene);
        scene.draw_indexed(num_indices, &index_buffer);
    }

    pub fn get_layer(&self) -> &CoreAnimationLayerRef {
        return self.layer.as_ref();
    }
}

fn flat_hex_corner(i: f32, center: [f32; 2], size: f32) -> [f32; 2] {
    let deg = 60. * i;
    let rad = PI / 180. * deg;

    return [center[0] + 0.95 * size * rad.cos(), center[1] + 0.95 * size * rad.sin()];
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
