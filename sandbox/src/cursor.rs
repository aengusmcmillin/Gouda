use cgmath::{Matrix4, SquareMatrix, Vector3};
use gouda::rendering::{
    Renderer,
    Scene};
use std::rc::Rc;
use gouda::ecs::ECS;

pub struct Cursor {
    visible: bool,
    color: [f32; 4],
    transform: Matrix4<f32>,
}

impl Cursor {
    pub fn create(ecs: &mut ECS) {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let cursor = Cursor::new(renderer);
        ecs.add_res(cursor);
    }

    pub fn new(renderer: &Rc<Renderer>) -> Cursor {
        let mut res = Cursor {
            visible: false,
            color: [0., 0., 0., 1.],
            transform: Matrix4::identity(),
        };
        res
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_pos(&mut self, pos: [f32; 3]) {
        self.transform = Matrix4::from_translation(Vector3::new(pos[0], pos[1], pos[2]));
    }

    pub fn draw(&self, scene: &Scene) {
        if !self.visible {
            return;
        }
        scene.submit_shape_by_name("quad", "quad", (self.transform * Matrix4::from_nonuniform_scale(0.42, 0.02, 1.)) + Matrix4::from_translation(Vector3::new(0., 0.4, 0.)), self.color);
        scene.submit_shape_by_name("quad", "quad", (self.transform * Matrix4::from_nonuniform_scale(0.02, 0.42, 1.)) + Matrix4::from_translation(Vector3::new(-0.4, 0., 0.)), self.color);
        scene.submit_shape_by_name("quad", "quad", (self.transform * Matrix4::from_nonuniform_scale(0.42, 0.02, 1.)) + Matrix4::from_translation(Vector3::new(0., 0.4, 0.)), self.color);
        scene.submit_shape_by_name("quad", "quad", (self.transform * Matrix4::from_nonuniform_scale(0.02, 0.42, 1.)) + Matrix4::from_translation(Vector3::new(0., 0.4, 0.)), self.color);
    }
}

