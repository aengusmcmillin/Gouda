use std::{collections::HashMap, f32::consts::PI};

use crate::platform::metal::Renderable;

use super::{buffers::{VertexBuffer, IndexBuffer}, Renderer, Scene};


pub type Vertex2d = [f32; 2];

pub struct Shape2d {
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    num_indices: u64,
}

impl Renderable for Shape2d {
    fn bind(&self, scene: &Scene) {
        self.vertex_buffer.bind(scene);
    }

    fn num_indices(&self) -> u64 {
        self.num_indices
    }

    fn index_buffer(&self) -> &IndexBuffer {
        &self.index_buffer
    }
}


impl Shape2d {
    pub fn square(renderer: &Renderer) -> Shape2d {
        let vb = VertexBuffer::new(
            renderer,
            0,
            vec![
                [-1., -1.], // bottom left
                [1., -1.], // bottom right
                [1., 1.], // top right
                [-1., 1.], // top left
            ]);

        let ib = IndexBuffer::new(
            renderer,
            vec![
                0, 3, 2,
                0, 1, 2,
            ]);

        return Shape2d { vertex_buffer: vb, index_buffer: ib, num_indices: 6 }
    }

    pub fn texture_quad(renderer: &Renderer) -> Shape2d {
        let vb = VertexBuffer::new(
            renderer,
            0,
            vec![
                [-1., -1., 0., 1., 0., 1.], // bottom left
                [1., -1., 0., 1., 1., 1.], // bottom right
                [1., 1., 0., 1., 1., 0.], // top right
                [-1., 1., 0., 1., 0., 0.], // top left
            ]);

        let ib = IndexBuffer::new(
            renderer,
            vec![
                0, 3, 2,
                0, 1, 2,
            ]);

        return Shape2d { vertex_buffer: vb, index_buffer: ib, num_indices: 6 }
    }

    pub fn hex(renderer: &Renderer) -> Shape2d {

        fn flat_hex_corner(i: f32) -> [f32; 2] {
            let deg = 60. * i;
            let rad = PI / 180. * deg;
        
            return [rad.cos(), rad.sin()];
        }
        
        let verts = vec![
            [0., 0.],
            flat_hex_corner(0.),
            flat_hex_corner(1.),
            flat_hex_corner(2.),
            flat_hex_corner(3.),
            flat_hex_corner(4.),
            flat_hex_corner(5.),
        ];
        let indices = vec![
            0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1
        ];
        let num_indices = indices.len();
        let vb = VertexBuffer::new(
            renderer,
            0,
            verts);

        let ib = IndexBuffer::new(
            renderer,
            indices);
        return Shape2d { vertex_buffer: vb, index_buffer: ib, num_indices: num_indices as u64 }
    }
}

pub struct ShapeLibrary {
    shapes2d: HashMap<String, Shape2d>
}


impl ShapeLibrary {
    pub fn new() -> ShapeLibrary {
        return ShapeLibrary { shapes2d: HashMap::new() };
    }

    pub fn construct(renderer: &Renderer) -> ShapeLibrary {
        let mut lib = ShapeLibrary::new();
        lib.add_2d_shape("square".to_string(), Shape2d::square(renderer));
        lib.add_2d_shape("quad".to_string(), Shape2d::square(renderer));
        lib.add_2d_shape("hex".to_string(), Shape2d::hex(renderer));
        lib.add_2d_shape("texture".to_string(), Shape2d::texture_quad(renderer));
        return lib;
    }

    pub fn add_2d_shape(&mut self, name: String, shape2d: Shape2d) {
        self.shapes2d.insert(name, shape2d);
    }

    pub fn get(&self, name: String) -> Option<&Shape2d> {
        return self.shapes2d.get(&name);
    }

    // pub fn bind_shape(&self, scene: &Scene, name: String) {
    //     self.get(name).unwrap().bind(scene);
    // }
}