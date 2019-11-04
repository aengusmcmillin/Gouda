use gouda::ecs::{ECS, Entity, Mutation, Mutations};
use gouda::gui::{GuiComponent, ActiveGui, GuiText};
use gouda::types::Color;
use std::rc::Rc;
use gouda::rendering::Renderer;
use gouda::input::{GameInput, LetterKeys};
use gouda::gui::constraints::{Constraint, GuiConstraints};
use gouda::font::Font;
use gouda::mouse_capture::{MouseCaptureLayer, ActiveCaptureLayer};

pub struct MenuScreen {
    pub entity: Entity,
    pub capture_layer: Entity,
    active: bool,
}


pub struct MainMenu {}

impl MainMenu {

    pub fn create(ecs: &mut ECS) {
        let main_menu_layer = ecs.build_entity().add(MouseCaptureLayer {sort_index: 2, capture_areas: vec![]}).entity();
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
        let font = ecs.read_res::<Rc<Font>>().clone();
        let renderer = ecs.read_res::<Rc<Renderer>>().clone();
        let menu_screen = ecs.write::<GuiComponent>(&menu_screen_entity).unwrap();
        menu_screen.add_text(GuiText::new(
            &renderer,
            Some(menu_screen.calculated_bounds),
            "LONG TEST STRING PAy ATTENTION TO ME".parse().unwrap(),
            font.clone(),
            GuiConstraints::new(
                Constraint::CenterConstraint,
                Constraint::CenterConstraint,
                Constraint::RelativeConstraint {size: 1.},
                Constraint::RelativeConstraint {size: 1.}
            ),
            Color::from_u8(0x00, 0x00, 0x00, 0xFF)));
        ecs.add_res(MenuScreen {entity: menu_screen_entity, capture_layer: main_menu_layer, active: false});
    }
}

pub struct ShowMenuMutation {

}

impl Mutation for ShowMenuMutation {
    fn apply(&self, ecs: &mut ECS) {
        let menu = ecs.write_res::<MenuScreen>();
        let was_active = menu.active;
        let e = menu.entity.clone();
        let capture_layer = menu.capture_layer.clone();
        menu.active = !menu.active;
        if was_active {
            ecs.remove_component::<ActiveGui>(&e);
            ecs.remove_component::<ActiveCaptureLayer>(&capture_layer);
        } else {
            ecs.add_component(&e, ActiveGui {});
            ecs.add_component(&capture_layer, ActiveCaptureLayer {});
        }
    }
}

pub fn menu_show_system(ecs: &ECS) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let mut mutations: Mutations = Vec::new();
    if input.keyboard.letter_pressed(LetterKeys::B) {
        mutations.push(Box::new(ShowMenuMutation {}));
    }
    return mutations;
}
