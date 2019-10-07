use gouda::{Gouda, GameLogic};
use gouda::ecs::{ECS, Mutations, Mutation, Entity};
use gouda::rendering::{Scene, QuadDrawable, Renderer, VertexBuffer};
use std::rc::Rc;
use std::ops::Deref;
use gouda::input::{LetterKeys, GameInput};
use gouda::math::Mat4x4;

extern crate rand;
use rand::Rng;
use crate::tilemap::{Tile, Tilemap};

mod tilemap;

#[derive(Debug)]
struct Camera {
    pub projection_matrix: Mat4x4,
    pub projection_buffer: VertexBuffer,
    center: [f32; 2],
    width: f32,
    aspect: f32,
}

impl Camera {
    pub fn create(ecs: &mut ECS)  {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let mut camera = Camera {
            projection_matrix: Mat4x4::identity(),
            projection_buffer: VertexBuffer::new(renderer, 0, Mat4x4::identity().to_vec()),
            center: [0., 0.],
            width: 11.,
            aspect: 1.,
        };
        camera.update_projection_matrix();
        ecs.build_entity().add(camera);
    }

    pub fn change_width(&mut self, dw: f32) {
        self.width += dw;
        self.update_projection_matrix();
    }

    pub fn change_pos(&mut self, dx: f32, dy: f32) {
        self.center[0] += dx;
        self.center[1] += dy;

        self.update_projection_matrix();
    }

    fn screen_space_to_world_space(&self, screen_x: f32, screen_y: f32) -> [f32; 2] {
        let height = self.width * self.aspect;
        let right = self.center[0] + self.width/2.;
        let left = self.center[0] - self.width/2.;
        let top = self.center[1] + height/2.;
        let bottom = self.center[1] - height/2.;

        let world_x = (screen_x + (right + left)/(right - left))  * (right - left) / 2.;
        let world_y = (screen_y + (top + bottom)/(top - bottom))  * (top - bottom) / 2.;
        return [world_x, world_y];
    }

    fn update_projection_matrix(&mut self) {
        let height = self.width * self.aspect;

        let right = self.center[0] + self.width/2.;
        let left = self.center[0] - self.width/2.;
        let top = self.center[1] + height/2.;
        let bottom = self.center[1] - height/2.;

        let projection = Mat4x4::new(
            [
                [2./(right - left), 0., 0., -1. * (right + left)/(right - left)],
                [0., 2./(top - bottom), 0., -1. * (top + bottom)/(top - bottom)],
                [0., 0., 1., 1.],
                [0., 0., 0., 1.],
            ]
        );


        self.projection_buffer.update_data(projection.to_vec());
        self.projection_matrix = projection;
    }
}

#[derive(Debug)]
struct Hearth {
    drawable: QuadDrawable,
}

#[derive(Debug)]
struct Player {
    drawable: QuadDrawable,
    x: f32,
    y: f32,
}

impl Player {
    pub fn create(ecs: &mut ECS) {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let player_drawable = QuadDrawable::new(false, renderer, [0.7, 0.7, 0.7], [-4., -1., 0.], [0.3, 0.3, 1.]);
        ecs.build_entity().add(Player {drawable: player_drawable, x: -4., y: -1.});
    }

    pub fn set_pos(&mut self, new_x: f32, new_y: f32) {
        self.x = new_x;
        self.y = new_y;
        self.drawable.translate([self.x, self.y, 0.], [0.3, 0.3, 1.]);
    }
}

#[derive(Debug)]
struct Monster {
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

    pub fn set_pos(&mut self, new_x: f32, new_y: f32) {
        self.x = new_x;
        self.y = new_y;
        self.drawable.translate([self.x, self.y, 0.], [0.3, 0.3, 1.]);
    }
}

#[derive(Debug)]
struct Spawner {
    max_time: f32,
    current_time: f32,
}

#[derive(Debug)]
struct GuiElement {
    drawable: QuadDrawable,
}

struct Cursor {
    drawable: QuadDrawable,
}

struct MoveMutation {
    entity: Entity,
    dx: f32,
    dy: f32,
}

impl Mutation for MoveMutation {
    fn apply(&self, ecs: &mut ECS) {
        let player = ecs.write::<Player>(&self.entity).unwrap();
        player.set_pos(player.x + self.dx, player.y + self.dy);
    }
}

struct ZoomMutation {
    entity: Entity,
    dw: f32,
}

impl Mutation for ZoomMutation {
    fn apply(&self, ecs: &mut ECS) {
        let camera = ecs.write::<Camera>(&self.entity).unwrap();
        camera.change_width(self.dw);
    }
}

fn camera_scroll_system(ecs: &ECS) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let mut mutations: Mutations = Vec::new();
    for (c, ent) in ecs.read1::<Camera>() {
        if input.keyboard.letter_pressed(LetterKeys::U) {
            mutations.push(Box::new(ZoomMutation {entity: ent, dw: -2.}))
        } else if input.keyboard.letter_pressed(LetterKeys::I) {
            mutations.push(Box::new(ZoomMutation {entity: ent, dw: 2.}))
        }
    }
    return mutations;
}

fn player_move_system(ecs: &ECS) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let mut mutations: Mutations = Vec::new();
    for (p, ent) in ecs.read1::<Player>() {
        if input.keyboard.letter_pressed(LetterKeys::A) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: -1., dy: 0.}))
        } else if input.keyboard.letter_pressed(LetterKeys::D) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: 1., dy: 0.}))
        }
        if input.keyboard.letter_pressed(LetterKeys::W) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: 0., dy: 1.}))
        } else if input.keyboard.letter_pressed(LetterKeys::S) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: 0., dy: -1.}))
        }
    }
    return mutations;
}

struct ClickMutation {
    pub x: f32,
    pub y: f32,
}

impl Mutation for ClickMutation {
    fn apply(&self, ecs: &mut ECS) {
        Monster::create(ecs, self.x, self.y);
    }
}

fn mouse_cursor_system(ecs: &ECS) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let mut mutations: Mutations = Vec::new();
    if input.mouse.buttons[0].ended_down && input.mouse.buttons[0].half_transition_count == 1 {
        for (camera, entity) in ecs.read1::<Camera>() {
            let screen_x = input.mouse.x as f32 / 450. - 1.;
            let screen_y = input.mouse.y as f32 / 450. - 1.;
            let pos = camera.screen_space_to_world_space(screen_x, -1. * screen_y);
            mutations.push(Box::new(ClickMutation {
                x: (pos[0] + 0.5).floor(),
                y: (pos[1] + 0.5).floor(),
            }))
        }
    }
    return mutations;
}

struct SpawnMutation {

}

impl Mutation for SpawnMutation {
    fn apply(&self, ecs: &mut ECS) {

        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-5, 5) as f32;
        let y = rng.gen_range(-5, 5) as f32;

        Monster::create(ecs, x, y);
    }
}

struct SpawnTimerMutation {
    spawner: Entity,
    new_time: f32,
}

impl Mutation for SpawnTimerMutation {
    fn apply(&self, ecs: &mut ECS) {
        let mut spawner = ecs.write::<Spawner>(&self.spawner).unwrap();
        spawner.current_time = self.new_time;
    }
}

fn monster_spawn_system(ecs: &ECS) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let dt = input.seconds_to_advance_over_update;
    let mut mutations: Mutations = Vec::new();
    for (spawner, spawner_entity) in ecs.read1::<Spawner>() {
        if spawner.current_time - dt <= 0. {
            mutations.push(Box::new(SpawnMutation {}));
            mutations.push(Box::new(SpawnTimerMutation {spawner: spawner_entity, new_time: spawner.max_time - (dt - spawner.current_time)}));
        } else {
            mutations.push(Box::new(SpawnTimerMutation {spawner: spawner_entity, new_time: spawner.current_time - dt}));
        }
    }

    return mutations;
}

struct Game {}

impl Game {
    pub fn new() -> Self { Game {} }
}

impl GameLogic for Game {
    fn register_components(&self, ecs: &mut ECS) {
        ecs.register_component_type::<Tile>();
        ecs.register_component_type::<Player>();
        ecs.register_component_type::<Monster>();
        ecs.register_component_type::<GuiElement>();
        ecs.register_component_type::<Camera>();
        ecs.register_component_type::<Spawner>();
    }

    fn register_systems(&self, ecs: &mut ECS) {
        ecs.add_system(Box::new(player_move_system));
        ecs.add_system(Box::new(mouse_cursor_system));
        ecs.add_system(Box::new(monster_spawn_system));
        ecs.add_system(Box::new(camera_scroll_system));
    }

    fn setup(&mut self, ecs: &mut ECS) {
        let tilemap = Tilemap::create(ecs);
        ecs.add_res(tilemap);

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let cursor = Cursor {
            drawable: QuadDrawable::new(false, renderer, [0., 0., 0.8], [0., 0., 0.], [0.4, 0.4, 0.4]),
        };
        ecs.add_res(cursor);

        Player::create(ecs);
        Camera::create(ecs);

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let bottom = QuadDrawable::new(true, renderer, [0.1, 0.1, 0.1], [0., -0.815, 0.], [1., 0.18, 1.]);
        ecs.build_entity().add(GuiElement {drawable: bottom});

        ecs.build_entity().add(Spawner {max_time: 5., current_time: 5.});

    }

    fn draw_scene(&self, ecs: &ECS, scene: &Scene) {
        let input = ecs.read_res::<GameInput>();
        let screen_x = input.mouse.x as f32 / 450. - 1.;
        let screen_y = input.mouse.y as f32 / 450. - 1.;
        let cursor = ecs.read_res::<Cursor>();

        for (camera, entity) in ecs.read1::<Camera>() {
            let pos = camera.screen_space_to_world_space(screen_x, -1. * screen_y);
            let pos = [(pos[0] + 0.5).floor(), (pos[1] + 0.5).floor()];
            for (tile, e) in ecs.read1::<Tile>() {
                tile.draw(&scene, &camera);
                if tile.x == pos[0] as i32 && tile.y == pos[1] as i32 {
                    cursor.drawable.translate([pos[0], pos[1], 0.], [0.4, 0.4, 0.4]);
                    cursor.drawable.draw_with_projection(&scene, &camera.projection_buffer);
                }
            }

            for (monster, e) in ecs.read1::<Monster>() {
                monster.drawable.draw_with_projection(&scene, &camera.projection_buffer);
            }

            for (player, e) in ecs.read1::<Player>() {
                player.drawable.draw_with_projection(&scene, &camera.projection_buffer);
            }

            for (gui, e) in ecs.read1::<GuiElement>() {
                gui.drawable.draw(&scene);
            }
        }
    }
}

fn main() {
    let mut gouda = Gouda::new(Game::new());
    gouda.run();
}
