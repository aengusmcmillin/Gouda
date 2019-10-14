#![cfg(target_os = "macos")]

use metal::*;
use core_graphics::geometry::CGSize;
use crate::window::{GameWindowImpl};
use crate::platform::osx::osx_window::OsxWindow;
use std::mem;
use cocoa::base::id;
use std::any::Any;
use std::collections::HashMap;
use metal::MTLPrimitiveType::Triangle;
use std::convert::TryInto;
use std::f32;
use crate::math::{Mat4x4, create_transformation_matrix, create_projection_matrix, create_view_matrix};
use std::os::raw::c_void;
use crate::input::{GameInput, LetterKeys};
use crate::platform::metal::shader::Shader;
use crate::platform::metal::buffers::{VertexBuffer, IndexBuffer};
use metal::MTLFunctionType::Vertex;

pub mod drawable;
pub mod shader;
pub mod buffers;
pub mod texture;

pub struct Scene<'a> {
    encoder: &'a RenderCommandEncoderRef,
    command_buffer: &'a CommandBufferRef,
    drawable: &'a DrawableRef,
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
}

fn prepare_render_pass_descriptor(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();

    color_attachment.set_texture(Some(texture));
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_clear_color(MTLClearColor::new(0.2, 0.2, 0.2, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}

pub struct Renderer {
    device: Device,
    layer: CoreAnimationLayer,
    command_queue: CommandQueue,
    width: usize,
    height: usize,
    drawables: Vec<Box<Drawable>>,
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
            drawables: vec![],
        };
        return res;
    }

    pub fn begin_scene(&self) -> Option<Scene> {
        if let Some(drawable) = self.layer.next_drawable() {
            let render_pass_descriptor = RenderPassDescriptor::new();
            prepare_render_pass_descriptor(&render_pass_descriptor, drawable.texture());

            let command_buffer = self.command_queue.new_command_buffer();

            let encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);
            let scene = Scene {encoder, command_buffer, drawable};
            return Some(scene);
        } else {
            return None;
        }
    }

    pub fn end_scene(&self, scene: Scene) {
        scene.encoder.end_encoding();

        scene.command_buffer.present_drawable(&scene.drawable);
        scene.command_buffer.commit();
    }

    pub fn get_layer(&self) -> &CoreAnimationLayerRef {
        return self.layer.as_ref();
    }
}


