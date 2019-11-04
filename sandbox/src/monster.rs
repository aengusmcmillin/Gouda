use gouda::rendering::drawable::QuadDrawable;
use gouda::rendering::{Renderer, Scene};
use gouda::ecs::{ECS, Entity, Mutation, Mutations};
use std::rc::Rc;
use crate::camera::Camera;
use gouda::input::GameInput;
use rand::Rng;

#[derive(Debug)]
pub struct Monster {
    drawable: QuadDrawable,
    x: f32,
    y: f32,
}

impl Monster {
    pub fn create(ecs: &mut ECS, x_pos: f32, y_pos: f32) {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let monster_drawable = QuadDrawable::new(false, renderer, [0.7, 0.2, 0.2], [x_pos, y_pos, 0.], [0.4, 0.4, 1.]);
        ecs.build_entity().add(Monster {drawable: monster_drawable, x: x_pos, y: y_pos});
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera) {
        self.drawable.draw_with_projection(&scene, &camera.projection_buffer);
    }

    pub fn move_pos(&mut self, renderer: &Renderer, dx: f32, dy: f32) {
        self.set_pos(renderer, self.x + dx, self.y + dy);
    }

    pub fn set_pos(&mut self, renderer: &Renderer, new_x: f32, new_y: f32) {
        self.x = new_x;
        self.y = new_y;
        self.drawable.translate(renderer, [self.x, self.y, 0.], [0.3, 0.3, 1.]);
    }
}

pub struct MonsterMoveMutation {
    monster: Entity,
}

impl Mutation for MonsterMoveMutation {
    fn apply(&self, ecs: &mut ECS) {
        let renderer = ecs.read_res::<Rc<Renderer>>().clone();
        let monster = ecs.write::<Monster>(&self.monster).unwrap();
        if monster.x > 0.03 {
            monster.move_pos(&renderer, -0.06, 0.);
        } else if monster.x < -0.03 {
            monster.move_pos(&renderer, 0.06, 0.);
        } else if monster.y > 1.1 {
            monster.move_pos(&renderer, 0.0, -0.06);
        } else if monster.y < 0.9 {
            monster.move_pos(&renderer, 0.0, 0.06);
        } else {
            println!("Deleting");
            ecs.remove_component::<Monster>(&self.monster);
            ecs.delete_entity(&self.monster);
        }
    }
}

pub fn monster_move_system(ecs: &ECS) -> Mutations {
    let mut mutations: Mutations = Vec::new();
    for monster in ecs.get1::<Monster>() {
        mutations.push(Box::new(MonsterMoveMutation {monster}));
    }
    return mutations;
}

