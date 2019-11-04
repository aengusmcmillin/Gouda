use gouda::ecs::{ECS, Entity, Mutation, Mutations};
use gouda::gui::{GuiComponent, ActiveGui, GuiText};
use gouda::types::Color;
use std::rc::Rc;
use gouda::rendering::Renderer;
use gouda::input::{GameInput, LetterKeys};
use gouda::gui::constraints::{Constraint, GuiConstraints};
use gouda::font::Font;

pub struct MenuScreen {
    pub entity: Entity,
    active: bool,
}


pub struct MainMenu {}

impl MainMenu {

    pub fn create(ecs: &mut ECS) {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let mut menu_screen = GuiComponent::new(
            renderer,
            None,
            GuiConstraints::new(
                Constraint::CenterConstraint,
                Constraint::CenterConstraint,
                Constraint::RelativeConstraint {size: 1.},
                Constraint::RelativeConstraint {size: 1.},
            ),
            0.,
            Color::new(0.9, 0.5, 0.5, 1.0),
        );
        let font = ecs.read_res::<Rc<Font>>();
        menu_screen.add_text(GuiText::new(
            renderer,
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
        let menu = ecs.build_entity().add(menu_screen).entity();
        ecs.add_res(MenuScreen {entity: menu, active: false});
    }
}

pub struct ShowMenuMutation {

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

pub fn menu_show_system(ecs: &ECS) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let mut mutations: Mutations = Vec::new();
    if input.keyboard.letter_pressed(LetterKeys::B) {
        mutations.push(Box::new(ShowMenuMutation {}));
    }
    return mutations;
}
