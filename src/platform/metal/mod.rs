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
use crate::math::{Mat4x4, create_transformation_matrix};
use std::os::raw::c_void;

pub mod drawable;

fn prepare_render_pass_descriptor(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();

    color_attachment.set_texture(Some(texture));
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_clear_color(MTLClearColor::new(0.5, 0.3, 0.2, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}

pub struct Scene<'a> {
    encoder: &'a RenderCommandEncoderRef,
    command_buffer: &'a CommandBufferRef,
    drawable: &'a DrawableRef,
}

struct Shader {
    pipeline_state: RenderPipelineState,
}

impl Shader {
    pub fn bind(&self, scene: &Scene) {
        scene.encoder.set_render_pipeline_state(&self.pipeline_state);
    }
}

struct VertexBuffer {
    data: Buffer,
    offset: u64,
}

impl VertexBuffer {
    pub fn bind(&self, scene: &Scene) {
        scene.encoder.set_vertex_buffer(self.offset, Some(&self.data), 0);
    }

    pub fn update_data(&self, mut data: Vec<f32>) {
        unsafe {
            std::ptr::copy(data.as_mut_ptr(), mem::transmute(self.data.contents()), data.len());
        };
    }
}

pub trait Drawable {
    fn update(&mut self);
    fn bind(&self, scene: &Scene);
    fn draw(&self, scene: &Scene);
}

pub struct SquareDrawable {
    shader: Shader,
    vertex_buffer: VertexBuffer,
    color_buffer: VertexBuffer,
    transform_buffer: VertexBuffer,
    test: f32,
    scale: f32,
    translate: [f32; 3],
}

impl SquareDrawable {
    pub fn new(renderer: &mut Renderer, color: [f32; 3], scale: f32, translate: [f32; 3]) -> SquareDrawable {
        let source = "
using namespace metal;

struct VertexUniforms {
    float4x4 transformation;
};

struct ColoredVertex
{
    float4 position [[position]];
    float4 color;
};

vertex ColoredVertex vertex_main(constant float4 *position [[buffer(0)]],
                                 constant float4 &color [[buffer(1)]],
                                 constant VertexUniforms &uniforms [[buffer(2)]],
                                 uint vid [[vertex_id]])
{
    ColoredVertex vert;
    vert.position = position[vid] * uniforms.transformation;
    vert.color = color;
    return vert;
}

fragment float4 fragment_main(ColoredVertex vert [[stage_in]])
{
    return vert.color;
}
";
        let position_data = vec![
            -0.5f32, -0.5, 0., 1.,
            0.5, -0.5, 0., 1.,
            -0.5, 0.5, 0., 1.,
            0.5, 0.5, 0., 1.
        ];
        let shader = renderer.create_shader(source, "vertex_main", "fragment_main");
        let vertex_buffer = renderer.create_vertex_buffer(0, position_data);
        let color_data = vec![
            color[0], color[1], color[2], 1.,
        ];
        let color_buffer = renderer.create_vertex_buffer(1, color_data);

        let transform_mat = create_transformation_matrix([0.9, 0., 0.], 0., 0., 0., 1.2);

        let transform_buffer = renderer.create_vertex_buffer(2, transform_mat.raw_data().to_vec());

        return SquareDrawable {
            shader,
            vertex_buffer,
            color_buffer,
            transform_buffer,
            test: 0.,
            scale,
            translate,
        };
    }
}

impl Drawable for SquareDrawable {
    fn update(&mut self) {
        self.test += 1.;
        let transform_mat = create_transformation_matrix(self.translate, 0., 0. + self.test, 0., self.scale);
        self.transform_buffer.update_data(transform_mat.to_vec());
    }

    fn bind(&self, scene: &Scene) {
        self.shader.bind(&scene);
        self.vertex_buffer.bind(scene);
        self.color_buffer.bind(scene);
        self.transform_buffer.bind(scene);
    }

    fn draw(&self, scene: &Scene) {

        scene.encoder.draw_primitives(
            MTLPrimitiveType::TriangleStrip,
            0,
            4,
        );
    }
}

pub struct TriangleDrawable {
    shader: Shader,
    vertex_buffers: Vec<VertexBuffer>,
}

impl TriangleDrawable {
    fn update(&mut self) {

    }

    pub fn new(renderer: &mut Renderer) -> TriangleDrawable {
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
        let position_data = vec![0.0f32, 0.5, 0., 1., -0.5, -0.5, 0., 1., 0.5, -0.5, 0., 1.];
        let color_data = vec![1.0f32, 0., 0., 1., 0., 1., 0., 1., 0., 0., 1., 1.];
        let shader = renderer.create_shader(source, "vertex_main", "fragment_main");
        let vbuf = renderer.create_vertex_buffer(0, position_data);
        let cbuf = renderer.create_vertex_buffer(1, color_data);

        return TriangleDrawable {
            shader,
            vertex_buffers: vec![vbuf, cbuf],
        };
    }
}

impl Drawable for TriangleDrawable {
    fn update(&mut self) {}

    fn bind(&self, scene: &Scene) {
        self.shader.bind(&scene);
        for buf in self.vertex_buffers.iter() {
            buf.bind(&scene);
        }
    }

    fn draw(&self, scene: &Scene) {
        scene.encoder.draw_primitives(
            MTLPrimitiveType::TriangleStrip,
            0,
            3,
        );
    }
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
    fn create_shader(&mut self, source: &str, vert_name: &str, frag_name: &str) -> Shader {
        let library = self.device
            .new_library_with_source(source, &CompileOptions::new())
            .unwrap();

        let vert = library.get_function(vert_name, None).unwrap();
        let frag = library.get_function(frag_name, None).unwrap();

        let pipeline_state_descriptor = RenderPipelineDescriptor::new();
        pipeline_state_descriptor.set_vertex_function(Some(&vert));
        pipeline_state_descriptor.set_fragment_function(Some(&frag));
        pipeline_state_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap()
            .set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        let pipeline_state = self.device
            .new_render_pipeline_state(&pipeline_state_descriptor)
            .unwrap();
        return Shader {
            pipeline_state,
        }
    }

    fn create_vertex_buffer(&mut self, offset: u64, position_data: Vec<f32>) -> VertexBuffer {
        let data = self.device.new_buffer_with_data(
            unsafe { mem::transmute(position_data.as_ptr()) },
            (position_data.len() * mem::size_of::<f32>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        );
        return VertexBuffer {
            offset,
            data,
        }
    }

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
            encoder.set_depth_clip_mode(MTLDepthClipMode::Clamp);
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

