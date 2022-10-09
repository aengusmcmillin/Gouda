use cgmath::{Matrix4, Vector3, SquareMatrix};
use gouda::ecs::{GameSceneId, Entity, ECS, Mutation, Mutations};
use gouda::rendering::model::ObjModel;
use gouda::{GameScene, RenderLayer};
use gouda::mouse_capture::{MouseCaptureLayer, ActiveCaptureLayer, MouseCaptureArea};
use gouda::gui::{GuiComponent, GuiText};
use gouda::gui::constraints::{GuiConstraints, Constraint};
use gouda::types::{Color, Bounds};
use crate::{register_core_systems, LastScene, MAIN_GAME_SCENE};
use gouda::rendering::Scene;
use gouda::gui::constraints::Constraint::{RelativeConstraint, CenterConstraint};
use std::rc::Rc;
use gouda::font::Font;
use crate::start_menu::StartMenuButtonId::Start;
use crate::tilemap::{Tilemap};
use crate::cursor::Cursor;
use crate::player::Player;
use gouda::camera::{Camera, OrthographicCamera};
use crate::main_menu::MainMenu;

pub struct StartMenuScreen {
    pub entity: Entity,
    pub button_layer: Entity,
    pub capture_layer: Entity,
}

pub struct StartEvent;

#[derive(Debug, Copy, Clone)]
pub enum StartMenuButtonId {
    Start,
}

pub const START_MENU_SCENE: GameSceneId = 3;

pub struct StartMenuScene {}

impl GameScene for StartMenuScene {
    fn on_scene_start(&self, ecs: &mut ECS) {
        register_core_systems(ecs);
        ecs.add_system(Box::new(start_menu_mouse_system));
        let capture_layer = ecs.read_res::<StartMenuScreen>().capture_layer;
        ecs.add_component(&capture_layer, ActiveCaptureLayer {});
        let button_layer = ecs.read_res::<StartMenuScreen>().button_layer;
        ecs.add_component(&button_layer, ActiveCaptureLayer {});
        ecs.build_entity().add(OrthographicCamera::new(-8., 8., -8., 8.));
    }

    fn on_scene_stop(&self, ecs: &mut ECS) {
        ecs.add_res(LastScene(START_MENU_SCENE));
        let capture_layer = ecs.read_res::<StartMenuScreen>().capture_layer;
        ecs.remove_component::<ActiveCaptureLayer>(&capture_layer);
        let button_layer = ecs.read_res::<StartMenuScreen>().button_layer;
        ecs.remove_component::<ActiveCaptureLayer>(&button_layer);
    }

    fn render_scene(&self, ecs: &ECS, scene: &Scene) {
        let model = ecs.read_res::<ObjModel>();
        let transform = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.5)) *
            Matrix4::from_nonuniform_scale(1., 1., 1.);
        scene.submit_obj(model, transform);
        // let menu = ecs.read_res::<StartMenuScreen>();
        // let menugui = ecs.read::<GuiComponent>(&menu.entity);
        // menugui.unwrap().render(&ecs, &scene);
    }

    fn next_scene(&self, ecs: &ECS) -> Option<u32> {
        if ecs.events::<StartEvent>().len() > 0 {
            return Some(MAIN_GAME_SCENE);
        }
        return None;
    }

    fn active_layers(&self, ecs: &ECS) -> Vec<RenderLayer> {
        return vec![
            String::from("GUI")
        ];
    }

    fn camera(&self, ecs: &ECS) -> Box<dyn Camera> {
        let cam = ecs.read1::<OrthographicCamera>()[0].0.clone();
        return Box::new(cam);
    }
}

pub struct MenuClickMutation {
    buttonid: StartMenuButtonId,
}

impl Mutation for MenuClickMutation {
    fn apply(&self, ecs: &mut ECS) {
        match self.buttonid {
            Start => {
                Tilemap::create(ecs);
                Cursor::create(ecs);
                Player::create(ecs);

                MainMenu::create(ecs);
                ecs.push_event(StartEvent)
            },
        }
    }
}

pub fn start_menu_mouse_system(ecs: &ECS, dt: f32) -> Mutations {
    let mut mutations: Mutations = vec![];
    for (capture_area, button, _) in ecs.read2::<MouseCaptureArea, StartMenuButtonId>() {
        if capture_area.clicked_buttons[0] {
            mutations.push(Box::new(MenuClickMutation {
                buttonid: *button,
            }));
        }
    }
    return mutations;
}


fn add_menu_button(button_id: StartMenuButtonId, text: &str, menu_layer: Entity, bounds: Bounds, y: f32, ecs: &mut ECS, menu_screen_entity: Entity) {
    let button = GuiComponent::create_hoverable(
        ecs,
        Some(menu_layer),
        Some(bounds),
        GuiConstraints::new(
            Constraint::CenterConstraint,
            Constraint::CenterConstraint,
            RelativeConstraint {size: 0.6},
            RelativeConstraint {size: 0.12},
        ),
        10.,
        Color::from_u8(0x33, 0x33, 0x33, 0xAA),
        Color::from_u8(0x88, 0x33, 0x33, 0xAA),
    );
    ecs.add_component(&button, button_id);

    let comp = ecs.read::<GuiComponent>(&button).unwrap();
    let text = GuiText::create(
        ecs,
        Some(comp.calculated_bounds),
        String::from(text),
        "segoe",
        true,
        true,
        32.,
        GuiConstraints::new(
            CenterConstraint,
            CenterConstraint,
            RelativeConstraint {size: 1.},
            RelativeConstraint {size: 1.},
        ),
        Color::from_u8(0xFF, 0xFF, 0xFF, 0xFF),
    );
    let comp = ecs.write::<GuiComponent>(&button).unwrap();
    comp.add_text(text);

    let menu_screen = ecs.write::<GuiComponent>(&menu_screen_entity).unwrap();
    menu_screen.add_child(button);
}

pub struct StartMenu {}

impl StartMenu {

    pub fn create(ecs: &mut ECS) {
        let main_menu_layer = ecs.build_entity().add(MouseCaptureLayer {sort_index: 2, capture_areas: vec![]}).entity();
        let menu_button_layer = ecs.build_entity().add(MouseCaptureLayer {sort_index: 3, capture_areas: vec![]}).entity();
        let menu_screen_entity = GuiComponent::create(
            ecs,
            Some(main_menu_layer),
            None,
            GuiConstraints::new(
                Constraint::CenterConstraint,
                Constraint::CenterConstraint,
                Constraint::RelativeConstraint {size: 1.},
                Constraint::RelativeConstraint {size: 1.},
            ),
            0.,
            Color::from_u8(0xAA, 0xAA, 0xAA, 0xFF),
        );

        let menu_screen = ecs.read::<GuiComponent>(&menu_screen_entity).unwrap();
        let bounds = menu_screen.calculated_bounds;
        add_menu_button(Start, "Start", menu_button_layer, bounds, 0.5, ecs, menu_screen_entity);

        ecs.write::<GuiComponent>(&menu_screen_entity).unwrap();
        ecs.add_res(StartMenuScreen {entity: menu_screen_entity, button_layer: menu_button_layer, capture_layer: main_menu_layer});
    }
}

