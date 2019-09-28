use metal::*;
use crate::rendering::{PlatformRendererImpl};
use core_graphics::geometry::CGSize;
use crate::window::{GameWindowImpl};
use crate::platform::osx::osx_window::OsxWindow;
use std::mem;
use cocoa::base::id;

pub struct MetalRenderer {
    device: Device,
    layer: CoreAnimationLayer,
    command_queue: CommandQueue,
    width: usize,
    height: usize,
    library: Library,
    position_buffer: Buffer,
    color_buffer: Buffer,
    pipeline_state: RenderPipelineState,
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

        let source = "
using namespace metal;

struct ColoredVertex
{
    float4 position [[position]];
    float4 color;
};

vertex ColoredVertex vertex_main(constant float4 *position [[buffer(0)]],
                                 constant float4 *color [[buffer(1)]],
                                 uint vid [[vertex_id]])
{
    ColoredVertex vert;
    vert.position = position[vid];
    vert.color = color[vid];
    return vert;
}

fragment float4 fragment_main(ColoredVertex vert [[stage_in]])
{
    return vert.color;
}
";
        let library = device
            .new_library_with_source(source, &CompileOptions::new())
            .unwrap();

        let vert = library.get_function("vertex_main", None).unwrap();
        let frag = library.get_function("fragment_main", None).unwrap();

        let pipeline_state_descriptor = RenderPipelineDescriptor::new();
        pipeline_state_descriptor.set_vertex_function(Some(&vert));
        pipeline_state_descriptor.set_fragment_function(Some(&frag));
        pipeline_state_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap()
            .set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        let pipeline_state = device
            .new_render_pipeline_state(&pipeline_state_descriptor)
            .unwrap();

        let position_buffer = {
            let position_data = [
                0.0f32, 0.5, 0., 1., -0.5, -0.5, 0., 1., 0.5, -0.5, 0., 1.,
            ];

            device.new_buffer_with_data(
                unsafe { mem::transmute(position_data.as_ptr()) },
                (position_data.len() * mem::size_of::<f32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache,
            )
        };
        let color_buffer = {
            let color_data = [
                1.0f32, 0., 0., 1., 0., 1., 0., 1., 0., 0., 1., 1.,
            ];

            device.new_buffer_with_data(
                unsafe { mem::transmute(color_data.as_ptr()) },
                (color_data.len() * mem::size_of::<f32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache,
            )
        };

        MetalRenderer {
            device,
            layer,
            command_queue,
            width,
            height,
            library,
            position_buffer,
            color_buffer,
            pipeline_state,
        }
    }

    pub fn render(&self) {
        if let Some(drawable) = self.layer.next_drawable() {
            let render_pass_descriptor = RenderPassDescriptor::new();
            prepare_render_pass_descriptor(&render_pass_descriptor, drawable.texture());

            let command_buffer = self.command_queue.new_command_buffer();

            let encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);
            encoder.set_render_pipeline_state(&self.pipeline_state);
            encoder.set_vertex_buffer(0, Some(&self.position_buffer), 0);
            encoder.set_vertex_buffer(1, Some(&self.color_buffer), 0);
            encoder.draw_primitives(
                MTLPrimitiveType::TriangleStrip,
                0,
                3,
            );
            encoder.end_encoding();

            command_buffer.present_drawable(&drawable);
            command_buffer.commit();
        }
    }

    pub fn get_layer(&self) -> &CoreAnimationLayerRef {
        return self.layer.as_ref();
    }
}


