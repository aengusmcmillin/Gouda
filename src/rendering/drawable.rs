use crate::rendering::shader::Shader;
use crate::rendering::buffers::{VertexBuffer, IndexBuffer, VertexConstantBuffer, FragmentConstantBuffer};
use crate::rendering::{Scene, Renderer};
use crate::input::{GameInput, LetterKeys};
use crate::math::{create_transformation_matrix, create_projection_matrix, create_view_matrix, Mat4x4};
use crate::rendering::texture::RenderableTexture;

#[derive(Debug)]
pub struct TextureDrawable {
    pub vertex_buffer: VertexBuffer<[f32; 6]>,
    pub index_buffer: IndexBuffer,
    pub transform_buffer: VertexConstantBuffer<f32>,
    pub shader: Shader,
    pub identity_buffer: VertexConstantBuffer<f32>,
    pub texture: RenderableTexture,
    pub position: [f32; 3],
    pub scale: [f32; 3],
    pub rotation: [f32; 3],
}

impl TextureDrawable {
    pub fn new(is_gui: bool, renderer: &Renderer, texture: RenderableTexture, position: [f32; 3], scale: [f32; 3], rotation: [f32; 3]) -> Self {
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

        let transform_mat = create_transformation_matrix(position, rotation, scale);
        let transform_buffer = VertexConstantBuffer::new(renderer, 0, transform_mat.raw_data().to_vec());
        let identity_buffer = VertexConstantBuffer::new(renderer, 1, Mat4x4::identity().to_vec());

        return Self {
            vertex_buffer: vb,
            index_buffer: ib,
            transform_buffer,
            shader,
            identity_buffer,
            texture,
            position,
            scale,
            rotation,
        }
    }

    pub fn set_position(&mut self, renderer: &Renderer, position: [f32; 3]) {
        self.position = position;
        self.update_transform(renderer);
    }

    pub fn set_scale(&mut self, renderer: &Renderer, scale: [f32; 3]) {
        self.scale = scale;
        self.update_transform(renderer);
    }

    pub fn set_rotation(&mut self, renderer: &Renderer, rotation: [f32; 3]) {
        self.rotation = rotation;
        self.update_transform(renderer);
    }

    fn update_transform(&self, renderer: &Renderer) {
        let transform_mat = create_transformation_matrix(self.position, self.rotation, self.scale);
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

#[derive(Debug)]
pub struct QuadDrawable {
    pub vertex_buffer: VertexBuffer<[f32; 4]>,
    pub index_buffer: IndexBuffer,
    pub transform_buffer: VertexConstantBuffer<f32>,
    pub shader: Shader,
    pub color_buffer: FragmentConstantBuffer<f32>,
    pub identity_buffer: VertexConstantBuffer<f32>,
    pub position: [f32; 3],
    pub scale: [f32; 3],
    pub rotation: [f32; 3],
}

impl QuadDrawable {
    pub fn new(is_gui: bool, renderer: &Renderer, color: [f32; 3], position: [f32; 3], scale: [f32; 3], rotation: [f32; 3]) -> Self {
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

        let shader = Shader::new(
                renderer,
                false,
                "shaders/quadVertexShader.txt",
                "shaders/quadFragmentShader.txt");

        let transform_mat = create_transformation_matrix(position, rotation, scale);
        let transform_buffer = VertexConstantBuffer::new(renderer,0, transform_mat.raw_data().to_vec());

        let alpha = if is_gui {
            0.5
        } else {
            1.0
        };
        let data = [color[0], color[1], color[2], alpha];
        let color_buffer = FragmentConstantBuffer::new(renderer, 0, data.to_vec());

        let identity_buffer = VertexConstantBuffer::new(renderer, 1, Mat4x4::identity().to_vec());
        return Self {
            vertex_buffer: vb,
            index_buffer: ib,
            transform_buffer,
            color_buffer,
            shader,
            identity_buffer,
            position,
            scale,
            rotation
        }
    }

    pub fn translate(&self, renderer: &Renderer, position: [f32; 3], scale: [f32; 3]) {
        let transform_mat = create_transformation_matrix(position, [0., 0., 0.], scale);
        self.transform_buffer.update_data(renderer, transform_mat.to_vec());
    }

    pub fn set_position(&mut self, renderer: &Renderer, position: [f32; 3]) {
        self.position = position;
        self.update_transform(renderer);
    }

    pub fn set_scale(&mut self, renderer: &Renderer, scale: [f32; 3]) {
        self.scale = scale;
        self.update_transform(renderer);
    }

    pub fn set_rotation(&mut self, renderer: &Renderer, rotation: [f32; 3]) {
        self.rotation = rotation;
        self.update_transform(renderer);
    }

    fn update_transform(&self, renderer: &Renderer) {
        let transform_mat = create_transformation_matrix(self.position, self.rotation, self.scale);
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
