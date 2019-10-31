use gouda::ecs::{ECS, Mutations, Entity, Mutation};
use crate::{Monster, Pos};
use gouda::input::GameInput;

pub struct ProcessSpawnerMutation {
    entity: Entity,
    dt: f32,
    x: f32,
    y: f32,
}

impl Mutation for ProcessSpawnerMutation {
    fn apply(&self, ecs: &mut ECS) {
        let spawner = ecs.write::<WaveSpawner>(&self.entity).unwrap();
        if let Some(spawned) = spawner.progress(self.dt) {
            Monster::create(ecs, self.x, self.y);
        }
    }
}
pub fn wave_spawner_system(ecs: &ECS) -> Mutations {
    let mut mutations: Mutations = Vec::new();
    let input = ecs.read_res::<GameInput>();

    for (position, _spawner, entity) in ecs.read2::<Pos, WaveSpawner>() {
        mutations.push(Box::new(ProcessSpawnerMutation {entity, dt: input.seconds_to_advance_over_update, x: position.x, y: position.y}));
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
    pub fn new(spec: WaveSpec, spawn_cd: f32) -> WaveSpawner {
        let num_monsters = spec.monsters.len();
        WaveSpawner {
            wave_spec: spec,
            num_monsters: num_monsters,
            current_monster_index: 0,
            spawn_max_cd: spawn_cd,
            spawn_current_cd: spawn_cd,
        }
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

#[derive(Debug)]
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