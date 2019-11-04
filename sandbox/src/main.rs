use gouda::{Gouda, GameLogic, GameState};
use gouda::ecs::{ECS, Mutations, Mutation, Entity, GameStateId};
use gouda::rendering::{
    Scene, drawable::QuadDrawable, Renderer, buffers::VertexBuffer};
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
use crate::spawners::{WaveSpawner, wave_spawner_system, WaveSpec, MonsterSpec, MonsterType};
use crate::monster::{Monster, monster_move_system};
use gouda::gui::{GuiComponent, ActiveGui};
use gouda::gui::constraints::GuiConstraints;
use gouda::types::Color;
use gouda::gui::constraints::Constraint::RelativeConstraint;
use gouda::input::AnyKey::Letter;
use crate::gui::GameGui;
use crate::main_menu::{MenuScreen, menu_show_system, MainMenu};
use std::collections::HashMap;

mod tilemap;
mod player;
mod building;
mod cursor;
mod camera;
mod game_stage;
mod spawners;
mod monster;
mod pathfinding;
mod villager;
mod hearth;
mod gui;
mod main_menu;

#[derive(Debug)]
struct Pos {
    pub x: f32,
    pub y: f32,
}

struct ClickMutation {
    pub x: f32,
    pub y: f32,
}

impl Mutation for ClickMutation {
    fn apply(&self, ecs: &mut ECS) {
//        Monster::create(ecs, self.x, self.y);
        let tilemap = ecs.read_res::<Tilemap>();
        let t = tilemap.tile_at_pos((self.x + 5.) as usize, (self.y + 3.) as usize);
        println!("{} {}", self.x + 5., self.y + 4.);


        let mut selected_players = vec![];
        let mut deselected_players = vec![];
        for (player, e) in ecs.read1::<Player>() {
            if player.current_tile == t {
                selected_players.push(e);
            } else {
                deselected_players.push(e);
            }
        }
        for e in selected_players {
            ecs.write::<Player>(&e).unwrap().set_selected(true);
        }
        for e in deselected_players {
            ecs.write::<Player>(&e).unwrap().set_selected(false);
        }
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

fn register_core_systems(ecs: &mut ECS) {
    ecs.add_system(Box::new(player_move_system));
    ecs.add_system(Box::new(menu_show_system));
}

fn draw_everything(ecs: &ECS, scene: &Scene) {
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
            let renderer = ecs.read_res::<Rc<Renderer>>();
            cursor.draw_at_pos(&renderer, &scene, &camera, [pos[0], pos[1], 0.]);
        }
    }
    for (monster, e) in ecs.read1::<Monster>() {
        monster.draw(&scene, &camera);
    }

    for (player, e) in ecs.read1::<Player>() {
        player.draw(&scene, &camera);
    }

    for (gui, _active, e) in ecs.read2::<GuiComponent, ActiveGui>() {
        gui.render(&scene);
    }
}

pub const MAIN_GAME_STATE: GameStateId = 0;

pub struct MainGameState {
}

impl GameState for MainGameState {
    fn on_state_start(&self, ecs: &mut ECS) {
        register_core_systems(ecs);
        ecs.add_system(Box::new(wave_spawner_system));
        ecs.add_system(Box::new(monster_move_system));
        ecs.add_system(Box::new(mouse_click_system));
    }

    fn on_state_stop(&self, ecs: &mut ECS) {
    }

    fn render_state(&self, ecs: &ECS, scene: &Scene) {
        draw_everything(ecs, scene);
    }

    fn next_state(&self, ecs: &ECS) -> Option<u32> {
        let input = ecs.read_res::<GameInput>();
        if input.keyboard.letter_pressed(LetterKeys::B) {
            return Some(MAIN_MENU_GAME_STATE);
        }
        return None;
    }
}

pub const DAY_GAME_STATE: GameStateId = 10;

pub struct DayGameState {

}

impl GameState for DayGameState {
    fn on_state_start(&self, ecs: &mut ECS) {
        unimplemented!()
    }

    fn on_state_stop(&self, ecs: &mut ECS) {
        unimplemented!()
    }

    fn render_state(&self, ecs: &ECS, scene: &Scene) {
        unimplemented!()
    }

    fn next_state(&self, ecs: &ECS) -> Option<u32> {
        unimplemented!()
    }
}

pub const NIGHT_GAME_STATE: GameStateId = 11;

pub struct NightGameState {

}

impl GameState for NightGameState {
    fn on_state_start(&self, ecs: &mut ECS) {
        unimplemented!()
    }

    fn on_state_stop(&self, ecs: &mut ECS) {
        unimplemented!()
    }

    fn render_state(&self, ecs: &ECS, scene: &Scene) {
        unimplemented!()
    }

    fn next_state(&self, ecs: &ECS) -> Option<u32> {
        unimplemented!()
    }
}


pub const MAIN_MENU_GAME_STATE: GameStateId = 1;

pub struct MainMenuGameState {

}

impl GameState for MainMenuGameState {
    fn on_state_start(&self, ecs: &mut ECS) {
        register_core_systems(ecs);
    }

    fn on_state_stop(&self, ecs: &mut ECS) {
    }

    fn render_state(&self, ecs: &ECS, scene: &Scene) {
        let menu = ecs.read_res::<MenuScreen>();
        let menugui = ecs.read::<GuiComponent>(&menu.entity);
        menugui.unwrap().render(&scene);
    }

    fn next_state(&self, ecs: &ECS) -> Option<u32> {
        let input = ecs.read_res::<GameInput>();
        if input.keyboard.letter_pressed(LetterKeys::B) {
            return Some(MAIN_GAME_STATE);
        }
        return None;
    }
}

struct Game {
}

impl Game {
    pub fn new() -> Self { Game {} }
}

impl GameLogic for Game {
    fn register_components(&self, ecs: &mut ECS) {
        ecs.register_component_type::<Tile>();
        ecs.register_component_type::<Player>();
        ecs.register_component_type::<Monster>();
        ecs.register_component_type::<WaveSpawner>();
        ecs.register_component_type::<Pos>();
        ecs.register_component_type::<GuiComponent>();
        ecs.register_component_type::<ActiveGui>();
    }

    fn game_states(&self) -> HashMap<GameStateId, Box<dyn GameState>> {
        let mut res: HashMap<GameStateId, Box<dyn GameState>> = HashMap::new();
        res.insert(MAIN_GAME_STATE, Box::new(MainGameState {}));
        res.insert(MAIN_MENU_GAME_STATE, Box::new(MainMenuGameState {}));
        return res;
    }

    fn initial_game_state(&self) -> u32 {
        return MAIN_GAME_STATE;
    }

    fn setup(&mut self, ecs: &mut ECS) {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let font = Font::new(renderer, "bitmap/segoe.fnt", "bitmap/segoe.png");
        ecs.add_res(Rc::new(font));

        Tilemap::create(ecs);
        Cursor::create(ecs);
        Player::create(ecs);
        Camera::create(ecs);

        GameGui::create(ecs);
        MainMenu::create(ecs);

        ecs.build_entity()
            .add(WaveSpawner::new(
                WaveSpec {
                    monsters: vec![MonsterSpec { monster_type: MonsterType::Wolf }; 20],
                }, 1.))
            .add(Pos {x: 3., y: 3.});
    }
}

fn main() {
    let mut gouda = Gouda::new(Game::new());
    gouda.run();
}
