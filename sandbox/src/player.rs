use gouda::rendering::{QuadDrawable, Renderer, Scene};
use std::rc::Rc;
use gouda::ecs::{ECS, Mutations, Entity, Mutation};
use gouda::input::{GameInput, LetterKeys};
use crate::camera::Camera;

#[derive(Debug)]
pub struct Player {
    drawable: QuadDrawable,
    x: f32,
    y: f32,
}

impl Player {
    pub fn create(ecs: &mut ECS) {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let player_drawable = QuadDrawable::new(false, renderer, [0.7, 0.7, 0.7], [-4., -1., 0.], [0.3, 0.3, 1.]);
        ecs.build_entity().add(Player {drawable: player_drawable, x: -4., y: -1.});
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera) {
        self.drawable.draw_with_projection(&scene, &camera.projection_buffer)
    }

    pub fn set_pos(&mut self, new_x: f32, new_y: f32) {
        self.x = new_x;
        self.y = new_y;
        self.drawable.translate([self.x, self.y, 0.], [0.3, 0.3, 1.]);
    }

    pub fn move_pos(&mut self, dx: f32, dy: f32) {
        self.set_pos(self.x + dx, self.y + dy);
    }
}

struct MoveMutation {
    entity: Entity,
    dx: f32,
    dy: f32,
}

impl Mutation for MoveMutation {
    fn apply(&self, ecs: &mut ECS) {
        let player = ecs.write::<Player>(&self.entity).unwrap();
        player.move_pos(self.dx, self.dy);
    }
}

pub fn player_move_system(ecs: &ECS) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let mut mutations: Mutations = Vec::new();
    for (p, ent) in ecs.read1::<Player>() {
        if input.keyboard.letter_pressed(LetterKeys::A) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: -1., dy: 0.}))
        } else if input.keyboard.letter_pressed(LetterKeys::D) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: 1., dy: 0.}))
        }
        if input.keyboard.letter_pressed(LetterKeys::W) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: 0., dy: 1.}))
        } else if input.keyboard.letter_pressed(LetterKeys::S) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: 0., dy: -1.}))
        }
    }
    return mutations;
}
