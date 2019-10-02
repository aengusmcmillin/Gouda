use crate::platform::metal::{Renderer, Scene};
use metal::*;
use std::env;
use std::fs;

#[derive(Debug)]
pub struct Shader {
    pipeline_state: RenderPipelineState,
}

impl Shader {
    pub fn new(gfx: &Renderer, vertex_file: &str, fragment_file: &str) -> Shader {
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
        pipeline_state_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap()
            .set_pixel_format(MTLPixelFormat::BGRA8Unorm);
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
