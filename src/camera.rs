use std::rc::Rc;

use cgmath::{Matrix4, ortho, SquareMatrix, Vector3, Deg};

use crate::{math::Mat4x4, rendering::{buffers::VertexConstantBuffer, Renderer}, ecs::ECS};

pub fn matrix_to_vec<T>(matrix: Matrix4<T>) -> Vec<T> {
    return vec![
        matrix.x.x, matrix.x.y, matrix.x.z, matrix.x.w,
        matrix.y.x, matrix.y.y, matrix.y.z, matrix.y.w,
        matrix.z.x, matrix.z.y, matrix.z.z, matrix.z.w,
        matrix.w.x, matrix.w.y, matrix.w.z, matrix.w.w,
    ]
}

pub trait Camera {
    fn set_position(&mut self, position: Vector3<f32>);
    fn set_rotation(&mut self, rotation: f32);
    fn get_view_projection_matrix(&self) -> Matrix4<f32>;
}

#[derive(Debug, Clone, Copy)]
pub struct OrthographicCamera {
    projection_matrix: Matrix4<f32>,
    view_matrix: Matrix4<f32>,
    view_projection_matrix: Matrix4<f32>,

    pub position: Vector3<f32>,
    pub rotation: f32,
}

impl Camera for OrthographicCamera {
    fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
        self.recalculate();
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.recalculate();
    }

    fn get_view_projection_matrix(&self) -> Matrix4<f32> {
        return self.view_projection_matrix
    }
}

impl OrthographicCamera {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        let mut res = Self {
            projection_matrix: Matrix4::new(1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0.5, 0., 0., 0., 0.5, 1.) * ortho(left, right, bottom, top, -1., 1.),
            view_matrix: Matrix4::identity(),
            view_projection_matrix: Matrix4::identity(),
            position: Vector3::new(0., 0., 0.),
            rotation: 0.,
        };
        res.recalculate();
        return res
    }

    fn recalculate(&mut self) {
        let transform = Matrix4::from_translation(Vector3::new(self.position.x, self.position.y, self.position.z)) * Matrix4::from_angle_z(Deg(self.rotation));
        let inverse = transform.invert();
        if let Some(inverted) = inverse {
            self.view_matrix = inverted;
            self.view_projection_matrix = self.projection_matrix * self.view_matrix
        }
    }
}


#[derive(Debug)]
pub struct NormCamera {
    pub projection_matrix: Mat4x4,
    pub projection_buffer: VertexConstantBuffer,
    center: [f32; 2],
    width: f32,
    aspect: f32,
}

impl NormCamera {
    pub fn create(ecs: &mut ECS)  {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let mut camera = NormCamera {
            projection_matrix: Mat4x4::identity(),
            projection_buffer: VertexConstantBuffer::new(renderer, 1, Mat4x4::identity().to_vec()),
            center: [0., 0.],
            width: 11.,
            aspect: 1.,
        };
        camera.update_projection_matrix();
        ecs.add_res(camera);
    }

    pub fn change_width(&mut self, dw: f32) {
        self.width += dw;
        self.update_projection_matrix();
    }

    pub fn change_pos(&mut self, dx: f32, dy: f32) {
        self.center[0] += dx;
        self.center[1] += dy;

        self.update_projection_matrix();
    }

    pub fn screen_space_to_world_space(&self, screen_x: f32, screen_y: f32) -> [f32; 2] {
        let height = self.width * self.aspect;
        let right = self.center[0] + self.width/2.;
        let left = self.center[0] - self.width/2.;
        let top = self.center[1] + height/2.;
        let bottom = self.center[1] - height/2.;

        let world_x = (screen_x + (right + left)/(right - left))  * (right - left) / 2.;
        let world_y = (screen_y + (top + bottom)/(top - bottom))  * (top - bottom) / 2.;
        return [world_x, world_y];
    }

    fn update_projection_matrix(&mut self) {
        let height = self.width * self.aspect;

        let right = self.center[0] + self.width/2.;
        let left = self.center[0] - self.width/2.;
        let top = self.center[1] + height/2.;
        let bottom = self.center[1] - height/2.;

        let projection = Mat4x4::new(
            [
                [2./(right - left), 0., 0., -1. * (right + left)/(right - left)],
                [0., 2./(top - bottom), 0., -1. * (top + bottom)/(top - bottom)],
                [0., 0., 1., 1.],
                [0., 0., 0., 1.],
            ]
        );

        self.projection_buffer.update_data(projection.to_vec());
        self.projection_matrix = projection;
    }
}
