use gouda::{Gouda, GameLogic};
use gouda::ecs::{ECS, Mutations, Mutation, Entity};
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
use crate::monster::{Monster, monster_move_system, monster_spawn_system, Spawner};
use gouda::gui::{GuiComponent, GuiConstraints, Constraint, ActiveGui};
use gouda::types::Color;
use gouda::gui::Constraint::RelativeConstraint;
use gouda::input::AnyKey::Letter;

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

struct MenuScreen {
    entity: Entity,
    active: bool,
}

#[derive(Debug)]
struct Pos {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
struct Hearth {
    drawable: QuadDrawable,
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
        let renderer = ecs.read_res::<Rc<Renderer>>().clone();
        ecs.write_res::<Camera>().change_width(&renderer, self.dw);
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
        ecs.register_component_type::<GuiElement>();
        ecs.register_component_type::<TextElement>();
        ecs.register_component_type::<Spawner>();
        ecs.register_component_type::<WaveSpawner>();
        ecs.register_component_type::<Pos>();
        ecs.register_component_type::<GuiComponent>();
        ecs.register_component_type::<ActiveGui>();
    }

    fn register_systems(&self, ecs: &mut ECS) {
        ecs.add_system(Box::new(player_move_system));
        ecs.add_system(Box::new(mouse_click_system));
        ecs.add_system(Box::new(monster_spawn_system));
        ecs.add_system(Box::new(wave_spawner_system));
        ecs.add_system(Box::new(monster_move_system));
        ecs.add_system(Box::new(menu_show_system));

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
        let mut component = GuiComponent::new(
            renderer,
            None,
            GuiConstraints::new(
                Constraint::CenterConstraint,
                Constraint::RelativeConstraint { size: 0.0 },
                Constraint::RelativeConstraint {size: 1.},
                Constraint::PixelConstraint {size: 160}
            ),
            0.,
            Color {r: 1., g: 0., b: 0., a: 1.});
        let sub_component = GuiComponent::new(
            renderer,
            Some(&component),
            GuiConstraints::new(
                Constraint::CenterConstraint,
                Constraint::CenterConstraint,
                Constraint::PixelConstraint {size: -15},
                Constraint::PixelConstraint {size: -15},
        ),
            0.,
            Color {r: 0., g: 1., b: 1., a: 1.});
        component.add_child(sub_component);
        ecs.build_entity().add(component).add(ActiveGui{});

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let menu_screen = GuiComponent::new(
            renderer,
            None,
            GuiConstraints::new(
                Constraint::CenterConstraint,
                Constraint::CenterConstraint,
                Constraint::RelativeConstraint {size: 1.},
                Constraint::RelativeConstraint {size: 1.},
            ),
            0.,
            Color::new(0.2, 0.2, 0.2, 1.0)
        );
        let menu = ecs.build_entity().add(menu_screen).entity();
        ecs.add_res(MenuScreen {entity: menu, active: false});


        let renderer = ecs.read_res::<Rc<Renderer>>();
        let font = Font::new(renderer, "bitmap/segoe.fnt", "bitmap/segoe.png");
        let text = TextDrawable::new(renderer, [-0.95, -0.68], Rc::new(font), "Test String #1: Segoe".to_string(), 14.);
        ecs.build_entity().add(TextElement { drawable: text, });

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let font = Font::new(renderer, "bitmap/arial.fnt", "bitmap/arial.png");
        let text = TextDrawable::new(renderer, [-0.95, -0.75], Rc::new(font), "Test String #2: Arial".to_string(), 14.);
        ecs.build_entity().add(TextElement { drawable: text, });

        ecs.build_entity().add(Spawner {max_time: 5., current_time: 5.});

        ecs.build_entity()
            .add(WaveSpawner::new(
                WaveSpec {
                    monsters: vec![MonsterSpec { monster_type: MonsterType::Wolf }; 20],
                }, 1.))
            .add(Pos {x: 3., y: 3.});
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

        for (gui, e) in ecs.read1::<GuiElement>() {
            gui.drawable.draw(&scene);
        }

        for (gui, _active, e) in ecs.read2::<GuiComponent, ActiveGui>() {
            gui.render(&scene);
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
