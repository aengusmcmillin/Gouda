use gouda::ecs::{ECS, Entity, Mutations, Mutation};
use crate::tilemap::Tile;
use gouda::rendering::drawable::{TextureDrawable, QuadDrawable};
use gouda::rendering::{Renderer, Scene};
use gouda::rendering::texture::RenderableTexture;
use std::rc::Rc;
use gouda::png::PNG;
use crate::camera::Camera;
use gouda::input::GameInput;
use crate::monster::Monster;

#[derive(Debug)]
pub struct Turret {
    texture_drawable: TextureDrawable,
    fire_cooldown: f32,
    fire_timer: f32,
    x: f32,
    y: f32,
}

impl Turret {
    pub fn create(ecs: &mut ECS, tile: Entity) {
        let tile = ecs.read::<Tile>(&tile).unwrap();

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let texture = RenderableTexture::new_from_png(renderer, PNG::from_file("bitmap/turret.png").unwrap());
        let texture_drawable = TextureDrawable::new(false, renderer, texture, [tile.x as f32, tile.y as f32, 0.], [0.4, 0.4, 1.0], [0.; 3]);
        let turret = Turret {
            texture_drawable,
            fire_cooldown: 4.,
            fire_timer: 4.,
            x: tile.x as f32,
            y: tile.y as f32,
        };
        ecs.build_entity().add(turret);
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera) {
        self.texture_drawable.draw_with_projection(scene, &camera.projection_buffer);
    }
}

#[derive(Debug)]
pub struct Arrow {
    drawable: QuadDrawable,
    x: f32,
    y: f32,
    speed: f32,
}

impl Arrow {
    pub fn create(ecs: &mut ECS, x: f32, y: f32) {
        println!("Creating arrow");
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let quad = QuadDrawable::new(false, renderer, [1.0, 0.0, 0.0], [x, y, 0.], [0.1; 3], [0.; 3]);
        ecs.build_entity().add(Arrow {drawable: quad, x, y, speed: 3.});
    }

    pub fn set_position(&mut self, renderer: &Renderer, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.drawable.set_position(renderer, [x, y, 0.]);
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera) {
        self.drawable.draw_with_projection(scene, &camera.projection_buffer);
    }
}

struct ArrowCollisionMutation {
    arrow: Entity,
}

impl Mutation for ArrowCollisionMutation {
    fn apply(&self, ecs: &mut ECS) {
        ecs.remove_component::<Arrow>(&self.arrow);
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
        let renderer = ecs.read_res::<Rc<Renderer>>().clone();
        let arrow = ecs.write::<Arrow>(&self.arrow).unwrap();
        arrow.set_position(&renderer, arrow.x + self.dx, arrow.y + self.dy);
    }
}

pub fn arrow_move_system(ecs: &ECS) -> Mutations {
    let mut mutations: Mutations = vec![];
    let mut monster_positions: Vec<(Entity, f32, f32)> = vec![];
    for (monster, entity) in ecs.read1::<Monster>() {
        monster_positions.push((entity, monster.x, monster.y));
    }

    let dt = ecs.read_res::<GameInput>().seconds_to_advance_over_update;

    for (arrow, entity) in ecs.read1::<Arrow>() {
        let mut closest: Option<(f32, f32, f32)> = None;

        for (monster, x, y) in &monster_positions {
            let x_dist = arrow.x - *x;
            let y_dist = arrow.y - *y;
            let distsq = x_dist * x_dist + y_dist * y_dist;

            if let Some((_, _, closest_distsq)) = closest {
                if distsq < closest_distsq {
                    closest = Some((*x, *y, distsq));
                }
            } else {
                closest = Some((*x, *y, distsq));
            }
        }

        if let Some((x, y, distsq)) = closest {
            if distsq < 0.6 {
                mutations.push(Box::new(ArrowCollisionMutation {
                    arrow: entity,
                }));
            } else {
                let v = (x - arrow.x, y - arrow.y);
                mutations.push(Box::new(MoveArrowTowardsMutation {
                    arrow: entity,
                    dx: v.0 * dt / distsq.sqrt() * arrow.speed,
                    dy: v.1 * dt / distsq.sqrt() * arrow.speed,
                }))
            }
        } else {
            mutations.push(Box::new(ArrowCollisionMutation {
                arrow: entity,
            }));
        }
    }

    return mutations;
}


pub struct FireArrowMutation {
    pub turret: Entity,
}

impl Mutation for FireArrowMutation {
    fn apply(&self, ecs: &mut ECS) {
        let turret = ecs.write::<Turret>(&self.turret).unwrap();
        turret.fire_timer = turret.fire_cooldown;
        let (x, y) = (turret.x, turret.y);
        Arrow::create(ecs, x, y);
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

pub fn turret_attack_system(ecs: &ECS) -> Mutations {
    let mut mutations: Mutations = vec![];

    let input = ecs.read_res::<GameInput>();
    for (turret, e) in ecs.read1::<Turret>() {
        if turret.fire_timer - input.seconds_to_advance_over_update <= 0. {
            if ecs.get1::<Monster>().len() > 0 {
                mutations.push(Box::new(FireArrowMutation {turret: e}));
            }
        } else {
            mutations.push(Box::new(DecrTurretTimerMutation {turret: e, dt: input.seconds_to_advance_over_update}));
        }
    }

    return mutations;
}