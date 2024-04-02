use crate::spawners::MonsterType::Wolf;
use crate::Monster;
use gouda::ecs::{Entity, Mutation, Mutations, ECS};
use gouda::input::GameInput;
use gouda::rendering::drawable::ShapeDrawable;
use gouda::transform::TransformComponent;

pub struct ProcessSpawnerMutation {
    entity: Entity,
    dt: f32,
    x: f32,
    y: f32,
}

impl Mutation for ProcessSpawnerMutation {
    fn apply(&self, ecs: &mut ECS) {
        let spawner = ecs.write::<WaveSpawner>(&self.entity).unwrap();
        if let Some(_) = spawner.progress(self.dt) {
            Monster::create(ecs, self.x, self.y);
        }

        let spawner = ecs.read::<WaveSpawner>(&self.entity).unwrap();
        if spawner.is_finished() {
            ecs.delete_entity(&self.entity);
        }
    }
}
pub fn wave_spawner_system(ecs: &ECS, _dt: f32) -> Mutations {
    let mut mutations: Mutations = Vec::new();
    let input = ecs.read_res::<GameInput>();

    for (_, transform, entity) in ecs.read2::<WaveSpawner, TransformComponent>() {
        mutations.push(Box::new(ProcessSpawnerMutation {
            entity,
            dt: input.seconds_to_advance_over_update,
            x: transform.position.x,
            y: transform.position.y,
        }));
    }

    return mutations;
}

#[derive(Debug)]
pub struct WaveSpawner {
    wave_spec: WaveSpec,
    num_monsters: usize,
    current_monster_index: usize,
    spawn_max_cd: f32,
    spawn_current_cd: f32,
}

impl WaveSpawner {
    pub fn create(ecs: &mut ECS, spec: WaveSpec, x: f32, y: f32, spawn_cd: f32) {
        let num_monsters = spec.monsters.len();
        let color_box = ShapeDrawable::new("quad", "quad", [0.8, 0.8, 0.2, 1.]);
        let transform = TransformComponent::builder()
            .position(x, y)
            .scale(0.2, 0.2)
            .build();
        let spawner = WaveSpawner {
            wave_spec: spec,
            num_monsters: num_monsters,
            current_monster_index: 0,
            spawn_max_cd: spawn_cd,
            spawn_current_cd: spawn_cd,
        };
        ecs.build_entity()
            .add_component(spawner)
            .add_component(transform)
            .add_component(color_box);
    }

    pub fn progress(&mut self, dt: f32) -> Option<MonsterSpec> {
        if self.is_finished() {
            return None;
        }

        self.spawn_current_cd -= dt;
        if self.spawn_current_cd <= 0. {
            self.spawn_current_cd = self.spawn_max_cd + self.spawn_current_cd;
            let res = Some(self.wave_spec.monsters[self.current_monster_index]);
            self.current_monster_index += 1;
            return res;
        }
        return None;
    }

    pub fn is_finished(&self) -> bool {
        return self.current_monster_index >= self.num_monsters;
    }
}

#[derive(Debug, Clone)]
pub struct WaveSpec {
    pub monsters: Vec<MonsterSpec>,
}

#[derive(Debug, Clone, Copy)]
pub struct MonsterSpec {
    pub monster_type: MonsterType,
}

#[derive(Debug, Clone, Copy)]
pub enum MonsterType {
    Wolf,
}

#[derive(Debug, Clone)]
pub struct SpawnerSpec {
    pub wave: WaveSpec,
}

pub struct GameDay {
    pub day_length: f32,
    pub night_length: f32,
    pub waves: Vec<SpawnerSpec>,
}

pub fn generate_days() -> Vec<GameDay> {
    return vec![
        GameDay {
            day_length: 5.,
            night_length: 20.,
            waves: vec![SpawnerSpec {
                wave: WaveSpec {
                    monsters: vec![MonsterSpec { monster_type: Wolf }; 15],
                },
            }],
        },
        GameDay {
            day_length: 5.,
            night_length: 30.,
            waves: vec![SpawnerSpec {
                wave: WaveSpec {
                    monsters: vec![MonsterSpec { monster_type: Wolf }; 20],
                },
            }],
        },
        GameDay {
            day_length: 5.,
            night_length: 40.,
            waves: vec![SpawnerSpec {
                wave: WaveSpec {
                    monsters: vec![MonsterSpec { monster_type: Wolf }; 25],
                },
            }],
        },
        GameDay {
            day_length: 5.,
            night_length: 50.,
            waves: vec![SpawnerSpec {
                wave: WaveSpec {
                    monsters: vec![MonsterSpec { monster_type: Wolf }; 30],
                },
            }],
        },
        GameDay {
            day_length: 5.,
            night_length: 60.,
            waves: vec![SpawnerSpec {
                wave: WaveSpec {
                    monsters: vec![MonsterSpec { monster_type: Wolf }; 35],
                },
            }],
        },
        GameDay {
            day_length: 5.,
            night_length: 70.,
            waves: vec![SpawnerSpec {
                wave: WaveSpec {
                    monsters: vec![MonsterSpec { monster_type: Wolf }; 40],
                },
            }],
        },
    ];
}
