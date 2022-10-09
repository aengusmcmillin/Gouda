use gouda::transform::TransformComponent;
use gouda::ecs::{ECS, Entity, Mutations, Mutation};
use gouda::rendering::sprites::SpriteComponent;
use crate::tilemap::Tile;
use gouda::input::GameInput;
use crate::monster::Monster;

#[derive(Debug)]
pub struct Turret {
    pub selected: bool,
    fire_cooldown: f32,
    fire_timer: f32,
    range: f32,
    pub range_indicator: Option<Entity>,
}

impl Turret {
    pub fn create(ecs: &mut ECS, tile: Entity) {
        let tile = ecs.read::<Tile>(&tile).unwrap();

        let location = TransformComponent::builder().position(tile.x as f32, tile.y as f32).scale(0.7, 0.7).build();
        let turret = Turret {
            selected: false,
            fire_cooldown: 1.,
            fire_timer: 1.,
            range: 3.,
            range_indicator: None
        };

        let turret_sprite = SpriteComponent::new(ecs, "./assets/bitmap/turret2.png".to_string());
        ecs.build_entity()
           .add(location)
           .add(turret_sprite)
           .add(turret);
    }
}

#[derive(Debug)]
pub struct Arrow {
    target: Entity,
    speed: f32,
    damage: u32,
}

impl Arrow {
    pub fn create(ecs: &mut ECS, target: Entity, x: f32, y: f32) {
        let sprite = SpriteComponent::new(ecs, "./assets/bitmap/arrow.png".to_string());
        ecs.build_entity()
        .add(TransformComponent::builder().position(x, y).scale(0.3, 0.1).build())
        .add(sprite)
        .add(Arrow {target, speed: 5., damage: 1});
    }
}

#[derive(Debug)]
pub struct DamageDealt {
    pub damage: u32,
}

struct ArrowCollisionMutation {
    arrow: Entity,
}

impl Mutation for ArrowCollisionMutation {
    fn apply(&self, ecs: &mut ECS) {
        let arrow = ecs.read::<Arrow>(&self.arrow).unwrap();
        let target = arrow.target.clone();
        let damage = arrow.damage;
        ecs.delete_entity(&self.arrow);

        ecs.add_component(&target, DamageDealt {damage});
    }
}

struct ArrowDestroyMutation {
    arrow: Entity,
}

impl Mutation for ArrowDestroyMutation {
    fn apply(&self, ecs: &mut ECS) {
        ecs.delete_entity(&self.arrow);
    }
}

struct MoveArrowTowardsMutation {
    arrow: Entity,
    dx: f32,
    dy: f32,
}

impl Mutation for MoveArrowTowardsMutation {
    fn apply(&self, ecs: &mut ECS) {
        let arrow = ecs.write::<TransformComponent>(&self.arrow).unwrap();
        arrow.change_pos(self.dx, self.dy);
    }
}

pub fn arrow_move_system(ecs: &ECS, _dt: f32) -> Mutations {
    let mut mutations: Mutations = vec![];
    let dt = ecs.read_res::<GameInput>().seconds_to_advance_over_update;
    for (arrow, arrow_location, entity) in ecs.read2::<Arrow, TransformComponent>() {
        let target = ecs.read::<TransformComponent>(&arrow.target);
        if let Some(monster) = target {
            let v = (monster.position.x - arrow_location.position.x, monster.position.y - arrow_location.position.y);
            let dist = (v.0 * v.0 + v.1 * v.1).sqrt();
            if dist < 0.5 {
                mutations.push(Box::new(ArrowCollisionMutation {
                    arrow: entity,
                }));
            } else {
                mutations.push(Box::new(MoveArrowTowardsMutation {
                    arrow: entity,
                    dx: v.0 * dt / dist * arrow.speed,
                    dy: v.1 * dt / dist * arrow.speed,
                }))
            }
        } else {
            mutations.push(Box::new(ArrowDestroyMutation {
                arrow: entity,
            }));
        }
    }

    return mutations;
}


pub struct FireArrowMutation {
    pub turret: Entity,
    pub target: Entity,
}

impl Mutation for FireArrowMutation {
    fn apply(&self, ecs: &mut ECS) {
        let turret_loc = ecs.read::<TransformComponent>(&self.turret).unwrap();
        let (x, y) = (turret_loc.position.x, turret_loc.position.y);
        let turret = ecs.write::<Turret>(&self.turret).unwrap();
        turret.fire_timer = turret.fire_cooldown;
        Arrow::create(ecs, self.target, x, y);
    }
}

pub struct DecrTurretTimerMutation {
    pub dt: f32,
    pub turret: Entity,
}

impl Mutation for DecrTurretTimerMutation {
    fn apply(&self, ecs: &mut ECS) {
        let turret = ecs.write::<Turret>(&self.turret).unwrap();
        turret.fire_timer -= self.dt;
    }
}

pub fn turret_attack_system(ecs: &ECS, _dt: f32) -> Mutations {
    let mut mutations: Mutations = vec![];

    let mut monster_positions: Vec<(Entity, f32, f32)> = vec![];
    for (_, transform, entity) in ecs.read2::<Monster, TransformComponent>() {
        monster_positions.push((entity, transform.position.x, transform.position.y));
    }

    let input = ecs.read_res::<GameInput>();
    for (turret, loc, e) in ecs.read2::<Turret, TransformComponent>() {
        let mut closest: Option<(Entity, f32)> = None;
        for (monster, x, y) in &monster_positions {
            let (x, y) = (loc.position.x - x, loc.position.y - y);
            let dist = (x * x + y * y).sqrt();

            if let Some((_, closest_dist)) = closest {
                if dist < closest_dist {
                    closest = Some((monster.clone(), dist));
                }
            } else {
                closest = Some((monster.clone(), dist));
            }
        }

        if let Some((monster, dist)) = closest {
            if turret.fire_timer - input.seconds_to_advance_over_update <= 0. {
                if dist < turret.range {
                    mutations.push(Box::new(FireArrowMutation {turret: e, target: monster}));
                }
            } else {
                mutations.push(Box::new(DecrTurretTimerMutation {turret: e, dt: input.seconds_to_advance_over_update}));
            }
        }
    }

    return mutations;
}