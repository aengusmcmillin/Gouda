use gouda::TransformComponent;
use gouda::rendering::sprites::ColorBoxComponent;
use gouda::ecs::{ECS, Entity, Mutation, Mutations};
use crate::building::DamageDealt;

#[derive(Debug)]
pub struct Monster {
    health: u32,
}

impl Monster {
    pub fn create(ecs: &mut ECS, x_pos: f32, y_pos: f32) {
        let drawable = ColorBoxComponent::new(ecs, [0.7, 0.2, 0.2]);
        let transform = TransformComponent::builder().location(x_pos, y_pos).scale(0.4, 0.4).build();
        ecs.build_entity().add(Monster {health: 2}).add(drawable).add(transform);
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
        let monster = ecs.write::<TransformComponent>(&self.monster).unwrap();
        if monster.x > 0.03 {
            monster.change_pos(-0.06, 0.);
        } else if monster.x < -0.03 {
            monster.change_pos(0.06, 0.);
        } else if monster.y > 1.1 {
            monster.change_pos(0.0, -0.06);
        } else if monster.y < 0.9 {
            monster.change_pos(0.0, 0.06);
        } else {
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
    for (_, damage, entity) in ecs.read2::<Monster, DamageDealt>() {
        mutations.push(Box::new(MonsterDamageMutation {monster: entity, damage: damage.damage}));
    }
    return mutations;
}