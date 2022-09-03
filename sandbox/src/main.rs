use cgmath::Vector2;
use gouda::rendering::sprites::{SpriteComponent, ColorBoxComponent, SpriteSheetComponent};
use gouda::{Gouda, GameLogic, GameState, RenderLayer, QuitEvent};
use gouda::ecs::{ECS, Mutations, Mutation, Entity, GameStateId};
use gouda::rendering::{Scene, Renderer, drawable::ShapeDrawable};
use tree::create_tree;
use std::rc::Rc;
use gouda::input::{LetterKeys, GameInput};
use gouda::TransformComponent;
use gouda::camera::{Camera, OrthographicCamera};

extern crate rand;
use rand::{Rng, thread_rng};
use crate::tilemap::{Tile, Tilemap};
use crate::player::{Player, player_move_system};
use crate::cursor::Cursor;
use gouda::font::Font;
use crate::spawners::{WaveSpawner, wave_spawner_system, GameDay, generate_days};
use crate::monster::{Monster, monster_move_system, monster_damage_system};
use gouda::gui::{GuiComponent, ActiveGui, GuiText, GuiImage};
use crate::gui::{GameGui, game_gui_system, StageText, change_stage_text, GoldText, WoodText, StoneText};
use crate::main_menu::{MenuScreen, menu_mouse_system, SaveEvent, ResumeEvent, SettingsEvent};
use std::collections::HashMap;
use gouda::window::{WindowProps, WindowEvent};
use gouda::mouse_capture::{MouseCaptureArea, MouseCaptureLayer, mouse_capture_system, ActiveCaptureLayer};
use crate::building::{Turret, turret_attack_system, Arrow, arrow_move_system, DamageDealt};
use crate::start_menu::{StartMenu, START_MENU_GAME_STATE, StartMenuButtonId, StartMenuGameState, StartEvent};
use crate::supplies::Supplies;
use crate::tree::TreeComponent;

mod tree;
mod start_menu;
mod tilemap;
mod player;
mod building;
mod cursor;
mod game_stage;
mod spawners;
mod monster;
mod pathfinding;
mod villager;
mod hearth;
mod gui;
mod main_menu;
mod supplies;

pub struct CreateTurretMutation {
    tile_e: Entity,
}

impl Mutation for CreateTurretMutation {
    fn apply(&self, ecs: &mut ECS) {
        if ecs.write_res::<Supplies>().spend_supplies(0, 5, 0) {
            Turret::create(ecs, self.tile_e);

            ecs.write::<Tile>(&self.tile_e).unwrap().occupied = true;
        } else {

        }
    }
}

pub struct TurretSelectMutation {
    turret_e: Entity,
}

impl Mutation for TurretSelectMutation {
    fn apply(&self, ecs: &mut ECS) {
        let mut loc = *ecs.read::<TransformComponent>(&self.turret_e).unwrap();
        loc.scale = Vector2::new(3.0, 3.0);
        let range_sprite = SpriteComponent::new(ecs, "bitmap/range_indicator.png".to_string());
        let range_indicator = Some(ecs.build_entity().add(range_sprite).add(loc).entity());
        let turret = ecs.write::<Turret>(&self.turret_e).unwrap();
        turret.selected = true;
        turret.range_indicator = range_indicator;
    }
}

pub struct TurretDeselectMutation {
}

impl Mutation for TurretDeselectMutation {
    fn apply(&self, ecs: &mut ECS) {
        let turrets = ecs.get1::<Turret>();
        for turret in &turrets {
            let turret = ecs.write::<Turret>(&turret).unwrap();
            turret.selected = false;
            let indicator = turret.range_indicator;
            if let Some(e) = indicator {
                println!("Deletintg indicator");
                turret.range_indicator = None;
                ecs.delete_entity(&e);
            }
        }
    }
}

pub struct TreeHarvestMutation {
    tree: Entity,
}

impl Mutation for TreeHarvestMutation {
    fn apply(&self, ecs: &mut ECS) {
        let tree = ecs.write::<TreeComponent>(&self.tree).unwrap();
        let wood = tree.harvest();
        ecs.write_res::<Supplies>().add_wood(wood);
        ecs.delete_entity(&self.tree);
    }
}

fn mouse_click_system(ecs: &ECS, dt: f32) -> Mutations {
    let mut mutations: Mutations = Vec::new();
    for (tile, mouse_capture, tile_e) in ecs.read2::<Tile, MouseCaptureArea>() {
        if mouse_capture.clicked_buttons[0] {
            mutations.push(Box::new(TurretDeselectMutation{}));
            if !tile.occupied {
                mutations.push(Box::new(CreateTurretMutation{tile_e}));
            } else {
                for (turret, loc, e) in ecs.read2::<Turret, TransformComponent>() {
                    if loc.position.x == tile.x as f32 && loc.position.y == tile.y as f32 && !turret.selected {
                        mutations.push(Box::new(TurretSelectMutation{turret_e: e}));
                    }
                }

                for (_, location, e) in ecs.read2::<TreeComponent, TransformComponent>() {
                    if location.position.x == tile.x as f32 && location.position.y == tile.y as f32 {
                        mutations.push(Box::new(TreeHarvestMutation {tree: e}))
                    }
                }
            }
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

        let cursor = ecs.write_res::<Cursor>();
        cursor.set_pos([x as f32, y as f32, 0.]);
    }
}

struct CursorVisibilityMutation {
    visible: bool,
}

impl Mutation for CursorVisibilityMutation {
    fn apply(&self, ecs: &mut ECS) {
        ecs.write_res::<Cursor>().set_visible(self.visible);
    }
}

fn mouse_cursor_system(ecs: &ECS, dt: f32) -> Mutations {
    let mut mutations: Mutations = vec![];
    let mut any_hovered = false;
    for (_, mouse_capture, e) in ecs.read2::<Tile, MouseCaptureArea>() {
        if mouse_capture.is_hovered {
            mutations.push(Box::new(CursorSetPositionMutation {tile: e}));
            any_hovered = true;
        }
    }
    mutations.push(Box::new(CursorVisibilityMutation {visible: any_hovered}));
    return mutations;
}

fn register_core_systems(ecs: &mut ECS) {
    ecs.add_system(Box::new(player_move_system));
    ecs.add_system(Box::new(mouse_capture_system));
    ecs.add_system(Box::new(game_gui_system));
}

fn draw_everything(ecs: &ECS, scene: &Scene) {

    // for (location, sprite, _) in ecs.read2::<TransformComponent, SpriteComponent>() {
    //     sprite.draw(&scene, location);
    // }

    // for (location, color_box, _) in ecs.read2::<TransformComponent, ColorBoxComponent>() {
    //     color_box.draw(&scene, location);
    // }

    // for (location, spritesheet, _) in ecs.read2::<TransformComponent, SpriteSheetComponent>() {
    //     spritesheet.draw(&scene, location);
    // }

    // for (shape, transform, _) in ecs.read2::<ShapeDrawable, TransformComponent>() {
    //     scene.submit_shape_by_name(&shape.shader_name, &shape.shape_name, transform.transform_matrix(), shape.color);
    // }

    ecs.read_res::<Cursor>().draw(&scene);

    // for (player, _) in ecs.read1::<Player>() {
    //     player.draw(&scene);
    // }

    // for (gui, _active, _) in ecs.read2::<GuiComponent, ActiveGui>() {
    //     gui.render(&ecs, &scene);
    // }
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
        ecs.build_entity().add(OrthographicCamera::new(-1., 1., -1., 1.));
    }

    fn on_state_stop(&self, ecs: &mut ECS) {
        ecs.add_res(LastState(MAIN_GAME_STATE));
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

    fn active_layers(&self, ecs: &ECS) -> Vec<RenderLayer> {
        return vec![
            String::from("Tilemap"),
            String::from("Turrets"),
            String::from("Monsters"),
            String::from("Players"),
            String::from("GUI")
        ];
    }

    fn camera(&self, ecs: &ECS) -> Box<dyn Camera> {
        let cam = ecs.read1::<OrthographicCamera>()[0].0.clone();
        return Box::new(cam);
    }
}

pub struct StateTimer {
    pub countdown_s: f32,
}

pub struct ChangeState {}

pub struct StateCountdownMutation {
    dt: f32,
}

impl Mutation for StateCountdownMutation {
    fn apply(&self, ecs: &mut ECS) {
        ecs.write_res::<StateTimer>().countdown_s -= self.dt;
        change_stage_text(ecs, &(ecs.read_res::<StateTimer>().countdown_s.ceil().to_string()));
    }
}

fn day_state_countdown(_ecs: &ECS, dt: f32) -> Mutations {
    return vec![Box::new(StateCountdownMutation {dt: dt})];
}

fn next_day(ecs: &mut ECS) {
    change_stage_text(ecs, "Day");

    let game_day = ecs.write_res::<Vec<GameDay>>().remove(0);
    ecs.add_res(game_day);
    ecs.write_res::<StateTimer>().countdown_s = ecs.read_res::<GameDay>().day_length;


    let tilemap = ecs.read_res::<Tilemap>();
    let borders: Vec<(f32, f32)> = tilemap.borders().iter().map(|border| {
        tilemap.pos_of_tile(*border)
    }).collect();

    let waves = &ecs.read_res::<GameDay>().waves.clone();
    for wave in waves {
        let border_index = thread_rng().gen_range(0, borders.len());
        let border = borders.get(border_index).unwrap();
        WaveSpawner::create(ecs, wave.wave.clone(),  border.0, border.1, 1.);
    }

    let tilemap = ecs.read_res::<Tilemap>();
    create_tree(ecs, tilemap.tile_at_pos(2, 4));
    let tilemap = ecs.read_res::<Tilemap>();
    create_tree(ecs, tilemap.tile_at_pos(8, 2));
}

fn next_night(ecs: &mut ECS) {
    change_stage_text(ecs, "Night");

    ecs.write_res::<StateTimer>().countdown_s = ecs.read_res::<GameDay>().night_length;
}

pub const DAY_GAME_STATE: GameStateId = 10;

pub struct DayGameState {
}

impl GameState for DayGameState {
    fn on_state_start(&self, ecs: &mut ECS) {
        register_core_systems(ecs);
        ecs.add_system(Box::new(mouse_click_system));
        ecs.add_system(Box::new(mouse_cursor_system));
        ecs.add_system(Box::new(day_state_countdown));
        ecs.build_entity().add(OrthographicCamera::new(-1., 1., -1., 1.));

        if ecs.read_res::<StateTimer>().countdown_s <= 0. {
            next_day(ecs);
        }
    }

    fn on_state_stop(&self, ecs: &mut ECS) {
        ecs.add_res(LastState(DAY_GAME_STATE));
    }

    fn render_state(&self, ecs: &ECS, scene: &Scene) {
        draw_everything(ecs, scene);
    }

    fn next_state(&self, ecs: &ECS) -> Option<u32> {
        if ecs.read_res::<StateTimer>().countdown_s <= 0. {
            return Some(NIGHT_GAME_STATE);
        } else {
            if ecs.read_res::<GameInput>().keyboard.letter_pressed(LetterKeys::B) {
                return Some(MAIN_MENU_GAME_STATE);
            }
            return None;
        }
    }

    fn active_layers(&self, ecs: &ECS) -> Vec<RenderLayer> {
        return vec![
            String::from("Tilemap"),
            String::from("Turrets"),
            String::from("Monsters"),
            String::from("Players"),
            String::from("GUI")
        ];
    }

    fn camera(&self, ecs: &ECS) -> Box<dyn Camera> {
        let cam = ecs.read1::<OrthographicCamera>()[0].0.clone();
        return Box::new(cam);
    }
}

pub const NIGHT_GAME_STATE: GameStateId = 11;

pub struct NightGameState {

}

impl GameState for NightGameState {
    fn on_state_start(&self, ecs: &mut ECS) {
        register_core_systems(ecs);
        ecs.add_system(Box::new(wave_spawner_system));
        ecs.add_system(Box::new(monster_move_system));
        ecs.add_system(Box::new(mouse_click_system));
        ecs.add_system(Box::new(mouse_cursor_system));
        ecs.add_system(Box::new(turret_attack_system));
        ecs.add_system(Box::new(arrow_move_system));
        ecs.add_system(Box::new(monster_damage_system));
        ecs.add_system(Box::new(day_state_countdown));
        ecs.build_entity().add(OrthographicCamera::new(-1., 1., -1., 1.));
        if ecs.read_res::<StateTimer>().countdown_s <= 0. {
            next_night(ecs);
        }
    }

    fn on_state_stop(&self, ecs: &mut ECS) {
        ecs.add_res(LastState(NIGHT_GAME_STATE));
    }

    fn render_state(&self, ecs: &ECS, scene: &Scene) {
        draw_everything(ecs, scene);
    }

    fn next_state(&self, ecs: &ECS) -> Option<u32> {
        if ecs.read_res::<StateTimer>().countdown_s <= 0. {
            return Some(DAY_GAME_STATE);
        } else {
            if ecs.read_res::<GameInput>().keyboard.letter_pressed(LetterKeys::B) {
                return Some(MAIN_MENU_GAME_STATE);
            }
            return None;
        }
    }

    fn active_layers(&self, ecs: &ECS) -> Vec<RenderLayer> {
        return vec![
            String::from("Tilemap"),
            String::from("Turrets"),
            String::from("Monsters"),
            String::from("Players"),
            String::from("GUI")
        ];
    }

    fn camera(&self, ecs: &ECS) -> Box<dyn Camera> {
        let cam = ecs.read1::<OrthographicCamera>()[0].0.clone();
        return Box::new(cam);
    }
}

pub struct LastState(GameStateId);

pub const MAIN_MENU_GAME_STATE: GameStateId = 1;

pub struct MainMenuGameState {

}

impl GameState for MainMenuGameState {
    fn on_state_start(&self, ecs: &mut ECS) {
        register_core_systems(ecs);
        ecs.add_system(Box::new(menu_mouse_system));
        let capture_layer = ecs.read_res::<MenuScreen>().capture_layer;
        ecs.add_component(&capture_layer, ActiveCaptureLayer {});
        let button_layer = ecs.read_res::<MenuScreen>().button_layer;
        ecs.add_component(&button_layer, ActiveCaptureLayer {});
        ecs.build_entity().add(OrthographicCamera::new(-1., 1., -1., 1.));
    }

    fn on_state_stop(&self, ecs: &mut ECS) {
        ecs.add_res(LastState(MAIN_MENU_GAME_STATE));
        let capture_layer = ecs.read_res::<MenuScreen>().capture_layer;
        ecs.remove_component::<ActiveCaptureLayer>(&capture_layer);
        let button_layer = ecs.read_res::<MenuScreen>().button_layer;
        ecs.remove_component::<ActiveCaptureLayer>(&button_layer);
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
            return Some(ecs.read_res::<LastState>().0);
        }
        if ecs.events::<ResumeEvent>().len() > 0 {
            return Some(ecs.read_res::<LastState>().0);
        }
        return None;
    }

    fn active_layers(&self, ecs: &ECS) -> Vec<RenderLayer> {
        return vec![
            String::from("Tilemap"),
            String::from("Turrets"),
            String::from("Monsters"),
            String::from("Players"),
            String::from("GUI")
        ];
    }

    fn camera(&self, ecs: &ECS) -> Box<dyn Camera> {
        let cam = ecs.read1::<OrthographicCamera>()[0].0.clone();
        return Box::new(cam);
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

    fn cleanup_components(&self, ecs: &mut ECS) {
        ecs.cleanup_components::<Tile>();
        ecs.cleanup_components::<Player>();
        ecs.cleanup_components::<Monster>();
        ecs.cleanup_components::<WaveSpawner>();
        ecs.cleanup_components::<SpriteComponent>();
        ecs.cleanup_components::<TransformComponent>();
        ecs.cleanup_components::<GuiComponent>();
        ecs.cleanup_components::<GuiText>();
        ecs.cleanup_components::<GuiImage>();
        ecs.cleanup_components::<ActiveGui>();
        ecs.cleanup_components::<MouseCaptureArea>();
        ecs.cleanup_components::<MouseCaptureLayer>();
        ecs.cleanup_components::<ActiveCaptureLayer>();
        ecs.cleanup_components::<Turret>();
        ecs.cleanup_components::<TreeComponent>();
        ecs.cleanup_components::<Arrow>();
        ecs.cleanup_components::<DamageDealt>();
        ecs.cleanup_components::<StageText>();
        ecs.cleanup_components::<GoldText>();
        ecs.cleanup_components::<WoodText>();
        ecs.cleanup_components::<StoneText>();
        ecs.cleanup_components::<StartMenuButtonId>();
    }

    fn register_events(&self, ecs: &mut ECS) {
        ecs.register_event_type::<ResumeEvent>();
        ecs.register_event_type::<SaveEvent>();
        ecs.register_event_type::<SettingsEvent>();
        ecs.register_event_type::<QuitEvent>();
        ecs.register_event_type::<StartEvent>();
    }

    fn migrate_events(&self, ecs: &mut ECS) {
        ecs.migrate_events::<ResumeEvent>();
        ecs.migrate_events::<SaveEvent>();
        ecs.migrate_events::<SettingsEvent>();
        ecs.migrate_events::<QuitEvent>();
        ecs.migrate_events::<StartEvent>();
    }

    fn game_states(&self) -> HashMap<GameStateId, Box<dyn GameState>> {
        let mut res: HashMap<GameStateId, Box<dyn GameState>> = HashMap::new();
        res.insert(START_MENU_GAME_STATE, Box::new(StartMenuGameState {}));
        res.insert(MAIN_GAME_STATE, Box::new(MainGameState {}));
        res.insert(MAIN_MENU_GAME_STATE, Box::new(MainMenuGameState {}));
        res.insert(DAY_GAME_STATE, Box::new(DayGameState {}));
        res.insert(NIGHT_GAME_STATE, Box::new(NightGameState {}));
        return res;
    }

    fn initial_game_state(&self) -> u32 {
        return START_MENU_GAME_STATE;
    }

    fn setup(&mut self, ecs: &mut ECS) {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let font = Font::new(renderer, "bitmap/segoe.fnt", "bitmap/segoe.png");
        ecs.add_res(Rc::new(font));

        ecs.add_res(StateTimer{countdown_s: 0.});

        GameGui::create(ecs);
        ecs.add_res(generate_days());
        ecs.add_res::<Vec<WindowEvent>>(vec![]);
        ecs.add_res(Supplies::new());
        StartMenu::create(ecs);
    }
}

fn main() {
    let mut gouda = Gouda::new(Game::new());
    gouda.run();
}
