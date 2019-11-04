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

struct ShowMenuMutation {

}

impl Mutation for ShowMenuMutation {
    fn apply(&self, ecs: &mut ECS) {
        let menu = ecs.write_res::<MenuScreen>();
        let was_active = menu.active;
        let e = menu.entity.clone();
        menu.active = !menu.active;
        if was_active {
            ecs.remove_component::<ActiveGui>(&e);
        } else {
            ecs.add_component(&e, ActiveGui {});
        }
    }
}

fn menu_show_system(ecs: &ECS) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let mut mutations: Mutations = Vec::new();
    if input.keyboard.letter_pressed(LetterKeys::B) {
        mutations.push(Box::new(ShowMenuMutation {}));
    }
    return mutations;
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
