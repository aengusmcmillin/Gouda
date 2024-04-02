use camera::{camera_control_system, CameraComponent};
use gouda::camera::{Camera, OrthographicCamera};
use gouda::ecs::{Entity, GameSceneId, Mutation, Mutations, ECS};
use gouda::input::{GameInput, LetterKeys};
use gouda::rendering::drawable::ShapeDrawable;
use gouda::rendering::obj::{load_mtl_file, load_obj_file, ObjMesh};
use gouda::rendering::sprites::{ColorBoxComponent, SpriteComponent, SpriteSheetComponent};
use gouda::rendering::{Renderer, Scene};
use gouda::transform::TransformComponent;
use gouda::{GameLogic, GameScene, Gouda, QuitEvent, RenderLayer};
use std::rc::Rc;
use tree::create_tree;

extern crate rand;
use crate::building::{arrow_move_system, turret_attack_system, Turret};
use crate::cursor::Cursor;
use crate::gui::{change_stage_text, game_gui_system, GameGui};
use crate::main_menu::{menu_mouse_system, MenuScreen, ResumeEvent, SaveEvent, SettingsEvent};
use crate::monster::{monster_damage_system, monster_move_system, Monster};
use crate::player::{player_move_system, Player};
use crate::spawners::{generate_days, wave_spawner_system, GameDay, WaveSpawner};
use crate::start_menu::{StartEvent, StartMenu, StartMenuScene, START_MENU_SCENE};
use crate::supplies::Supplies;
use crate::tilemap::{Tile, Tilemap};
use crate::tree::TreeComponent;
use crate::turret::{CreateTurretMutation, TurretDeselectMutation, TurretSelectMutation};
use gouda::gui::{ActiveGui, GuiComponent};
use gouda::mouse_capture::{mouse_capture_system, ActiveCaptureLayer, MouseCaptureArea};
use gouda::window::{WindowEvent, WindowProps};
use rand::{thread_rng, Rng};
use std::collections::HashMap;

mod building;
mod camera;
mod cursor;
mod game_stage;
mod gui;
mod hearth;
mod main_menu;
mod monster;
mod pathfinding;
mod player;
mod spawners;
mod start_menu;
mod supplies;
mod tilemap;
mod tree;
mod turret;
mod villager;

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

fn mouse_click_system(ecs: &ECS, _dt: f32) -> Mutations {
    let mut mutations: Mutations = Vec::new();
    for (tile, mouse_capture, tile_e) in ecs.read2::<Tile, MouseCaptureArea>() {
        if mouse_capture.clicked_buttons[0] {
            mutations.push(Box::new(TurretDeselectMutation {}));
            if !tile.occupied {
                mutations.push(Box::new(CreateTurretMutation { tile_e }));
            } else {
                for (turret, loc, e) in ecs.read2::<Turret, TransformComponent>() {
                    if loc.position.x == tile.x as f32
                        && loc.position.y == tile.y as f32
                        && !turret.selected
                    {
                        mutations.push(Box::new(TurretSelectMutation { turret_e: e }));
                    }
                }

                for (_, location, e) in ecs.read2::<TreeComponent, TransformComponent>() {
                    if location.position.x == tile.x as f32 && location.position.y == tile.y as f32
                    {
                        mutations.push(Box::new(TreeHarvestMutation { tree: e }))
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

fn mouse_cursor_system(ecs: &ECS, _dt: f32) -> Mutations {
    let mut mutations: Mutations = vec![];
    let mut any_hovered = false;
    for (_, mouse_capture, e) in ecs.read2::<Tile, MouseCaptureArea>() {
        if mouse_capture.is_hovered {
            mutations.push(Box::new(CursorSetPositionMutation { tile: e }));
            any_hovered = true;
        }
    }
    mutations.push(Box::new(CursorVisibilityMutation {
        visible: any_hovered,
    }));
    return mutations;
}

fn register_core_systems(ecs: &mut ECS) {
    ecs.add_system(Box::new(player_move_system));
    ecs.add_system(Box::new(mouse_capture_system));
    ecs.add_system(Box::new(game_gui_system));
}

fn draw_everything(ecs: &ECS, scene: &Scene) {
    for (location, sprite, _) in ecs.read2::<TransformComponent, SpriteComponent>() {
        sprite.draw(&scene, location);
    }

    for (location, color_box, _) in ecs.read2::<TransformComponent, ColorBoxComponent>() {
        color_box.draw(&scene, location);
    }

    for (location, spritesheet, _) in ecs.read2::<TransformComponent, SpriteSheetComponent>() {
        spritesheet.draw(&scene, location);
    }

    for (shape, transform, _) in ecs.read2::<ShapeDrawable, TransformComponent>() {
        scene.submit_shape_by_name(
            &shape.shader_name,
            &shape.shape_name,
            transform.transform_matrix(),
            shape.color,
        );
    }

    ecs.read_res::<Cursor>().draw(&scene);
    ecs.read_res::<ObjMesh>().draw(&scene);

    for (player, _) in ecs.read1::<Player>() {
        player.draw(&scene);
    }

    for (gui, _active, _) in ecs.read2::<GuiComponent, ActiveGui>() {
        gui.render(&ecs, &scene);
    }
}

pub const MAIN_GAME_SCENE: GameSceneId = 0;

pub struct MainGameScene {}

impl GameScene for MainGameScene {
    fn on_scene_start(&self, ecs: &mut ECS) {
        register_core_systems(ecs);
        ecs.add_system(Box::new(wave_spawner_system));
        ecs.add_system(Box::new(monster_move_system));
        ecs.add_system(Box::new(mouse_click_system));
        ecs.add_system(Box::new(mouse_cursor_system));
        ecs.add_system(Box::new(turret_attack_system));
        ecs.add_system(Box::new(arrow_move_system));
        ecs.add_system(Box::new(monster_damage_system));
        ecs.build_entity()
            .add_component(OrthographicCamera::new(6.))
            .add_component(CameraComponent::new())
            .add_component(TransformComponent::builder().build());
    }

    fn on_scene_stop(&self, ecs: &mut ECS) {
        ecs.add_res(LastScene(MAIN_GAME_SCENE));
    }

    fn render_scene(&self, ecs: &ECS, scene: &Scene) {
        draw_everything(ecs, scene);
    }

    fn next_scene(&self, ecs: &ECS) -> Option<u32> {
        let input = ecs.read_res::<GameInput>();
        if input.keyboard.letter_pressed(LetterKeys::B) {
            return Some(MAIN_MENU_GAME_SCENE);
        }
        return None;
    }

    fn active_layers(&self, _ecs: &ECS) -> Vec<RenderLayer> {
        return vec![
            String::from("Tilemap"),
            String::from("Turrets"),
            String::from("Monsters"),
            String::from("Players"),
            String::from("GUI"),
        ];
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
        change_stage_text(
            ecs,
            &(ecs.read_res::<StateTimer>().countdown_s.ceil().to_string()),
        );
    }
}

fn day_state_countdown(_ecs: &ECS, dt: f32) -> Mutations {
    return vec![Box::new(StateCountdownMutation { dt: dt })];
}

fn next_day(ecs: &mut ECS) {
    change_stage_text(ecs, "Day");

    let game_day = ecs.write_res::<Vec<GameDay>>().remove(0);
    ecs.add_res(game_day);
    ecs.write_res::<StateTimer>().countdown_s = ecs.read_res::<GameDay>().day_length;

    let tilemap = ecs.read_res::<Tilemap>();
    let borders: Vec<(f32, f32)> = tilemap
        .borders()
        .iter()
        .map(|border| tilemap.pos_of_tile(*border))
        .collect();

    let waves = &ecs.read_res::<GameDay>().waves.clone();
    for wave in waves {
        let border_index = thread_rng().gen_range(0, borders.len());
        let border = borders.get(border_index).unwrap();
        WaveSpawner::create(ecs, wave.wave.clone(), border.0, border.1, 1.);
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

pub const DAY_GAME_SCENE: GameSceneId = 10;

pub struct DayGameScene {}

impl GameScene for DayGameScene {
    fn on_scene_start(&self, ecs: &mut ECS) {
        register_core_systems(ecs);
        ecs.add_system(Box::new(mouse_click_system));
        ecs.add_system(Box::new(mouse_cursor_system));
        ecs.add_system(Box::new(day_state_countdown));
        ecs.build_entity()
            .add_component(OrthographicCamera::new(6.))
            .add_component(CameraComponent::new())
            .add_component(TransformComponent::builder().build());

        if ecs.read_res::<StateTimer>().countdown_s <= 0. {
            next_day(ecs);
        }
    }

    fn on_scene_stop(&self, ecs: &mut ECS) {
        ecs.add_res(LastScene(DAY_GAME_SCENE));
    }

    fn render_scene(&self, ecs: &ECS, scene: &Scene) {
        draw_everything(ecs, scene);
    }

    fn next_scene(&self, ecs: &ECS) -> Option<u32> {
        if ecs.read_res::<StateTimer>().countdown_s <= 0. {
            return Some(NIGHT_GAME_SCENE);
        } else {
            if ecs
                .read_res::<GameInput>()
                .keyboard
                .letter_pressed(LetterKeys::B)
            {
                return Some(MAIN_MENU_GAME_SCENE);
            }
            return None;
        }
    }

    fn active_layers(&self, _ecs: &ECS) -> Vec<RenderLayer> {
        return vec![
            String::from("Tilemap"),
            String::from("Turrets"),
            String::from("Monsters"),
            String::from("Players"),
            String::from("GUI"),
        ];
    }
}

pub const NIGHT_GAME_SCENE: GameSceneId = 11;

pub struct NightGameScene {}

impl GameScene for NightGameScene {
    fn on_scene_start(&self, ecs: &mut ECS) {
        register_core_systems(ecs);
        ecs.add_system(Box::new(wave_spawner_system));
        ecs.add_system(Box::new(monster_move_system));
        ecs.add_system(Box::new(mouse_click_system));
        ecs.add_system(Box::new(mouse_cursor_system));
        ecs.add_system(Box::new(turret_attack_system));
        ecs.add_system(Box::new(arrow_move_system));
        ecs.add_system(Box::new(monster_damage_system));
        ecs.add_system(Box::new(day_state_countdown));
        ecs.build_entity()
            .add_component(OrthographicCamera::new(6.))
            .add_component(CameraComponent::new())
            .add_component(TransformComponent::builder().build());
        if ecs.read_res::<StateTimer>().countdown_s <= 0. {
            next_night(ecs);
        }
    }

    fn on_scene_stop(&self, ecs: &mut ECS) {
        ecs.add_res(LastScene(NIGHT_GAME_SCENE));
    }

    fn render_scene(&self, ecs: &ECS, scene: &Scene) {
        draw_everything(ecs, scene);
    }

    fn next_scene(&self, ecs: &ECS) -> Option<u32> {
        if ecs.read_res::<StateTimer>().countdown_s <= 0. {
            return Some(DAY_GAME_SCENE);
        } else {
            if ecs
                .read_res::<GameInput>()
                .keyboard
                .letter_pressed(LetterKeys::B)
            {
                return Some(MAIN_MENU_GAME_SCENE);
            }
            return None;
        }
    }

    fn active_layers(&self, _ecs: &ECS) -> Vec<RenderLayer> {
        return vec![
            String::from("Tilemap"),
            String::from("Turrets"),
            String::from("Monsters"),
            String::from("Players"),
            String::from("GUI"),
        ];
    }
}

pub struct LastScene(GameSceneId);

pub const MAIN_MENU_GAME_SCENE: GameSceneId = 1;

pub struct MainMenuGameScene {}

impl GameScene for MainMenuGameScene {
    fn on_scene_start(&self, ecs: &mut ECS) {
        register_core_systems(ecs);
        ecs.add_system(Box::new(menu_mouse_system));
        let capture_layer = ecs.read_res::<MenuScreen>().capture_layer;
        ecs.add_component(&capture_layer, ActiveCaptureLayer {});
        let button_layer = ecs.read_res::<MenuScreen>().button_layer;
        ecs.add_component(&button_layer, ActiveCaptureLayer {});
        ecs.build_entity()
            .add_component(OrthographicCamera::new(6.))
            .add_component(CameraComponent::new())
            .add_component(TransformComponent::builder().build());
    }

    fn on_scene_stop(&self, ecs: &mut ECS) {
        ecs.add_res(LastScene(MAIN_MENU_GAME_SCENE));
        let capture_layer = ecs.read_res::<MenuScreen>().capture_layer;
        ecs.remove_component::<ActiveCaptureLayer>(&capture_layer);
        let button_layer = ecs.read_res::<MenuScreen>().button_layer;
        ecs.remove_component::<ActiveCaptureLayer>(&button_layer);
    }

    fn render_scene(&self, ecs: &ECS, scene: &Scene) {
        draw_everything(ecs, scene);
        let menu = ecs.read_res::<MenuScreen>();
        let menugui = ecs.read::<GuiComponent>(&menu.entity);
        menugui.unwrap().render(&ecs, &scene);
    }

    fn next_scene(&self, ecs: &ECS) -> Option<u32> {
        let input = ecs.read_res::<GameInput>();
        if input.keyboard.letter_pressed(LetterKeys::B) {
            return Some(ecs.read_res::<LastScene>().0);
        }
        if ecs.events::<ResumeEvent>().len() > 0 {
            return Some(ecs.read_res::<LastScene>().0);
        }
        return None;
    }

    fn active_layers(&self, _ecs: &ECS) -> Vec<RenderLayer> {
        return vec![
            String::from("Tilemap"),
            String::from("Turrets"),
            String::from("Monsters"),
            String::from("Players"),
            String::from("GUI"),
        ];
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
            title: "Hearth of Hestia".to_string(),
            target_ms_per_frame: 30.0,
        }
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

    fn game_scenes(&self) -> HashMap<GameSceneId, Box<dyn GameScene>> {
        let mut res: HashMap<GameSceneId, Box<dyn GameScene>> = HashMap::new();
        res.insert(START_MENU_SCENE, Box::new(StartMenuScene {}));
        res.insert(MAIN_GAME_SCENE, Box::new(MainGameScene {}));
        res.insert(MAIN_MENU_GAME_SCENE, Box::new(MainMenuGameScene {}));
        res.insert(DAY_GAME_SCENE, Box::new(DayGameScene {}));
        res.insert(NIGHT_GAME_SCENE, Box::new(NightGameScene {}));
        return res;
    }

    fn initial_game_scene(&self) -> u32 {
        return START_MENU_SCENE;
    }

    fn setup(&mut self, ecs: &mut ECS) {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let objfile = load_obj_file("./assets/models/tree.obj").unwrap();
        let mtlfile = load_mtl_file("./assets/models/tree.mtl").unwrap();
        let model = ObjMesh::new(renderer, objfile, mtlfile);

        ecs.add_res(StateTimer { countdown_s: 0. });

        ecs.add_res(model);

        GameGui::create(ecs);
        ecs.add_res(generate_days());
        ecs.add_res(Supplies::new());
        StartMenu::create(ecs);
    }
}

fn main() {
    let mut gouda = Gouda::new(Game::new());
    gouda.run();
}
