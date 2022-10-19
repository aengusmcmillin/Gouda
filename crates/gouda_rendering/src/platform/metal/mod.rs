#![cfg(target_os = "macos")]

use cocoa::appkit::NSView;
use core_graphics::geometry::CGSize;
use gouda_window::osx::PlatformWindow;
use metal::*;
use std::mem::size_of;
use std::{f32, mem};

use self::buffers::PlatformIndexBuffer;

pub mod buffers;
pub mod drawable;
pub mod shader;
pub mod texture;

pub struct PlatformScene<'a> {
    encoder: &'a RenderCommandEncoderRef,
    command_buffer: &'a CommandBufferRef,
    drawable: &'a DrawableRef,
}

impl PlatformScene<'_> {
    pub fn end(self) {
        self.encoder.end_encoding();

        self.command_buffer.present_drawable(&self.drawable);
        self.command_buffer.commit();
    }

    pub fn draw_indexed(&self, num_indices: u64, index_buffer: &PlatformIndexBuffer) {
        self.encoder.draw_indexed_primitives(
            MTLPrimitiveType::TriangleStrip,
            num_indices,
            MTLIndexType::UInt16,
            &index_buffer.data,
            0,
        );
    }

    pub fn draw_indexed_tris(&self, num_indices: u64, index_buffer: &PlatformIndexBuffer) {
        self.encoder.draw_indexed_primitives(
            MTLPrimitiveType::Triangle,
            num_indices,
            MTLIndexType::UInt16,
            &index_buffer.data,
            0,
        );
    }

    pub fn draw_tri_strip(&self, num_verts: u64) {
        self.encoder
            .draw_primitives(MTLPrimitiveType::TriangleStrip, 0, num_verts);
    }

    pub fn draw_triangles(&self, num_verts: u64) {
        self.encoder
            .draw_primitives(MTLPrimitiveType::Triangle, 0, num_verts);
    }
}

fn prepare_render_pass_descriptor(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();

    color_attachment.set_texture(Some(texture));
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_clear_color(MTLClearColor::new(0.43, 0.73, 0.36, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}

pub struct PlatformRenderer {
    device: Device,
    layer: CoreAnimationLayer,
    command_queue: CommandQueue,
    width: usize,
    height: usize,
}

impl PlatformRenderer {
    pub fn attach_renderer(&self, window: &PlatformWindow) {
        unsafe {
            window
                .cocoa_window
                .view
                .unwrap()
                .setLayer(mem::transmute(self.get_layer()));
        }
    }

    pub fn new(window: &mut PlatformWindow) -> Result<Self, String> {
        let device = Device::system_default().unwrap();
        let layer = CoreAnimationLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);

        let width = window.get_width();
        let height = window.get_height();
        layer.set_drawable_size(CGSize::new(width as f64, height as f64));

        let command_queue = device.new_command_queue();

        let mut res = PlatformRenderer {
            device,
            layer,
            command_queue,
            width,
            height,
        };
        return Ok(res);
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width as usize;
        self.height = height as usize;
        println!("Resizing {} {}", width, height);
        self.layer
            .set_drawable_size(CGSize::new(width as f64, height as f64));
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn begin_scene(&self) -> Option<PlatformScene> {
        if let Some(drawable) = self.layer.next_drawable() {
            let render_pass_descriptor = RenderPassDescriptor::new();
            prepare_render_pass_descriptor(&render_pass_descriptor, drawable.texture());

            let command_buffer = self.command_queue.new_command_buffer();

            let encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);
            let scene = PlatformScene {
                encoder,
                command_buffer,
                drawable,
            };
            return Some(scene);
        } else {
            return None;
        }
    }

    pub fn end_scene(&self, scene: PlatformScene) {
        scene.end();
    }

    pub fn get_layer(&self) -> &CoreAnimationLayerRef {
        return self.layer.as_ref();
    }
}

pub trait Sizeable<T> {
    fn size() -> usize {
        return size_of::<T>();
    }
    fn stride() -> usize {
        return size_of::<T>();
    }
}

pub type Float2 = [f32; 2];
impl Sizeable<Float2> for Float2 {}
// pub type Float3 = [f32; 3];
// pub type Float4 = [f32; 4];
