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
use crate::monster::{Monster, monster_move_system, monster_damage_system};
use gouda::gui::{GuiComponent, ActiveGui};
use gouda::gui::constraints::GuiConstraints;
use gouda::types::Color;
use gouda::gui::constraints::Constraint::RelativeConstraint;
use gouda::input::AnyKey::Letter;
use crate::gui::{GameGui, game_gui_system};
use crate::main_menu::{MenuScreen, menu_show_system, MainMenu};
use std::collections::HashMap;
use gouda::window::WindowProps;
use gouda::mouse_capture::{MouseCaptureArea, MouseCaptureLayer, mouse_capture_system, ActiveCaptureLayer};
use crate::building::{Turret, turret_attack_system, Arrow, arrow_move_system, DamageDealt};

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

pub struct CreateTurretMutation {
    e: Entity,
}

impl Mutation for CreateTurretMutation {
    fn apply(&self, ecs: &mut ECS) {
        Turret::create(ecs, self.e);
    }
}

fn mouse_click_system(ecs: &ECS) -> Mutations {
    let mut mutations: Mutations = Vec::new();
    for (tile, mouse_capture, e) in ecs.read2::<Tile, MouseCaptureArea>() {
        if mouse_capture.clicked_buttons[0] {
            mutations.push(Box::new(CreateTurretMutation{e}));
        }
    }
    return mutations;
}

struct CursorSetPositionMutation {
    tile: Entity,
}

impl Mutation for CursorSetPositionMutation {
    fn apply(&self, ecs: &mut ECS) {
        let tile = ecs.read::<Tile>(&self.tile).unwrap();
        let (x, y) = (tile.x, tile.y);

        let renderer = ecs.read_res::<Rc<Renderer>>().clone();
        let cursor = ecs.write_res::<Cursor>();
        cursor.set_pos(&renderer, [x as f32, y as f32, 0.]);
    }
}

fn mouse_cursor_system(ecs: &ECS) -> Mutations {
    let mut mutations: Mutations = vec![];
    for (tile, mouse_capture, e) in ecs.read2::<Tile, MouseCaptureArea>() {
        if mouse_capture.is_hovered {
            mutations.push(Box::new(CursorSetPositionMutation {tile: e}))
        }
    }
    return mutations;
}

fn register_core_systems(ecs: &mut ECS) {
    ecs.add_system(Box::new(player_move_system));
    ecs.add_system(Box::new(menu_show_system));
    ecs.add_system(Box::new(mouse_capture_system));
    ecs.add_system(Box::new(game_gui_system));
}

fn draw_everything(ecs: &ECS, scene: &Scene) {
    let input = ecs.read_res::<GameInput>();
    let screen_x = input.mouse.x as f32 / 450. - 1.;
    let screen_y = input.mouse.y as f32 / 450. - 1.;
    let cursor = ecs.read_res::<Cursor>();

    let camera = ecs.read_res::<Camera>();
    let pos = camera.screen_space_to_world_space(screen_x, -1. * screen_y);
    let pos = [(pos[0] + 0.5).floor(), (pos[1] + 0.5).floor()];
    for (tile, mouse_capture, e) in ecs.read2::<Tile, MouseCaptureArea>() {
        tile.draw(&scene, &camera);
        if mouse_capture.is_hovered {
            let renderer = ecs.read_res::<Rc<Renderer>>();
            cursor.draw(&renderer, &scene, &camera);
        }
    }
    for (monster, e) in ecs.read1::<Monster>() {
        monster.draw(&scene, &camera);
    }

    for (player, e) in ecs.read1::<Player>() {
        player.draw(&scene, &camera);
    }

    for (turret, _) in ecs.read1::<Turret>() {
        turret.draw(&scene, &camera);
    }

    for (arrow, _) in ecs.read1::<Arrow>() {
        arrow.draw(&scene, &camera);
    }

    for (gui, _active, e) in ecs.read2::<GuiComponent, ActiveGui>() {
        gui.render(&ecs, &scene);
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
        ecs.add_system(Box::new(mouse_cursor_system));
        ecs.add_system(Box::new(turret_attack_system));
        ecs.add_system(Box::new(arrow_move_system));
        ecs.add_system(Box::new(monster_damage_system));
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
        draw_everything(ecs, scene);
        let menu = ecs.read_res::<MenuScreen>();
        let menugui = ecs.read::<GuiComponent>(&menu.entity);
        menugui.unwrap().render(&ecs, &scene);
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
    fn window_props(&self) -> WindowProps {
        WindowProps {
            width: 900.0,
            height: 900.0,
            title: "Hearth of Hestia".to_string(),
            target_ms_per_frame: 30.0,
        }
    }

    fn register_components(&self, ecs: &mut ECS) {
        ecs.register_component_type::<Tile>();
        ecs.register_component_type::<Player>();
        ecs.register_component_type::<Monster>();
        ecs.register_component_type::<WaveSpawner>();
        ecs.register_component_type::<Pos>();
        ecs.register_component_type::<GuiComponent>();
        ecs.register_component_type::<ActiveGui>();
        ecs.register_component_type::<MouseCaptureArea>();
        ecs.register_component_type::<MouseCaptureLayer>();
        ecs.register_component_type::<ActiveCaptureLayer>();
        ecs.register_component_type::<Turret>();
        ecs.register_component_type::<Arrow>();
        ecs.register_component_type::<DamageDealt>();
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
