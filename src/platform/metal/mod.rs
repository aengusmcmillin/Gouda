use metal::*;
use crate::rendering::{Renderer, PlatformRendererImpl};
use core_graphics::geometry::CGSize;
use crate::window::{GameWindow, GameWindowImpl};
use crate::platform::osx::osx_window::OsxWindow;

pub struct MetalRenderer {
    device: Device,
    layer: CoreAnimationLayer,
    command_queue: CommandQueue,
    region: MTLRegion,
    width: usize,
    height: usize,
}

fn prepare_render_pass_descriptor(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();

    color_attachment.set_texture(Some(texture));
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_clear_color(MTLClearColor::new(0.5, 0.3, 0.2, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}

impl PlatformRendererImpl for MetalRenderer {
    fn render(&self) {
        self.render();
    }
}

impl MetalRenderer {
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

        let region = MTLRegion {
            origin: MTLOrigin { x: 0, y: 0, z: 0 },
            size: MTLSize {
                width: width as u64,
                height: height as u64,
                depth: 1,
            },
        };

        MetalRenderer {
            device,
            layer,
            command_queue,
            region,
            width,
            height,
        }
    }

    pub fn render(&self) {
        if let Some(drawable) = self.layer.next_drawable() {
            let render_pass_descriptor = RenderPassDescriptor::new();
            prepare_render_pass_descriptor(&render_pass_descriptor, drawable.texture());

            let command_buffer = self.command_queue.new_command_buffer();

            let encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);
            encoder.end_encoding();

            command_buffer.present_drawable(&drawable);
            command_buffer.commit();
        }
    }

    pub fn get_layer(&self) -> &CoreAnimationLayerRef {
        return self.layer.as_ref();
    }
}


