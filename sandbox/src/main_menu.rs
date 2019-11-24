use gouda::ecs::{ECS, Entity, Mutation, Mutations};
use gouda::gui::{GuiComponent, ActiveGui, GuiText};
use gouda::types::{Color, Bounds};
use std::rc::Rc;
use gouda::rendering::Renderer;
use gouda::input::{GameInput, LetterKeys};
use gouda::gui::constraints::{Constraint, GuiConstraints};
use gouda::font::Font;
use gouda::mouse_capture::{MouseCaptureLayer, ActiveCaptureLayer};
use gouda::gui::constraints::Constraint::{RelativeConstraint, CenterConstraint};

pub struct MenuScreen {
    pub entity: Entity,
    pub button_layer: Entity,
    pub capture_layer: Entity,
    active: bool,
}

fn menu_mouse_system(ecs: &ECS) -> Mutations {
    let menu = ecs.read_res::<MenuScreen>();
    let mut mutations: Mutations = vec![];
    return mutations;
}

fn add_menu_button(menu_layer: Entity, bounds: Bounds, y: f32, ecs: &mut ECS, menu_screen_entity: Entity) {
    let button = GuiComponent::create_hoverable(
        ecs,
        Some(menu_layer),
        Some(bounds),
        GuiConstraints::new(
            Constraint::CenterConstraint,
            RelativeConstraint {size: y},
            RelativeConstraint {size: 0.6},
            RelativeConstraint {size: 0.12},
        ),
        10.,
        Color::from_u8(0x33, 0x33, 0x33, 0xAA),
        Color::from_u8(0x88, 0x33, 0x33, 0xAA),
    );

    let comp = ecs.read::<GuiComponent>(&button).unwrap();
    let font = ecs.read_res::<Rc<Font>>();
    let text = GuiText::create(
        ecs,
        Some(comp.calculated_bounds),
        String::from("BUTTON"),
        font.clone(),
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

pub struct MainMenu {}

impl MainMenu {

    pub fn create(ecs: &mut ECS) {
        let main_menu_layer = ecs.build_entity().add(MouseCaptureLayer {sort_index: 2, capture_areas: vec![]}).entity();
        let menu_button_layer = ecs.build_entity().add(MouseCaptureLayer {sort_index: 3, capture_areas: vec![]}).entity();
        let mut menu_screen_entity = GuiComponent::create(
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
            Color::from_u8(0xAA, 0xAA, 0xAA, 0x88),
        );

        let menu_screen = ecs.read::<GuiComponent>(&menu_screen_entity).unwrap();
        let bounds = menu_screen.calculated_bounds;
        add_menu_button(menu_button_layer, bounds, 0.2, ecs, menu_screen_entity);
        add_menu_button(menu_button_layer, bounds, 0.35, ecs, menu_screen_entity);
        add_menu_button(menu_button_layer, bounds, 0.5, ecs, menu_screen_entity);
        add_menu_button(menu_button_layer, bounds, 0.65, ecs, menu_screen_entity);


        let font = ecs.read_res::<Rc<Font>>().clone();
        let renderer = ecs.read_res::<Rc<Renderer>>().clone();


        let text = GuiText::create(
            ecs,
            Some(bounds),
            "LONG TEST STRING PAy ATTENTION TO ME".parse().unwrap(),
            font.clone(),
            true,
            true,
            20.,
            GuiConstraints::new(
                Constraint::CenterConstraint,
                Constraint::CenterConstraint,
                Constraint::RelativeConstraint {size: 1.},
                Constraint::RelativeConstraint {size: 1.}
            ),
            Color::from_u8(0x00, 0x00, 0x00, 0xFF));
        let menu_screen = ecs.write::<GuiComponent>(&menu_screen_entity).unwrap();
        menu_screen.add_text(text);
        ecs.add_res(MenuScreen {entity: menu_screen_entity, button_layer: menu_button_layer, capture_layer: main_menu_layer, active: false});
    }
}

