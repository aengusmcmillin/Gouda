use crate::platform::metal::{Renderer, Scene};
use metal::*;

use super::buffers::BufferLayout;

#[derive(Debug)]
pub struct Shader {
    pipeline_state: RenderPipelineState,
}

impl Shader {
    pub fn new(gfx: &Renderer, buffer_layout: BufferLayout, vertex_src: &str, fragment_src: &str) -> Shader {
        let vert = gfx.device
            .new_library_with_source(&vertex_src, &CompileOptions::new())
            .expect("Failed to compile vertex shader")
            .get_function("vertex_main", None)
            .unwrap();
        let frag = gfx.device
            .new_library_with_source(&fragment_src, &CompileOptions::new())
            .expect("Failed to compile fragment shader")
            .get_function("fragment_main", None)
            .unwrap();

        let pipeline_state_descriptor = RenderPipelineDescriptor::new();
        pipeline_state_descriptor.set_vertex_function(Some(&vert));
        pipeline_state_descriptor.set_fragment_function(Some(&frag));

        let vertex_descriptor = VertexDescriptor::new();

        for (index, attribute) in buffer_layout.elements.iter().enumerate() {
            let vertex_attribute_descriptor = &VertexAttributeDescriptor::new();
            vertex_attribute_descriptor.set_format(attribute.data_type.to_metal());
            vertex_attribute_descriptor.set_buffer_index(0);
            vertex_attribute_descriptor.set_offset(attribute.offset as u64);
            vertex_descriptor.attributes().set_object_at(index, Some(vertex_attribute_descriptor));
        }

        vertex_descriptor.layouts().object_at(0).unwrap().set_stride(buffer_layout.stride as u64);

        pipeline_state_descriptor.set_vertex_descriptor(Some(&vertex_descriptor));

        let render_buffer_attachment = pipeline_state_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();
        render_buffer_attachment.set_pixel_format(MTLPixelFormat::BGRA8Unorm);

        render_buffer_attachment.set_blending_enabled(true);
        render_buffer_attachment.set_rgb_blend_operation(MTLBlendOperation::Add);
        render_buffer_attachment.set_alpha_blend_operation(MTLBlendOperation::Add);
        render_buffer_attachment.set_source_rgb_blend_factor(MTLBlendFactor::SourceAlpha);
        render_buffer_attachment.set_source_alpha_blend_factor(MTLBlendFactor::SourceAlpha);
        render_buffer_attachment.set_destination_rgb_blend_factor(MTLBlendFactor::OneMinusSourceAlpha);
        render_buffer_attachment.set_destination_alpha_blend_factor(MTLBlendFactor::OneMinusSourceAlpha);

        let pipeline_state = gfx.device
            .new_render_pipeline_state(&pipeline_state_descriptor)
            .unwrap();

        return Shader {
            pipeline_state,
        }
    }

    pub fn bind(&self, scene: &Scene) {
        scene.encoder.set_render_pipeline_state(&self.pipeline_state);
    }
}
