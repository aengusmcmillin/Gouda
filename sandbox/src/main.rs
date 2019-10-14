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
use crate::player::{Player, player_move_system};
use crate::cursor::Cursor;
use crate::camera::Camera;
use gouda::font::{Font, TextDrawable};

mod tilemap;
mod player;
mod turret;
mod cursor;
mod camera;


#[derive(Debug)]
struct Hearth {
    drawable: QuadDrawable,
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
struct TextElement {
    drawable: TextDrawable,
}

#[derive(Debug)]
struct GuiElement {
    drawable: QuadDrawable,
}

struct ZoomMutation {
    dw: f32,
}

impl Mutation for ZoomMutation {
    fn apply(&self, ecs: &mut ECS) {
        ecs.write_res::<Camera>().change_width(self.dw);
    }
}

fn camera_scroll_system(ecs: &ECS) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let mut mutations: Mutations = Vec::new();
    if input.keyboard.letter_pressed(LetterKeys::U) {
        mutations.push(Box::new(ZoomMutation {dw: -2.}))
    } else if input.keyboard.letter_pressed(LetterKeys::I) {
        mutations.push(Box::new(ZoomMutation {dw: 2.}))
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

fn mouse_click_system(ecs: &ECS) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let mut mutations: Mutations = Vec::new();
    let camera = ecs.read_res::<Camera>();
    if input.mouse.buttons[0].ended_down && input.mouse.buttons[0].half_transition_count == 1 {
        let screen_x = input.mouse.x as f32 / 450. - 1.;
        let screen_y = input.mouse.y as f32 / 450. - 1.;
        let pos = camera.screen_space_to_world_space(screen_x, -1. * screen_y);
        mutations.push(Box::new(ClickMutation {
            x: (pos[0] + 0.5).floor(),
            y: (pos[1] + 0.5).floor(),
        }))
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
        ecs.register_component_type::<TextElement>();
        ecs.register_component_type::<Spawner>();
    }

    fn register_systems(&self, ecs: &mut ECS) {
        ecs.add_system(Box::new(player_move_system));
        ecs.add_system(Box::new(mouse_click_system));
        ecs.add_system(Box::new(monster_spawn_system));

        // Spawn waves of monsters
        // Handle pathfinding
        // ecs.add_system(Box::new(pathfinding_system));
        // Deal with collisions of monsters with hearth
        // Turrets aiming and attacking
    }

    fn setup(&mut self, ecs: &mut ECS) {
        let renderer = ecs.read_res::<Rc<Renderer>>();

        let tilemap = Tilemap::create(ecs);
        ecs.add_res(tilemap);

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let cursor = Cursor::new(renderer);
        ecs.add_res(cursor);

        Player::create(ecs);
        Camera::create(ecs);

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let quad = QuadDrawable::new(true, renderer, [1., 1., 1.], [-1., -0.825, 0.], [2., 0.175, 0.]);
        ecs.build_entity().add(GuiElement { drawable: quad, });

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let font = Font::new(renderer, "bitmap/segoe.fnt", "bitmap/segoe.png");
        let text = TextDrawable::new(renderer, [-0.95, -0.68], Rc::new(font), "Test String #1: Segoe".to_string(), 14.);
        ecs.build_entity().add(TextElement { drawable: text, });

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let font = Font::new(renderer, "bitmap/arial.fnt", "bitmap/arial.png");
        let text = TextDrawable::new(renderer, [-0.95, -0.75], Rc::new(font), "Test String #2: Arial".to_string(), 14.);
        ecs.build_entity().add(TextElement { drawable: text, });

        ecs.build_entity().add(Spawner {max_time: 5., current_time: 5.});
    }

    fn draw_scene(&self, ecs: &ECS, scene: &Scene) {
        let input = ecs.read_res::<GameInput>();
        let screen_x = input.mouse.x as f32 / 450. - 1.;
        let screen_y = input.mouse.y as f32 / 450. - 1.;
        let cursor = ecs.read_res::<Cursor>();

        let camera = ecs.read_res::<Camera>();
        let pos = camera.screen_space_to_world_space(screen_x, -1. * screen_y);
        let pos = [(pos[0] + 0.5).floor(), (pos[1] + 0.5).floor()];
        for (tile, e) in ecs.read1::<Tile>() {
            tile.draw(&scene, &camera);
            if tile.x == pos[0] as i32 && tile.y == pos[1] as i32 {
                cursor.draw_at_pos(&scene, &camera, [pos[0], pos[1], 0.]);
            }
        }

        for (monster, e) in ecs.read1::<Monster>() {
            monster.drawable.draw_with_projection(&scene, &camera.projection_buffer);
        }

        for (player, e) in ecs.read1::<Player>() {
            player.draw(&scene, &camera);
        }

        for (gui, e) in ecs.read1::<GuiElement>() {
            gui.drawable.draw(&scene);
        }

        for (text, e) in ecs.read1::<TextElement>() {
            text.drawable.draw(&scene);
        }
    }
}

fn main() {
    let mut gouda = Gouda::new(Game::new());
    gouda.run();
}
