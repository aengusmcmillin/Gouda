use gouda::rendering::drawable::QuadDrawable;
use gouda::rendering::{Renderer, Scene};
use gouda::ecs::{ECS, Entity, Mutation, Mutations};
use std::rc::Rc;
use crate::camera::Camera;
use gouda::input::GameInput;
use rand::Rng;
use crate::building::DamageDealt;

#[derive(Debug)]
pub struct Monster {
    drawable: QuadDrawable,
    pub x: f32,
    pub y: f32,
    health: u32,
}

impl Monster {
    pub fn create(ecs: &mut ECS, x_pos: f32, y_pos: f32) {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let monster_drawable = QuadDrawable::new(false, renderer, [0.7, 0.2, 0.2], [x_pos, y_pos, 0.], [0.4, 0.4, 1.], [0.; 3]);
        ecs.build_entity().add(Monster {drawable: monster_drawable, x: x_pos, y: y_pos, health: 2});
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

    pub fn take_damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage);
    }

    pub fn is_dead(&self) -> bool {
        return self.health == 0;
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

struct MonsterDamageMutation {
    monster: Entity,
    damage: u32,
}

impl Mutation for MonsterDamageMutation {
    fn apply(&self, ecs: &mut ECS) {
        ecs.remove_component::<DamageDealt>(&self.monster);
        let monster = ecs.write::<Monster>(&self.monster).unwrap();
        monster.take_damage(self.damage);
        if monster.is_dead() {
            ecs.delete_entity(&self.monster);
        }
    }
}

pub fn monster_damage_system(ecs: &ECS) -> Mutations {
    let mut mutations: Mutations = vec![];
    for (monster, damage, entity) in ecs.read2::<Monster, DamageDealt>() {
        mutations.push(Box::new(MonsterDamageMutation {monster: entity, damage: damage.damage}));
    }
    return mutations;
}