use gouda::{
    camera::{Camera, OrthographicCamera},
    ecs::{Entity, Mutation, Mutations, ECS},
    input::{GameInput, LetterKeys},
    transform::TransformComponent,
};

#[derive(Debug)]
pub struct CameraComponent {
    pub move_speed: f32,
    pub active: bool,
}

impl CameraComponent {
    pub fn new() -> CameraComponent {
        return CameraComponent {
            move_speed: 30.,
            active: true,
        };
    }
}

pub struct CameraMoveMutation {
    dx: f32,
    dy: f32,
    camera_entity: Entity,
}

impl Mutation for CameraMoveMutation {
    fn apply(&self, ecs: &mut ECS) {
        let transform = ecs
            .write::<TransformComponent>(&self.camera_entity)
            .unwrap();
        transform.change_pos(self.dx, self.dy);
    }
}

pub struct OrthoScaleMutation {
    size: f32,
    camera_entity: Entity,
}

impl Mutation for OrthoScaleMutation {
    fn apply(&self, ecs: &mut ECS) {
        let ortho = ecs
            .write::<OrthographicCamera>(&self.camera_entity)
            .unwrap();
        ortho.set_size(self.size);
    }
}

pub fn camera_control_system(ecs: &ECS, dt: f32) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let cameras = ecs.read2::<CameraComponent, TransformComponent>();

    let mut dx = 0.;
    let mut dy = 0.;

    let mut mutations: Mutations = vec![];

    for (camera, _, camera_entity) in cameras {
        if input.keyboard.letter_down(LetterKeys::A) {
            dx -= camera.move_speed * dt
        }
        if input.keyboard.letter_down(LetterKeys::D) {
            dx += camera.move_speed * dt
        }
        if input.keyboard.letter_down(LetterKeys::W) {
            dy += camera.move_speed * dt
        }
        if input.keyboard.letter_down(LetterKeys::S) {
            dy -= camera.move_speed * dt
        }
        if dx != 0. || dy != 0. {
            mutations.push(Box::new(CameraMoveMutation {
                dx,
                dy,
                camera_entity,
            }));
        }

        if let Some(ortho) = ecs.read::<OrthographicCamera>(&camera_entity) {
            if input.keyboard.letter_down(LetterKeys::Q) {
                mutations.push(Box::new(OrthoScaleMutation {
                    size: ortho.get_size() + camera.move_speed * dt,
                    camera_entity,
                }));
            } else if input.keyboard.letter_down(LetterKeys::E) {
                mutations.push(Box::new(OrthoScaleMutation {
                    size: ortho.get_size() - camera.move_speed * dt,
                    camera_entity,
                }));
            }
        }
    }

    return mutations;
}
