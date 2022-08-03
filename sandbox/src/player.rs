use gouda::{rendering::{
    drawable::{QuadDrawable},
    Renderer,
    Scene,
    sprites::SpriteSheetComponent}, TransformComponent};
use std::rc::Rc;
use gouda::ecs::{ECS, Mutations, Entity, Mutation};
use gouda::input::{GameInput, LetterKeys};
use gouda::camera::Camera;
use crate::tilemap::{Tilemap};

#[derive(Debug)]
pub struct Player {
    selected_drawable: QuadDrawable,
    pub current_tile: Entity,
    is_selected: bool,
}

impl Player {
    pub fn create(ecs: &mut ECS) {
        let spritesheet = SpriteSheetComponent::new(ecs, "bitmap/spritesheet.png".to_string(), 1, 4);

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let selected_drawable = QuadDrawable::new(false, renderer, [0.8, 0.8, 0.8]);

        let tile = ecs.read_res::<Tilemap>().tile_at_pos(1, 2);
        let transform = TransformComponent::builder().location(-4., -1.).scale(0.3, 0.3).build();

        ecs.build_entity()
            .add(Player {selected_drawable, current_tile: tile, is_selected: false})
            .add(transform)
            .add(spritesheet);
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera) {
        if self.is_selected {
            self.selected_drawable.draw_with_projection(&scene, &camera.projection_buffer);
        }
    }
}

struct MoveMutation {
    entity: Entity,
    dx: i32,
    dy: i32,
}

impl Mutation for MoveMutation {
    fn apply(&self, ecs: &mut ECS) {
        let playertransform = ecs.write::<TransformComponent>(&self.entity).unwrap();
        playertransform.change_pos(self.dx as f32, self.dy as f32);

        let spritesheet = ecs.write::<SpriteSheetComponent>(&self.entity).unwrap();
        if self.dx < 0 {
            spritesheet.active = 1;
        } else if self.dx > 0 {
            spritesheet.active = 3;
        } else if self.dy < 0 {
            spritesheet.active = 0;
        } else if self.dy > 0 {
            spritesheet.active = 2;
        }
    }
}

pub fn player_move_system(ecs: &ECS) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let mut mutations: Mutations = Vec::new();
    for (_, ent) in ecs.read1::<Player>() {
        if input.keyboard.letter_pressed(LetterKeys::A) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: -1, dy: 0}))
        } else if input.keyboard.letter_pressed(LetterKeys::D) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: 1, dy: 0}))
        }
        if input.keyboard.letter_pressed(LetterKeys::W) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: 0, dy: 1}))
        } else if input.keyboard.letter_pressed(LetterKeys::S) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: 0, dy: -1}))
        }
    }
    return mutations;
}
