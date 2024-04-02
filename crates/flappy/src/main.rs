use core::time;
use std::{
    collections::{HashMap, VecDeque},
    env,
    time::SystemTime,
};

use gouda::{
    camera::{Camera, OrthographicCamera},
    ecs::{Entity, GameSceneId, Mutation, Mutations, ECS},
    input::{GameInput, SpecialKeys},
    rendering::{sprites::{ColorBoxComponent, SpriteSheetComponent}, Scene},
    transform::{self, TransformComponent},
    window::WindowProps,
    GameLogic, GameScene, Gouda, QuitEvent, RenderLayer,
};

pub const START_MENU_SCENE: GameSceneId = 0;

#[derive(Debug)]
struct Collider {
    width: f32,
    height: f32,
}

#[derive(Debug)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Debug)]
struct HasGravity {
    gravity: f32,
}

#[derive(Debug)]
struct Animation {
    frame_duration_ms: f32,
    countdown: f32,
}

#[derive(Debug)]
struct Bird {}

impl Bird {
    pub fn create(ecs: &mut ECS) {
        let transform = TransformComponent::builder()
            .position(-4., -1.)
            .scale(0.8, 0.8)
            .build();
        let velocity = Velocity { dx: 0., dy: 1. };
        let gravity = HasGravity { gravity: -20. };
        let collider = Collider {
            width: 0.3,
            height: 0.3,
        };
        let sprite = SpriteSheetComponent::new(ecs, "./assets/bitmap/cheese.png".to_owned(), 1, 2);
        let animation = Animation { frame_duration_ms: 200., countdown: 0. };
        ecs.build_entity()
            .add_component(collider)
            .add_component(velocity)
            .add_component(gravity)
            .add_component(transform)
            .add_component(sprite)
            .add_component(animation)
            .add_component(Bird {});
    }
}

#[derive(Debug)]
struct Floor {}

impl Floor {
    pub fn create(ecs: &mut ECS) {
        let color = ColorBoxComponent::new([0.2, 0.8, 0.8]);
        let transform = TransformComponent::builder()
            .position(-10., -8.)
            .scale(20.0, 3.0)
            .build();
        let collider = Collider {
            width: 20.,
            height: 3.,
        };
        ecs.build_entity()
            .add_component(collider)
            .add_component(transform)
            .add_component(color)
            .add_component(Floor {});
    }
}

#[derive(Debug)]
struct DetectedCollision {
    other_entity: Entity,
}

struct ApplyGravityMutation {
    entity: Entity,
    dy: f32,
}

impl Mutation for ApplyGravityMutation {
    fn apply(&self, ecs: &mut ECS) {
        let velocity = ecs.write::<Velocity>(&self.entity).unwrap();
        velocity.dy += self.dy;
    }
}

struct ApplyMovementMutation {
    entity: Entity,
    dy: f32,
    dx: f32,
}

impl Mutation for ApplyMovementMutation {
    fn apply(&self, ecs: &mut ECS) {
        let transform = ecs.write::<TransformComponent>(&self.entity).unwrap();
        transform.change_pos(self.dx, self.dy);
    }
}

pub fn gravity_system(ecs: &ECS, dt: f32) -> Mutations {
    let mut mutations: Mutations = Vec::new();

    for (_, gravity, ent) in ecs.read2::<Velocity, HasGravity>() {
        if let None = ecs.read::<DetectedCollision>(&ent) {
            mutations.push(Box::new(ApplyGravityMutation {
                entity: ent,
                dy: gravity.gravity * dt,
            }));
        }
    }
    return mutations;
}

pub fn apply_velocity_system(ecs: &ECS, dt: f32) -> Mutations {
    let mut mutations: Mutations = Vec::new();
    for (vel, ent) in ecs.read1::<Velocity>() {
        mutations.push(Box::new(ApplyMovementMutation {
            entity: ent,
            dy: vel.dy * dt,
            dx: vel.dx * dt,
        }));
    }
    return mutations;
}

fn collides(
    c1: &Collider,
    t1: &TransformComponent,
    c2: &Collider,
    t2: &TransformComponent,
) -> bool {
    let lc1 = t1.position[0] - c1.width / 2.;
    let rc1 = lc1 + c1.width;
    let lc2 = t2.position[0] - c2.width / 2.;
    let rc2 = lc2 + c2.width;

    let bc1 = t1.position[1] - c1.height / 2.;
    let tc1 = bc1 + c1.height;
    let bc2 = t2.position[1] - c2.height / 2.;
    let tc2 = bc2 + c2.height;

    return rc1 >= lc2 && lc1 <= rc2 && bc1 <= tc2 && tc1 >= bc2;
}

struct CollisionDetectedMutation {
    e1: Entity,
    e2: Entity,
}

impl Mutation for CollisionDetectedMutation {
    fn apply(&self, ecs: &mut ECS) {
        ecs.add_component(
            &self.e1,
            DetectedCollision {
                other_entity: self.e2,
            },
        );
        ecs.add_component(
            &self.e2,
            DetectedCollision {
                other_entity: self.e1,
            },
        );
    }
}

pub fn collision_detection_system(ecs: &ECS, _dt: f32) -> Mutations {
    let colliders = ecs.read2::<Collider, TransformComponent>();
    let mut mutations: Mutations = Vec::new();

    for i in 0..(colliders.len() - 1) {
        for j in (i + 1)..(colliders.len()) {
            let c1 = colliders[i];
            let c2 = colliders[j];

            if collides(c1.0, c1.1, c2.0, c2.1) {
                mutations.push(Box::new(CollisionDetectedMutation { e1: c1.2, e2: c2.2 }));
            }
        }
    }

    return mutations;
}

struct CollisionCleanupMutation {
    e: Entity,
}

impl Mutation for CollisionCleanupMutation {
    fn apply(&self, ecs: &mut ECS) {
        ecs.remove_component::<DetectedCollision>(&self.e);
    }
}

pub fn collision_cleanup_system(ecs: &ECS, _dt: f32) -> Mutations {
    let collisions = ecs.read1::<DetectedCollision>();
    let mut mutations: Mutations = Vec::new();
    for collision in collisions {
        mutations.push(Box::new(CollisionCleanupMutation { e: collision.1 }));
    }

    return mutations;
}

struct JumpMutation {
    entity: Entity,
}

impl Mutation for JumpMutation {
    fn apply(&self, ecs: &mut ECS) {
        let velocity = ecs.write::<Velocity>(&self.entity).unwrap();
        velocity.dy = 5.;
    }
}

pub fn jump_system(ecs: &ECS, _dt: f32) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let mut mutations: Mutations = Vec::new();
    for (_, ent) in ecs.read1::<Bird>() {
        if input.keyboard.special_key_pressed(SpecialKeys::Space) {
            mutations.push(Box::new(JumpMutation { entity: ent }));
        }
    }
    return mutations;
}

struct FloorCollisionMutation {
    entity: Entity,
}

impl Mutation for FloorCollisionMutation {
    fn apply(&self, ecs: &mut ECS) {
        if let Some(v) = ecs.write::<Velocity>(&self.entity) {
            if v.dy < 0. {
                v.dy = 0.;
            }
        }
    }
}
pub fn ground_collision_system(ecs: &ECS, _dt: f32) -> Mutations {
    let mut mutations: Mutations = Vec::new();

    for (_, _, collision, ent) in ecs.read3::<Bird, Velocity, DetectedCollision>() {
        for (_, floor_ent) in ecs.read1::<Floor>() {
            if collision.other_entity == floor_ent {
                mutations.push(Box::new(FloorCollisionMutation { entity: ent }))
            }
        }
    }

    return mutations;
}

#[derive(Debug)]
struct PipeSpawner {
    last_time: SystemTime,
    generated: VecDeque<Entity>,
}

impl PipeSpawner {
    pub fn create(ecs: &mut ECS) {
        ecs.build_entity().add_component(PipeSpawner {
            last_time: SystemTime::now(),
            generated: VecDeque::new(),
        });
    }
}

#[derive(Debug)]
struct Pipe {}

impl Pipe {
    pub fn create(ecs: &mut ECS) -> Entity {
        let color = ColorBoxComponent::new([0.5, 0.5, 0.5]);
        let velocity = Velocity { dx: -3., dy: 0. };
        ecs.build_entity()
            .add_component(velocity)
            .add_component(Self::generate_transform())
            .add_component(color)
            .add_component(Pipe {})
            .entity()
    }

    pub fn update_transform(ecs: &mut ECS, entity: Entity) {
        ecs.remove_component::<TransformComponent>(&entity);
        ecs.add_component::<TransformComponent>(&entity, Self::generate_transform());
    }

    pub fn generate_transform() -> TransformComponent {
        let variance = (rand::random::<f32>() - 0.5) * 0.2;
        let y = if rand::random() {
            2. + variance
        } else {
            -2. + variance
        };
        TransformComponent::builder()
            .position(2., y)
            .scale(2.0, 4.0)
            .build()
    }
}

struct PipeSpawnMutation {
    current_time: SystemTime,
    spawner: Entity,
}

impl Mutation for PipeSpawnMutation {
    fn apply(&self, ecs: &mut ECS) {
        if let Some(spawner) = ecs.read::<PipeSpawner>(&self.spawner) {
            if spawner.generated.len() < 6 {
                let new = Pipe::create(ecs);
                if let Some(spawner) = ecs.write::<PipeSpawner>(&self.spawner) {
                    spawner.last_time = self.current_time;
                    spawner.generated.push_front(new);
                }
            } else {
                if let Some(&reused_entity) = spawner.generated.back() {
                    Pipe::update_transform(ecs, reused_entity);
                    if let Some(spawner) = ecs.write::<PipeSpawner>(&self.spawner) {
                        spawner.last_time = self.current_time;
                        spawner.generated.pop_back();
                        spawner.generated.push_front(reused_entity);
                    }
                }
            }
        }
    }
}

pub fn pipe_spawn_system(ecs: &ECS, _dt: f32) -> Mutations {
    let now = SystemTime::now();

    let mut mutations: Mutations = Vec::new();

    for (spawner, e) in ecs.read1::<PipeSpawner>() {
        if let Ok(t) = now.duration_since(spawner.last_time) {
            if t.as_millis() > 1500 {
                mutations.push(Box::new(PipeSpawnMutation {
                    current_time: now,
                    spawner: e,
                }));
            }
        }
    }

    return mutations;
}

struct AnimationMutation {
    anim: Entity,
    new_time: f32,
    bump_spritesheet: bool,
}

impl Mutation for AnimationMutation {
    fn apply(&self, ecs: &mut ECS) {
        if let Some(anim) = ecs.write::<Animation>(&self.anim) {
            anim.countdown = self.new_time;
        }
        if self.bump_spritesheet {
            if let Some(spritesheet) = ecs.write::<SpriteSheetComponent>(&self.anim) {
                spritesheet.next();
            }
        }

    }
}

pub fn animation_system(ecs: &ECS, dt:  f32) -> Mutations {
    let mut mutations: Mutations = Vec::new();
    for (anim, _, e) in ecs.read2::<Animation, SpriteSheetComponent>() {
        let new_time = anim.countdown + (dt * 1000.);
        mutations.push(Box::new(AnimationMutation { anim: e, new_time: new_time % anim.frame_duration_ms, bump_spritesheet: new_time > anim.frame_duration_ms }));
    }
    return mutations
}

pub struct GameOverScene {}

impl GameScene for GameOverScene {
    fn on_scene_start(&self, ecs: &mut ECS) {
        todo!()
    }

    fn on_scene_stop(&self, ecs: &mut ECS) {
        todo!()
    }

    fn render_scene(&self, ecs: &ECS, scene: &Scene) {
        todo!()
    }

    fn next_scene(&self, ecs: &ECS) -> Option<GameSceneId> {
        todo!()
    }

    fn active_layers(&self, ecs: &ECS) -> Vec<RenderLayer> {
        todo!()
    }
}
pub struct MainGameScene {}

impl GameScene for MainGameScene {
    fn on_scene_start(&self, ecs: &mut ECS) {
        ecs.build_entity()
            .add_component(OrthographicCamera::new(8.))
            .add_component(TransformComponent::builder().build());

        Bird::create(ecs);
        Floor::create(ecs);
        PipeSpawner::create(ecs);

        ecs.add_system(Box::new(apply_velocity_system));
        ecs.add_system(Box::new(jump_system));
        ecs.add_system(Box::new(collision_detection_system));
        ecs.add_system(Box::new(ground_collision_system));
        ecs.add_system(Box::new(gravity_system));
        ecs.add_system(Box::new(collision_cleanup_system));
        ecs.add_system(Box::new(pipe_spawn_system));
        ecs.add_system(Box::new(animation_system));
        // Create Ground
        // Setup Pipe Creation System
        // Setup Gravity System
        // Setup Movement System
        //
    }

    fn on_scene_stop(&self, ecs: &mut ECS) {}

    fn render_scene(&self, ecs: &ECS, scene: &Scene) {
        // Draw background
        // Draw ground
        // Draw bird
        // Draw pipes
        for (location, color_box, _) in ecs.read2::<TransformComponent, ColorBoxComponent>() {
            color_box.draw(&scene, location);
        }

        for (location, spritesheet, _) in ecs.read2::<TransformComponent, SpriteSheetComponent>() {
            spritesheet.draw(&scene, location);
        }
    }

    fn next_scene(&self, ecs: &ECS) -> Option<u32> {
        return None;
    }

    fn active_layers(&self, _ecs: &ECS) -> Vec<RenderLayer> {
        return vec![String::from("Editor")];
    }
}
struct Game {}

impl Game {
    pub fn new() -> Self {
        Game {}
    }
}

impl GameLogic for Game {
    fn window_props(&self) -> WindowProps {
        WindowProps {
            width: 900.0,
            height: 900.0,
            title: "Flappy Cheese".to_string(),
            target_ms_per_frame: 30.0,
        }
    }

    fn register_events(&self, ecs: &mut ECS) {}

    fn migrate_events(&self, ecs: &mut ECS) {}

    fn game_scenes(&self) -> HashMap<GameSceneId, Box<dyn GameScene>> {
        let mut res: HashMap<GameSceneId, Box<dyn GameScene>> = HashMap::new();
        res.insert(START_MENU_SCENE, Box::new(MainGameScene {}));
        return res;
    }

    fn initial_game_scene(&self) -> u32 {
        return START_MENU_SCENE;
    }

    fn setup(&mut self, ecs: &mut ECS) {}
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut gouda = Gouda::new(Game::new());
    gouda.run();
}
