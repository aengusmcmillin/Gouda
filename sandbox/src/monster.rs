use gouda::TransformComponent;
use gouda::rendering::drawable::ShapeDrawable;
use gouda::ecs::{ECS, Entity, Mutation, Mutations};
use crate::building::DamageDealt;

#[derive(Debug)]
pub struct Monster {
    health: u32,
}

impl Monster {
    pub fn create(ecs: &mut ECS, x_pos: f32, y_pos: f32) {
        let color = [0.7, 0.2, 0.2, 1.];
        let shape = ShapeDrawable::new("quad", "quad", color);
        let transform = TransformComponent::builder().position(x_pos, y_pos).scale(0.8, 0.8).build();
        ecs.build_entity().add(Monster {health: 2}).add(shape).add(transform);
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
    dx: f32,
    dy: f32,
    delete: bool,
}

impl Mutation for MonsterMoveMutation {
    fn apply(&self, ecs: &mut ECS) {
        if self.delete {
            ecs.delete_entity(&self.monster);
            return;
        }

        let monster = ecs.write::<TransformComponent>(&self.monster).unwrap();
        monster.change_pos(self.dx, self.dy);
    }
}

pub fn monster_move_system(ecs: &ECS, dt: f32) -> Mutations {
    let mut mutations: Mutations = Vec::new();
    for (_, transform, monster) in ecs.read2::<Monster, TransformComponent>() {
        let mut dx = 0.;
        let mut dy = 0.;
        let mut delete = false;

        let monster_speed = 2.;
        if transform.position.x > 0.03 {
            dx = -1. * monster_speed * dt;
        } else if transform.position.x < -0.03 {
            dx = 1. * monster_speed * dt;
        } else if transform.position.y > 1.1 {
            dy = -1. * monster_speed * dt;
        } else if transform.position.y < 0.9 {
            dy = 1. * monster_speed * dt;
        } else {
            delete = true;
        }
        mutations.push(Box::new(MonsterMoveMutation {monster, dx, dy, delete}));
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

pub fn monster_damage_system(ecs: &ECS, dt: f32) -> Mutations {
    let mut mutations: Mutations = vec![];
    for (_, damage, entity) in ecs.read2::<Monster, DamageDealt>() {
        mutations.push(Box::new(MonsterDamageMutation {monster: entity, damage: damage.damage}));
    }
    return mutations;
}