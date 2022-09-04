use crate::rendering::buffers::{VertexBuffer, IndexBuffer, VertexConstantBuffer, FragmentConstantBuffer};
use crate::rendering::{Scene, Renderer};
use crate::math::{create_transformation_matrix, Mat4x4};
use crate::rendering::texture::RenderableTexture;
use crate::shader_lib::basic_shader::{basic_shader_layout};
use crate::shader_lib::font_shader::font_shader_layout;

#[derive(Debug)]
pub struct ShapeDrawable {
    pub shader_name: String,
    pub shape_name: String,
    pub color: [f32; 4]
}

impl ShapeDrawable {
    pub fn new(shader_name: String, shape_name: String, color: [f32; 4]) -> ShapeDrawable {
        return ShapeDrawable { shader_name, shape_name, color };
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color
    }
}

#[derive(Debug)]
pub struct TextureDrawable {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer,
    pub transform_buffer: VertexConstantBuffer,
    pub identity_buffer: VertexConstantBuffer,
    pub texture: RenderableTexture,
    pub position: [f32; 3],
    pub scale: [f32; 3],
    pub rotation: [f32; 3],
}

impl TextureDrawable {
    pub fn new(_is_gui: bool, renderer: &Renderer, texture: RenderableTexture) -> Self {
        let vb = VertexBuffer::new(
            renderer,
            font_shader_layout(),
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


    pub fn draw_with_projection(&self, scene: &Scene, camera_projection: &VertexConstantBuffer) {
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
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer,
    pub transform_buffer: VertexConstantBuffer,
    pub color_buffer: FragmentConstantBuffer,
    pub identity_buffer: VertexConstantBuffer,
    pub position: [f32; 3],
    pub scale: [f32; 3],
    pub rotation: [f32; 3],
}

impl QuadDrawable {
    pub fn new(is_gui: bool, renderer: &Renderer, color: [f32; 3]) -> Self {
        let vb = VertexBuffer::new(
            renderer,
            basic_shader_layout(),
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

    pub fn draw_with_projection(&self, scene: &Scene, camera_projection: &VertexConstantBuffer) {
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
