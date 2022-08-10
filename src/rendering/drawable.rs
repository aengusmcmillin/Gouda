use std::rc::Rc;

use crate::ecs::ECS;
use crate::rendering::buffers::{VertexBuffer, IndexBuffer, VertexConstantBuffer, FragmentConstantBuffer};
use crate::rendering::{Scene, Renderer};
use crate::math::{create_transformation_matrix, Mat4x4};
use crate::rendering::texture::RenderableTexture;


#[derive(Debug)]
pub struct ShapeDrawable {
    vertex_buffer: VertexBuffer<[f32; 2]>,
    index_buffer: IndexBuffer,
    num_indices: u64,
    pub color_buffer: FragmentConstantBuffer<f32>,
}

impl ShapeDrawable {
    pub fn new(ecs: &mut ECS, verts: Vec<[f32; 2]>, indices: Vec<i16>) -> ShapeDrawable {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let num_indices = indices.len() as u64;
        let index_buffer = IndexBuffer::new(renderer, indices);
        let vertex_buffer = VertexBuffer::new(renderer, 0, verts);

        let color_data = [0.2, 0.2, 0.2, 1.];
        let color_buffer = FragmentConstantBuffer::new(renderer, 0, color_data.to_vec());

        return ShapeDrawable { vertex_buffer, index_buffer, num_indices, color_buffer };
    }

    pub fn draw(&self, scene: &Scene) {
        scene.bind_shader("hex".to_string());
        self.vertex_buffer.bind(scene);
        self.color_buffer.bind(scene);
        scene.draw_indexed(self.num_indices, &self.index_buffer);
    }

    pub fn set_color(&self, color: [f32; 4]) {
        self.color_buffer.update_data(color.to_vec());
    }
}

#[derive(Debug)]
pub struct TextureDrawable {
    pub vertex_buffer: VertexBuffer<[f32; 6]>,
    pub index_buffer: IndexBuffer,
    pub transform_buffer: VertexConstantBuffer<f32>,
    pub identity_buffer: VertexConstantBuffer<f32>,
    pub texture: RenderableTexture,
    pub position: [f32; 3],
    pub scale: [f32; 3],
    pub rotation: [f32; 3],
}

impl TextureDrawable {
    pub fn new(_is_gui: bool, renderer: &Renderer, texture: RenderableTexture) -> Self {
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

        let position = [0.; 3];
        let scale = [1.; 3];
        let rotation = [0.; 3];

        let transform_mat = create_transformation_matrix(position, rotation, scale);
        let transform_buffer = VertexConstantBuffer::new(renderer, 0, transform_mat.raw_data().to_vec());
        let identity_buffer = VertexConstantBuffer::new(renderer, 1, Mat4x4::identity().to_vec());

        return Self {
            vertex_buffer: vb,
            index_buffer: ib,
            transform_buffer,
            identity_buffer,
            texture,
            position,
            scale,
            rotation,
        }
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
        self.update_transform();
    }

    pub fn set_scale(&mut self, scale: [f32; 3]) {
        self.scale = scale;
        self.update_transform();
    }

    pub fn set_rotation(&mut self, rotation: [f32; 3]) {
        self.rotation = rotation;
        self.update_transform();
    }

    pub fn apply_transform(&self, position: [f32; 3], scale: [f32; 3], rotation: [f32; 3]) {
        let transform_mat = create_transformation_matrix(position, rotation, scale);
        self.transform_buffer.update_data(transform_mat.to_vec());
    }

    fn update_transform(&self) {
        let transform_mat = create_transformation_matrix(self.position, self.rotation, self.scale);
        self.transform_buffer.update_data(transform_mat.to_vec());
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
        scene.bind_shader("texture".to_string());
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
    pub color_buffer: FragmentConstantBuffer<f32>,
    pub identity_buffer: VertexConstantBuffer<f32>,
    pub position: [f32; 3],
    pub scale: [f32; 3],
    pub rotation: [f32; 3],
}

impl QuadDrawable {
    pub fn new(is_gui: bool, renderer: &Renderer, color: [f32; 3]) -> Self {
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

        let position = [0.; 3];
        let scale = [1.; 3];
        let rotation = [0.; 3];

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
            identity_buffer,
            position,
            scale,
            rotation
        }
    }

    pub fn translate(&self, position: [f32; 3], scale: [f32; 3]) {
        let transform_mat = create_transformation_matrix(position, [0., 0., 0.], scale);
        self.transform_buffer.update_data(transform_mat.to_vec());
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
        self.update_transform();
    }

    pub fn set_scale(&mut self, scale: [f32; 3]) {
        self.scale = scale;
        self.update_transform();
    }

    pub fn set_rotation(&mut self, rotation: [f32; 3]) {
        self.rotation = rotation;
        self.update_transform();
    }

    pub fn apply_transform(&self, position: [f32; 3], scale: [f32; 3], rotation: [f32; 3]) {
        let transform_mat = create_transformation_matrix(position, rotation, scale);
        self.transform_buffer.update_data(transform_mat.to_vec());
    }

    fn update_transform(&self) {
        let transform_mat = create_transformation_matrix(self.position, self.rotation, self.scale);
        self.transform_buffer.update_data(transform_mat.to_vec());
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
        scene.bind_shader("quad".to_string());
        self.vertex_buffer.bind(scene);
        self.transform_buffer.bind(scene);
        self.index_buffer.bind(scene);
        self.color_buffer.bind(&scene);

        scene.draw_indexed(6, &self.index_buffer);
    }
}
