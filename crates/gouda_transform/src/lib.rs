use cgmath::{Deg, Matrix4, Vector2, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct TransformComponent {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl TransformComponent {
    pub fn change_pos(&mut self, dx: f32, dy: f32) {
        self.position = self.position + Vector3::new(dx, dy, 0.);
    }

    pub fn builder() -> TransformComponentBuilder {
        TransformComponentBuilder::new()
    }

    pub fn transform_matrix(&self) -> Matrix4<f32> {
        return Matrix4::from_translation(Vector3::new(self.position.x, self.position.y, 0.))
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, 1.)
            * Matrix4::from_angle_x(Deg(self.rotation.x))
            * Matrix4::from_angle_y(Deg(self.rotation.y));
    }
}

pub struct TransformComponentBuilder {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl TransformComponentBuilder {
    pub fn new() -> TransformComponentBuilder {
        TransformComponentBuilder {
            position: Vector3::new(0., 0., 0.),
            rotation: Vector3::new(0., 0., 0.),
            scale: Vector3::new(1., 1., 1.),
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> TransformComponentBuilder {
        self.position = Vector3::new(x, y, 0.);
        self
    }

    pub fn scale(mut self, scale_x: f32, scale_y: f32) -> TransformComponentBuilder {
        self.scale = Vector3::new(scale_x, scale_y, 1.);
        self
    }

    pub fn rotation(mut self, rot_x: f32, rot_y: f32) -> TransformComponentBuilder {
        self.rotation = Vector3::new(rot_x, rot_y, 0.);
        self
    }

    pub fn build(self) -> TransformComponent {
        TransformComponent {
            position: self.position,
            scale: self.scale,
            rotation: self.rotation,
        }
    }
}
