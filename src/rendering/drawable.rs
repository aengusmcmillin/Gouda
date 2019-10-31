use crate::rendering::shader::Shader;
use crate::rendering::buffers::{VertexBuffer, IndexBuffer, VertexConstantBuffer, FragmentConstantBuffer};
use crate::rendering::{Scene, Renderer};
use crate::input::{GameInput, LetterKeys};
use crate::math::{create_transformation_matrix, create_projection_matrix, create_view_matrix, Mat4x4};
use crate::rendering::texture::RenderableTexture;

pub trait Drawable {
    fn update(&mut self, renderer: &Renderer, input: &GameInput);
    fn bind(&self, scene: &Scene);
    fn draw(&self, scene: &Scene);
}

#[derive(Debug)]
pub struct TextureDrawable {
    pub vertex_buffer: VertexBuffer<[f32; 6]>,
    pub index_buffer: IndexBuffer,
    pub transform_buffer: VertexConstantBuffer<f32>,
    pub shader: Shader,
    pub identity_buffer: VertexConstantBuffer<f32>,
    pub texture: RenderableTexture,
}

impl TextureDrawable {
    pub fn new(is_gui: bool, renderer: &Renderer, texture: RenderableTexture, position: [f32; 3], scale: [f32; 3]) -> Self {
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

        let shader = Shader::new(
            renderer,
            true,
            "shaders/textureVertexShader.txt",
            "shaders/textureFragmentShader.txt");

        let transform_mat = create_transformation_matrix(position, [0., 0., 0.], scale);
        let transform_buffer = VertexConstantBuffer::new(renderer, 0, transform_mat.raw_data().to_vec());
        let identity_buffer = VertexConstantBuffer::new(renderer, 1, Mat4x4::identity().to_vec());

        return Self {
            vertex_buffer: vb,
            index_buffer: ib,
            transform_buffer,
            shader,
            identity_buffer,
            texture
        }
    }

    pub fn translate(&self, renderer: &Renderer, position: [f32; 3], scale: [f32; 3]) {
        let transform_mat = create_transformation_matrix(position, [0., 0., 0.], scale);
        self.transform_buffer.update_data(renderer, transform_mat.to_vec());
    }

    pub fn draw_with_projection(&self, scene: &Scene, camera_projection: &VertexConstantBuffer<f32>) {
        camera_projection.bind_to_offset(scene, 1);
        self.draw_impl(scene);
    }

    pub fn draw(&self, scene: &Scene) {
        self.identity_buffer.bind(scene);
        self.draw_impl(scene);
    }

    fn draw_impl(&self, scene: &Scene) {
        self.shader.bind(scene);
        self.vertex_buffer.bind(scene);
        self.transform_buffer.bind(scene);
        self.texture.bind(scene);
        self.index_buffer.bind(scene);

        scene.draw_indexed(6, &self.index_buffer);
    }
}

pub struct TestDrawable {
    pub vertex_buffer: VertexBuffer<[f32; 4]>,
    pub transform_buffer: VertexConstantBuffer<f32>,
    pub transform_buffer2: VertexConstantBuffer<f32>,
    pub color_buffer: FragmentConstantBuffer<f32>,
    pub index_buffer: IndexBuffer,
    pub shader: Shader,
}

impl TestDrawable {
    pub fn new(renderer: &Renderer) -> Self {
        let vb = VertexBuffer::new(
            renderer,
            0,
            vec![
                [-0.5, -0.5, 0., 1.],
                [0.5, -0.5, 0., 1.],
                [0.5, 0.5, 0., 1.],
                [-0.5, 0.5, 0., 1.],
            ]);

        let shader = Shader::new(
            renderer,
            false,
            "shaders/testVertexShader.txt",
            "shaders/testFragmentShader.txt");

        let ib = IndexBuffer::new(
            renderer,
            vec![
                0, 1, 2, 2, 3, 0
            ]);

        let transform_mat = create_transformation_matrix([0.5, 0., 0.], [0., 0., 0.], [1., 1., 1.]);
        let transform_buffer = VertexConstantBuffer::new(renderer,0, transform_mat.raw_data().to_vec());

        let transform_mat2 = create_transformation_matrix([0.0, 0.5, 0.], [0., 0., 0.], [1., 1., 1.]);
        let transform_buffer2 = VertexConstantBuffer::new(renderer,1, transform_mat2.raw_data().to_vec());

        let data = [1.0, 0.0, 1.0, 1.0];
        let color_buffer = FragmentConstantBuffer::new(renderer, 0, data.to_vec());

        return Self {
            vertex_buffer: vb,
            transform_buffer,
            transform_buffer2,
            color_buffer,
            index_buffer: ib,
            shader,
        }
    }

    pub fn draw(&self, scene: &Scene) {
        self.shader.bind(scene);
        self.vertex_buffer.bind(scene);
        self.transform_buffer.bind(scene);
        self.transform_buffer2.bind(scene);
        self.index_buffer.bind(scene);
        self.color_buffer.bind(scene);

        scene.draw_indexed(6, &self.index_buffer);
    }
}


#[derive(Debug)]
pub struct QuadDrawable {
    pub vertex_buffer: VertexBuffer<[f32; 4]>,
    pub index_buffer: IndexBuffer,
    pub transform_buffer: VertexConstantBuffer<f32>,
    pub shader: Shader,
    pub color_buffer: FragmentConstantBuffer<f32>,
    pub identity_buffer: VertexConstantBuffer<f32>,
}

impl QuadDrawable {
    pub fn new(is_gui: bool, renderer: &Renderer, color: [f32; 3], position: [f32; 3], scale: [f32; 3]) -> Self {
        let vb = VertexBuffer::new(
            renderer,
            0,
            vec![
                [-1., -1., 0., 1.], // bottom left
                [1., -1., 0., 1.], // bottom right
                [1., 1., 0., 1.], // top right
                [-1., 1., 0., 1.], // top left
            ]);

        let ib = IndexBuffer::new(
            renderer,
            vec![
                0, 3, 2,
                0, 1, 2,
            ]);

        let shader = if is_gui {
            Shader::new(
                renderer,
                false,
                "shaders/guiVertexShader.txt",
                "shaders/guiFragmentShader.txt")
        } else {
            Shader::new(
                renderer,
                false,
                "shaders/quadVertexShader.txt",
                "shaders/quadFragmentShader.txt")
        };

        let transform_mat = create_transformation_matrix(position, [0., 0., 0.], scale);
        let transform_buffer = VertexConstantBuffer::new(renderer,0, transform_mat.raw_data().to_vec());

        let data = [color[0], color[1], color[2], 1.0];
        let color_buffer = FragmentConstantBuffer::new(renderer, 0, data.to_vec());

        let identity_buffer = VertexConstantBuffer::new(renderer, 1, Mat4x4::identity().to_vec());
        return Self {
            vertex_buffer: vb,
            index_buffer: ib,
            transform_buffer,
            color_buffer,
            shader,
            identity_buffer,
        }
    }

    pub fn translate(&self, renderer: &Renderer, position: [f32; 3], scale: [f32; 3]) {
        let transform_mat = create_transformation_matrix(position, [0., 0., 0.], scale);
        self.transform_buffer.update_data(renderer, transform_mat.to_vec());
    }

    pub fn draw_with_projection(&self, scene: &Scene, camera_projection: &VertexConstantBuffer<f32>) {
        camera_projection.bind_to_offset(scene, 1);
        self.draw_impl(scene);
    }

    pub fn draw(&self, scene: &Scene) {
        self.identity_buffer.bind(scene);
        self.draw_impl(scene);
    }

    fn draw_impl(&self, scene: &Scene) {
        self.shader.bind(scene);
        self.vertex_buffer.bind(scene);
        self.transform_buffer.bind(scene);
        self.index_buffer.bind(scene);
        self.color_buffer.bind(&scene);

        scene.draw_indexed(6, &self.index_buffer);
    }
}


pub struct TriangleDrawable {
    shader: Shader,
    vertex_buffers: Vec<VertexBuffer<f32>>,
}

impl TriangleDrawable {
    fn update(&mut self) { }

    pub fn new(renderer: &Renderer) -> TriangleDrawable {
        let position_data = vec![0.0f32, 0.5, 0., 1., -0.5, -0.5, 0., 1., 0.5, -0.5, 0., 1.];
        let color_data = vec![1.0f32, 0., 0., 1., 0., 1., 0., 1., 0., 0., 1., 1.];
        let shader = Shader::new(&renderer, false, "shaders/vertexShader.txt", "shaders/fragmentShader.txt");
        let vbuf = VertexBuffer::new(renderer, 0, position_data);
        let cbuf = VertexBuffer::new(renderer, 1, color_data);

        return TriangleDrawable {
            shader,
            vertex_buffers: vec![vbuf, cbuf],
        };
    }
}

impl Drawable for TriangleDrawable {
    fn update(&mut self, renderer: &Renderer, input: &GameInput) {}

    fn bind(&self, scene: &Scene) {
        self.shader.bind(&scene);
        for buf in self.vertex_buffers.iter() {
            buf.bind(&scene);
        }
    }

    fn draw(&self, scene: &Scene) {
        scene.draw_tri_strip(3);
    }
}
