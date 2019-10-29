use gouda::math::Mat4x4;
use gouda::rendering::{buffers::VertexBuffer, buffers::VertexConstantBuffer, Renderer};
use std::rc::Rc;
use gouda::ecs::ECS;

#[derive(Debug)]
pub struct Camera {
    pub projection_matrix: Mat4x4,
    pub projection_buffer: VertexConstantBuffer<f32>,
    center: [f32; 2],
    width: f32,
    aspect: f32,
}

impl Camera {
    pub fn create(ecs: &mut ECS)  {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let mut camera = Camera {
            projection_matrix: Mat4x4::identity(),
            projection_buffer: VertexConstantBuffer::new(renderer, 0, Mat4x4::identity().to_vec()),
            center: [0., 0.],
            width: 11.,
            aspect: 1.,
        };
        camera.update_projection_matrix(renderer);
        ecs.add_res(camera);
    }

    pub fn change_width(&mut self, renderer: &Renderer, dw: f32) {
        self.width += dw;
        self.update_projection_matrix(renderer);
    }

    pub fn change_pos(&mut self, renderer: &Renderer, dx: f32, dy: f32) {
        self.center[0] += dx;
        self.center[1] += dy;

        self.update_projection_matrix(renderer);
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

    fn update_projection_matrix(&mut self, renderer: &Renderer) {
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

        self.projection_buffer.update_data(renderer, projection.to_vec());
        self.projection_matrix = projection;
    }
}
