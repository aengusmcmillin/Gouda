use std::collections::HashMap;
use std::f32::consts::PI;

use crate::shader_lib::basic_shader::basic_shader_layout;
use crate::shader_lib::quad_shader::quad_shader_layout;
use crate::shader_lib::texture_shader::texture_shader_layout;
use crate::{Renderable, Scene};

use super::buffers::{IndexBuffer, VertexBuffer};
use super::Renderer;

pub type Vertex2d = [f32; 2];

pub struct Shape2d {
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    num_indices: u64,
}

impl Renderable for Shape2d {
    fn bind(&self, scene: &Scene) {
        self.vertex_buffer.bind(scene);
        self.index_buffer.bind(scene);
    }

    fn num_indices(&self) -> u64 {
        self.num_indices
    }

    fn index_buffer(&self) -> &IndexBuffer {
        &self.index_buffer
    }
}

impl Shape2d {
    pub fn gui(renderer: &Renderer) -> Shape2d {
        let vb = VertexBuffer::new::<[f32; 2]>(
            renderer,
            quad_shader_layout(),
            0,
            vec![
                [0., 0.], // bottom left
                [2., 0.], // bottom right
                [2., 2.], // top right
                [0., 2.], // top left
            ],
        );

        let ib = IndexBuffer::new(renderer, vec![0, 3, 2, 0, 1, 2]);

        Shape2d {
            vertex_buffer: vb,
            index_buffer: ib,
            num_indices: 6,
        }
    }
    pub fn square(renderer: &Renderer) -> Shape2d {
        let vb = VertexBuffer::new::<[f32; 3]>(
            renderer,
            quad_shader_layout(),
            0,
            vec![
                [-0.5, -0.5, 0.], // bottom left
                [0.5, -0.5, 0.],  // bottom right
                [0.5, 0.5, 0.],   // top right
                [-0.5, 0.5, 0.],  // top left
            ],
        );

        let ib = IndexBuffer::new(renderer, vec![0, 3, 2, 0, 1, 2]);

        Shape2d {
            vertex_buffer: vb,
            index_buffer: ib,
            num_indices: 6,
        }
    }

    pub fn triangle(renderer: &Renderer) -> Shape2d {
        let vb = VertexBuffer::new::<[f32; 3]>(
            renderer,
            quad_shader_layout(),
            0,
            vec![
                [-0.5, -0.5, 0.],
                [0.5, -0.5, 0.],
                [0.0, 0.5, 0.],
            ]
        );

        let ib = IndexBuffer::new(renderer, vec![0, 2, 1]);

        Shape2d {
            vertex_buffer: vb,
            index_buffer: ib,
            num_indices: 3,
        }
    }

    pub fn cube(renderer: &Renderer) -> Shape2d {
        let vb = VertexBuffer::new::<[f32; 3]>(
            renderer,
            quad_shader_layout(),
            0,
            vec![
                [-0.5, -0.5, -0.5],
                [0.5, -0.5, -0.5],
                [0.5,  0.5, -0.5], 
                [0.5,  0.5, -0.5], 
                [-0.5,  0.5, -0.5], 
                [-0.5, -0.5, -0.5], 

                [-0.5, -0.5,  0.5],  
                [0.5, -0.5,  0.5],  
                [0.5,  0.5,  0.5],  
                [0.5,  0.5,  0.5],  
                [-0.5,  0.5,  0.5],  
                [-0.5, -0.5,  0.5],  

                [-0.5,  0.5,  0.5],  
                [-0.5,  0.5, -0.5],  
                [-0.5, -0.5, -0.5],  
                [-0.5, -0.5, -0.5],  
                [-0.5, -0.5,  0.5],  
                [-0.5,  0.5,  0.5],  

                [0.5,  0.5,  0.5],  
                [0.5,  0.5, -0.5],  
                [0.5, -0.5, -0.5],  
                [0.5, -0.5, -0.5],  
                [0.5, -0.5,  0.5],  
                [0.5,  0.5,  0.5],  

                [-0.5, -0.5, -0.5],  
                [0.5, -0.5, -0.5],  
                [0.5, -0.5,  0.5],  
                [0.5, -0.5,  0.5],  
                [-0.5, -0.5,  0.5],  
                [-0.5, -0.5, -0.5],  

                [-0.5,  0.5, -0.5],  
                [0.5,  0.5, -0.5],  
                [0.5,  0.5,  0.5],  
                [0.5,  0.5,  0.5],  
                [-0.5,  0.5,  0.5],  
                [-0.5,  0.5, -0.5],  
            ]
        );

        let ib = IndexBuffer::new(renderer, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35]);

        Shape2d {
            vertex_buffer: vb,
            index_buffer: ib,
            num_indices: 36,
        }
    }

    pub fn texture_quad(renderer: &Renderer) -> Shape2d {
        let vb = VertexBuffer::new::<[f32; 6]>(
            renderer,
            texture_shader_layout(),
            0,
            vec![
                [-0.5, -0.5, 0., 1., 0., 1.], // bottom left
                [0.5, -0.5, 0., 1., 1., 1.],  // bottom right
                [0.5, 0.5, 0., 1., 1., 0.],   // top right
                [-0.5, 0.5, 0., 1., 0., 0.],  // top left
            ],
        );

        let ib = IndexBuffer::new(renderer, vec![0, 3, 2, 0, 1, 2]);

        Shape2d {
            vertex_buffer: vb,
            index_buffer: ib,
            num_indices: 6,
        }
    }

    pub fn hex(renderer: &Renderer) -> Shape2d {
        fn flat_hex_corner(i: f32) -> [f32; 2] {
            let deg = 60. * i;
            let rad = PI / 180. * deg;

            [rad.cos(), rad.sin()]
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
        let indices = vec![0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1];
        let num_indices = indices.len();
        let vb = VertexBuffer::new(renderer, basic_shader_layout(), 0, verts);

        let ib = IndexBuffer::new(renderer, indices);
        Shape2d {
            vertex_buffer: vb,
            index_buffer: ib,
            num_indices: num_indices as u64,
        }
    }
}

pub struct ShapeLibrary {
    shapes2d: HashMap<&'static str, Shape2d>,
}

impl Default for ShapeLibrary {
    fn default() -> Self {
        ShapeLibrary::new()
    }
}

impl ShapeLibrary {
    pub fn new() -> ShapeLibrary {
        ShapeLibrary {
            shapes2d: HashMap::new(),
        }
    }

    pub fn initialize(&mut self, renderer: &Renderer) {
        self.add_2d_shape("square", Shape2d::square(renderer));
        self.add_2d_shape("triangle", Shape2d::triangle(renderer));
        self.add_2d_shape("cube", Shape2d::cube(renderer));
        self.add_2d_shape("gui", Shape2d::gui(renderer));
        self.add_2d_shape("quad", Shape2d::square(renderer));
        self.add_2d_shape("hex", Shape2d::hex(renderer));
        self.add_2d_shape("texture", Shape2d::texture_quad(renderer));
    }

    pub fn add_2d_shape(&mut self, name: &'static str, shape2d: Shape2d) {
        self.shapes2d.insert(name, shape2d);
    }

    pub fn get(&self, name: &'static str) -> Option<&Shape2d> {
        return self.shapes2d.get(&name);
    }

    pub fn bind_shape(&self, scene: &Scene, name: &'static str) {
        self.get(name).unwrap().bind(scene);
    }
}
