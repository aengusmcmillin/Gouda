#![cfg(target_os = "windows")]

pub struct Renderer {

}

impl Renderer {
    pub fn begin_scene(&self) -> Option<Scene> {
        return None;
    }

    pub fn end_scene(&self, scene: Scene) {

    }
}

pub struct Scene {
}

impl Scene {
    pub fn draw_indexed(&self, num_indices: u64, index_buffer: &buffers::IndexBuffer) {

    }

    pub fn draw_tri_strip(&self, num_verts: u64) {

    }

    pub fn draw_triangles(&self, num_verts: u64) {

    }
}

pub mod buffers {
    pub use crate::platform::d3d::{Renderer, Scene};
    #[derive(Debug)]
    pub struct VertexBuffer {

    }

    impl VertexBuffer {
        pub fn new(renderer: &Renderer, offset: u32, data: Vec<f32>) -> VertexBuffer {
            VertexBuffer {}
        }

        pub fn update_data(&self, data: Vec<f32>) {

        }

        pub fn bind(&self, scene: &Scene) {

        }

        pub fn bind_to_offset(&self, scene: &Scene, offset: u32) {

        }
    }

    #[derive(Debug)]
    pub struct FragmentBuffer {

    }

    impl FragmentBuffer {
        pub fn new(renderer: &Renderer, offset: u32, data: Vec<f32>) -> FragmentBuffer {
            FragmentBuffer {}
        }

        pub fn update_data(&mut self, data: Vec<f32>) {

        }

        pub fn bind(&self, scene: &Scene) {

        }
    }

    #[derive(Debug)]
    pub struct IndexBuffer {

    }

    impl IndexBuffer {
        pub fn new(renderer: &Renderer, data: Vec<i32>) -> IndexBuffer {
            IndexBuffer {}
        }
    }
}

pub mod shader {
    pub use crate::platform::d3d::{Renderer, Scene};

    #[derive(Debug)]
    pub struct Shader {

    }

    impl Shader {
        pub fn new(renderer: &Renderer, vertex_file: &str, fragment_file: &str) -> Shader {
            Shader {}
        }

        pub fn bind(&self, scene: &Scene) {

        }
    }
}

pub mod texture {
    pub use crate::platform::d3d::{Renderer, Scene};
    pub use crate::png::PNG;
    pub use crate::bmp::Bitmap;

    #[derive(Debug)]
    pub struct RenderableTexture {

    }

    impl RenderableTexture {
        pub fn new(renderer: &Renderer, bmp: Bitmap) -> RenderableTexture {
            RenderableTexture {}
        }

        pub fn new_from_png(renderer: &Renderer, png: PNG) -> RenderableTexture {
            RenderableTexture {}
        }

        pub fn bind(&self, scene: &Scene) {

        }
    }
}
