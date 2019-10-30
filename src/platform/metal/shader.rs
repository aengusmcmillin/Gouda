use crate::platform::metal::{Renderer, Scene};
use metal::*;
use std::env;
use std::fs;

#[derive(Debug)]
pub struct Shader {
    pipeline_state: RenderPipelineState,
}

impl Shader {
    pub fn new(gfx: &Renderer, has_textures: bool, vertex_file: &str, fragment_file: &str) -> Shader {
        let vertex_src = fs::read_to_string(vertex_file).expect("Bad vertex shader filename");
        let fragment_src = fs::read_to_string(fragment_file).expect("Bad fragment shader filename");

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

        let mut vdesc = VertexDescriptor::new();
        let mut attr1 = &VertexAttributeDescriptor::new();
        attr1.set_format(MTLVertexFormat::Float4);
        attr1.set_offset(0);
        attr1.set_buffer_index(0);
        vdesc.attributes().set_object_at(0, Some(attr1));
        if has_textures {
            let mut attr2 = &VertexAttributeDescriptor::new();
            attr2.set_format(MTLVertexFormat::Float2);
            attr2.set_offset(16);
            attr2.set_buffer_index(0);
            vdesc.attributes().set_object_at(1, Some(attr2));
            vdesc.layouts().object_at(0).unwrap().set_stride(24);
        } else {
            vdesc.layouts().object_at(0).unwrap().set_stride(16);
        }
        pipeline_state_descriptor.set_vertex_descriptor(Some(&vdesc));


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
