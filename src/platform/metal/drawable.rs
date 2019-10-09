use crate::platform::metal::shader::Shader;
use crate::platform::metal::buffers::{VertexBuffer, IndexBuffer, FragmentBuffer};
use crate::platform::metal::{Scene, Renderer};
use crate::input::{GameInput, LetterKeys};
use crate::math::{create_transformation_matrix, create_projection_matrix, create_view_matrix, Mat4x4};
use crate::platform::metal::texture::RenderableTexture;

pub trait Drawable {
    fn update(&mut self, input: &GameInput);
    fn bind(&self, scene: &Scene);
    fn draw(&self, scene: &Scene);
}

#[derive(Debug)]
pub struct TextureDrawable {
    pub vertex_buffer: VertexBuffer,
    pub texture_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer,
    pub transform_buffer: VertexBuffer,
    pub shader: Shader,
    pub identity_buffer: VertexBuffer,
    pub texture: RenderableTexture,
}

impl TextureDrawable {
    pub fn new(is_gui: bool, renderer: &Renderer, texture: RenderableTexture, position: [f32; 3], scale: [f32; 3]) -> Self {
        let vb = VertexBuffer::new(
            renderer,
            0,
            vec![
                -1., 1., 0., 1.,
                -1., -1., 0., 1.,
                1., -1., 0., 1.,
                1., 1., 0., 1.,
            ]);

        let tb = VertexBuffer::new(
            renderer,
            1,
            vec![
                0., 0.,
                0., 1.,
                1., 1.,
                1., 0.,
            ]);

        let ib = IndexBuffer::new(
            renderer,
            vec![
                0, 1, 3,
                3, 1, 2,
            ]);

        let shader = Shader::new(
            renderer,
            "shaders/textureShader.txt",
            "shaders/textureShader.txt");

        let transform_mat = create_transformation_matrix(position, [0., 0., 0.], scale);
        let transform_buffer = VertexBuffer::new(renderer,2, transform_mat.raw_data().to_vec());
        let identity_buffer = VertexBuffer::new(renderer, 3,Mat4x4::identity().to_vec());

        return Self {
            vertex_buffer: vb,
            texture_buffer: tb,
            index_buffer: ib,
            transform_buffer,
            shader,
            identity_buffer,
            texture
        }
    }

    pub fn translate(&self, position: [f32; 3], scale: [f32; 3]) {
        let transform_mat = create_transformation_matrix(position, [0., 0., 0.], scale);
        self.transform_buffer.update_data(transform_mat.to_vec());
    }

    pub fn draw_with_projection(&self, scene: &Scene, camera_projection: &VertexBuffer) {
        camera_projection.bind_to_offset(scene, 3);
        self.draw_impl(scene);
    }

    pub fn draw(&self, scene: &Scene) {
        self.identity_buffer.bind(scene);
        self.draw_impl(scene);
    }

    fn draw_impl(&self, scene: &Scene) {
        self.shader.bind(scene);
        self.vertex_buffer.bind(scene);
        self.texture_buffer.bind(scene);
        self.transform_buffer.bind(scene);
        self.texture.bind(scene);

        scene.draw_indexed(6, &self.index_buffer);
    }
}

#[derive(Debug)]
pub struct QuadDrawable {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer,
    pub transform_buffer: VertexBuffer,
    pub shader: Shader,
    pub color_buffer: FragmentBuffer,
    pub identity_buffer: VertexBuffer,
}

impl QuadDrawable {
    pub fn new(is_gui: bool, renderer: &Renderer, color: [f32; 3], position: [f32; 3], scale: [f32; 3]) -> Self {
        let vb = VertexBuffer::new(
            renderer,
            0,
            vec![
                -1., -1., 0., 1.,
                1., -1., 0., 1.,
                1., 1., 0., 1.,
                -1., 1., 0., 1.,
            ]);

        let ib = IndexBuffer::new(
            renderer,
            vec![
                0, 1, 2, 2, 3, 0
            ]);

        let shader = if is_gui {
            Shader::new(
                renderer,
                "shaders/guiVertexShader.txt",
                "shaders/guiFragmentShader.txt")
        } else {
            Shader::new(
                renderer,
                "shaders/quadVertexShader.txt",
                "shaders/quadFragmentShader.txt")
        };

        let transform_mat = create_transformation_matrix(position, [0., 0., 0.], scale);
        let transform_buffer = VertexBuffer::new(renderer,1, transform_mat.raw_data().to_vec());

        let color_buffer = FragmentBuffer::new(renderer, 0, vec![color[0], color[1], color[2]]);

        let identity_buffer = VertexBuffer::new(renderer, 2, Mat4x4::identity().to_vec());
        return Self {
            vertex_buffer: vb,
            index_buffer: ib,
            transform_buffer,
            color_buffer,
            shader,
            identity_buffer,
        }
    }

    pub fn translate(&self, position: [f32; 3], scale: [f32; 3]) {
        let transform_mat = create_transformation_matrix(position, [0., 0., 0.], scale);
        self.transform_buffer.update_data(transform_mat.to_vec());
    }

    pub fn draw_with_projection(&self, scene: &Scene, camera_projection: &VertexBuffer) {
        camera_projection.bind_to_offset(scene, 2);
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

        self.color_buffer.bind(&scene);

        scene.draw_indexed(6, &self.index_buffer);
    }
}

pub struct CubeDrawable {
    shader: Shader,
    vertex_buffer: VertexBuffer,
    color_buffer: VertexBuffer,
    transform_buffer: VertexBuffer,
    projection_buffer: VertexBuffer,
    view_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    scale: f32,
    translate: [f32; 3],

    xrot: f32,
    yrot: f32,
    zrot: f32,
}

impl CubeDrawable {
    pub fn new(renderer: &Renderer, color: [f32; 3], scale: f32, translate: [f32; 3]) -> CubeDrawable {
        let shader = Shader::new(&renderer, "shaders/vertexShader.txt", "shaders/fragmentShader.txt");

        let position_data = vec![
            -0.5, 0.5, -0.5, 1.,
            -0.5, -0.5, -0.5, 1.,
            0.5, -0.5, -0.5, 1.,
            0.5, 0.5, -0.5, 1.,

            -0.5, 0.5, 0.5, 1.,
            -0.5, -0.5, 0.5, 1.,
            0.5, -0.5, 0.5, 1.,
            0.5, 0.5, 0.5, 1.,

            0.5, 0.5, -0.5, 1.,
            0.5, -0.5, -0.5, 1.,
            0.5, -0.5, 0.5, 1.,
            0.5, 0.5, 0.5, 1.,

            -0.5, 0.5, -0.5, 1.,
            -0.5, -0.5, -0.5, 1.,
            -0.5, -0.5, 0.5, 1.,
            -0.5, 0.5, 0.5, 1.,

            -0.5, 0.5, 0.5, 1.,
            -0.5, 0.5, -0.5, 1.,
            0.5, 0.5, -0.5, 1.,
            0.5, 0.5, 0.5, 1.,

            -0.5, -0.5, 0.5, 1.,
            -0.5, -0.5, -0.5, 1.,
            0.5, -0.5, -0.5, 1.,
            0.5, -0.5, 0.5, 1.,
        ];
        let vertex_buffer = VertexBuffer::new(renderer, 0, position_data);

        let color_data = vec![
            color[0], color[1], color[2], 1.,
        ];
        let color_buffer = VertexBuffer::new(renderer, 1, color_data);

        let transform_mat = create_transformation_matrix([0., 0., 0.], [0., 0., 0.], [1., 1., 1.]);
        let transform_buffer = VertexBuffer::new(renderer,2, transform_mat.raw_data().to_vec());

        let projection_mat = create_projection_matrix(1., 75., 100., 0.1);
        let projection_buffer = VertexBuffer::new(renderer, 3, projection_mat.raw_data().to_vec());

        let view_mat = create_view_matrix(0., 0., [0., 0., 0.]);
        let view_buffer = VertexBuffer::new(renderer, 4, view_mat.raw_data().to_vec());

        let indices = vec![
            0,1,3,
            3,1,2,
            4,5,7,
            7,5,6,
            8,9,11,
            11,9,10,
            12,13,15,
            15,13,14,
            16,17,19,
            19,17,18,
            20,21,23,
            23,21,22,
        ];
        let index_buffer = IndexBuffer::new(renderer, indices);

        return CubeDrawable {
            shader,
            vertex_buffer,
            color_buffer,
            transform_buffer,
            projection_buffer,
            view_buffer,
            index_buffer,
            scale,
            translate,
            xrot: 0.,
            yrot: 0.,
            zrot: 0.,
        };
    }
}

impl Drawable for CubeDrawable {
    fn update(&mut self, input: &GameInput) {
        let dt = input.seconds_to_advance_over_update;
        if input.keyboard.letter_down(LetterKeys::W) {
            self.xrot += 100. * dt;
        }
        if input.keyboard.letter_down(LetterKeys::S) {
            self.xrot -= 100. * dt;
        }
        if input.keyboard.letter_down(LetterKeys::A) {
            self.yrot += 100. * dt;
        }
        if input.keyboard.letter_down(LetterKeys::D) {
            self.yrot -= 100. * dt;
        }
        if input.keyboard.letter_down(LetterKeys::Q) {
            self.zrot += 100. * dt;
        }
        if input.keyboard.letter_down(LetterKeys::E) {
            self.zrot -= 100. * dt;
        }

        self.xrot = self.xrot % 360.;
        self.yrot = self.yrot % 360.;
        self.zrot = self.zrot % 360.;
        let transform_mat = create_transformation_matrix(self.translate, [self.xrot, self.yrot, self.zrot], [self.scale, self.scale, self.scale]);
        self.transform_buffer.update_data(transform_mat.to_vec());
    }

    fn bind(&self, scene: &Scene) {
        self.shader.bind(&scene);
        self.vertex_buffer.bind(scene);
        self.color_buffer.bind(scene);
        self.transform_buffer.bind(scene);
        self.projection_buffer.bind(scene);
        self.view_buffer.bind(scene);
    }

    fn draw(&self, scene: &Scene) {
        scene.draw_indexed(36, &self.index_buffer);
    }
}

pub struct TriangleDrawable {
    shader: Shader,
    vertex_buffers: Vec<VertexBuffer>,
}

impl TriangleDrawable {
    fn update(&mut self) { }

    pub fn new(renderer: &Renderer) -> TriangleDrawable {
        let position_data = vec![0.0f32, 0.5, 0., 1., -0.5, -0.5, 0., 1., 0.5, -0.5, 0., 1.];
        let color_data = vec![1.0f32, 0., 0., 1., 0., 1., 0., 1., 0., 0., 1., 1.];
        let shader = Shader::new(&renderer, "shaders/vertexShader.txt", "shaders/fragmentShader.txt");
        let vbuf = VertexBuffer::new(renderer, 0, position_data);
        let cbuf = VertexBuffer::new(renderer, 1, color_data);

        return TriangleDrawable {
            shader,
            vertex_buffers: vec![vbuf, cbuf],
        };
    }
}

impl Drawable for TriangleDrawable {
    fn update(&mut self, input: &GameInput) {}

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
