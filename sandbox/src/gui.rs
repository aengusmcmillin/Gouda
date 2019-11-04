use gouda::ecs::{ECS, Entity, Mutations, Mutation};
use gouda::gui::{GuiComponent, ActiveGui};
use gouda::types::Color;
use std::rc::Rc;
use gouda::rendering::Renderer;
use gouda::gui::constraints::{Constraint, GuiConstraints};
use gouda::gui::constraints::Constraint::RelativeConstraint;
use gouda::mouse_capture::{MouseCaptureLayer, MouseCaptureArea, ActiveCaptureLayer};

pub struct GuiHoveredMutation {
    entity: Entity,
    hovered: bool,
}

impl Mutation for GuiHoveredMutation {
    fn apply(&self, ecs: &mut ECS) {
        ecs.write::<GuiComponent>(&self.entity).unwrap().set_hovered(self.hovered);
    }
}

pub fn game_gui_system(ecs: &ECS) -> Mutations {
    let mut mutations: Mutations = vec![];
    for (capture, gui, entity) in ecs.read2::<MouseCaptureArea, GuiComponent>() {
        mutations.push(Box::new(GuiHoveredMutation{entity, hovered: capture.is_hovered}));

        if capture.clicked_buttons[0] {
            println!("Clicked a button");
        }
    }
    return mutations;
}

pub struct GameGui {

}

impl GameGui {
    pub fn create(ecs: &mut ECS) {
        println!("Creating gui");
        let renderer = ecs.read_res::<Rc<Renderer>>();

        let mouse_layer = ecs.build_entity().add(MouseCaptureLayer {sort_index: 1, capture_areas: vec![]}).add(ActiveCaptureLayer {}).entity();
        let bottom_panel = create_bottom_panel(ecs, mouse_layer);
        let top_bar = create_top_bar(ecs, mouse_layer);

        ecs.add_component(&bottom_panel, ActiveGui {});
        ecs.add_component(&top_bar, ActiveGui {});

    }
}

fn create_top_bar(ecs: &mut ECS, mouse_layer: Entity) -> Entity {
    const TOP_BAR_PERCENT_HEIGHT: f32 = 0.03;
    let top_bar = GuiComponent::create_hoverable(
        ecs,
        Some(mouse_layer),
        None,
        GuiConstraints::new(
            Constraint::CenterConstraint,
            Constraint::RelativeConstraint {size: 1.0 - TOP_BAR_PERCENT_HEIGHT},
            RelativeConstraint {size: 1.0},
            RelativeConstraint {size: TOP_BAR_PERCENT_HEIGHT},
        ),
        0.0,
        Color::from_u8(0x22, 0x22, 0x22, 0xFF),
    Color::from_u8(0x55, 0x55, 0x55, 0xFF));

    ecs.write::<MouseCaptureLayer>(&mouse_layer).unwrap().capture_areas.push(top_bar);
    return top_bar;
}

fn create_bottom_panel(ecs: &mut ECS, mouse_layer: Entity) -> Entity {
    let mut bottom_panel_entity = GuiComponent::create(
        ecs,
        None,
        None,
        GuiConstraints::new(
            Constraint::CenterConstraint,
            Constraint::RelativeConstraint { size: 0.0 },
            Constraint::RelativeConstraint {size: 1.},
            Constraint::PixelConstraint {size: 160}
        ),
        0.,
        Color::from_u8(0x22, 0x22, 0x22, 0xFF));
    let bottom_panel = ecs.read::<GuiComponent>(&bottom_panel_entity).unwrap();
    let mut buttons_box_entity = GuiComponent::create(
        ecs,
        None,
        Some(bottom_panel.calculated_bounds),
        GuiConstraints::new(
            Constraint::CenterConstraint,
            Constraint::CenterConstraint,
            Constraint::PixelConstraint {size: -15},
            Constraint::PixelConstraint {size: -15},
        ),
        8.,
        Color::from_u8(0x44, 0x44, 0x44, 0xFF));
    let buttons_box = ecs.read::<GuiComponent>(&buttons_box_entity).unwrap();
    let buttons_box_bounds = buttons_box.calculated_bounds.clone();
    let child1 = GuiComponent::create_hoverable(
        ecs,
        Some(mouse_layer),
        Some(buttons_box_bounds),
        GuiConstraints::new(
            Constraint::RelativeConstraint { size: 0.05 },
            Constraint::CenterConstraint,
            Constraint::AspectConstraint {aspect: 1.0},
            Constraint::PixelConstraint {size: -10},
        ),
        20.,
        Color::from_u8(0x88, 0x22, 0x33, 0xFF),
        Color::from_u8(0x22, 0x88, 0x33, 0xFF),
    );
    let child2 = GuiComponent::create_hoverable(
        ecs,
        Some(mouse_layer),
        Some(buttons_box_bounds),
        GuiConstraints::new(
            Constraint::RelativeConstraint { size: 0.25 },
            Constraint::CenterConstraint,
            Constraint::AspectConstraint {aspect: 1.0},
            Constraint::PixelConstraint {size: -10},
        ),
        20.,
        Color::from_u8(0x88, 0x22, 0x33, 0xFF),
        Color::from_u8(0x22, 0x88, 0x33, 0xFF),
    );
    ecs.write::<MouseCaptureLayer>(&mouse_layer).unwrap().capture_areas.push(child1);
    ecs.write::<MouseCaptureLayer>(&mouse_layer).unwrap().capture_areas.push(child2);
    let mut buttons_box = ecs.write::<GuiComponent>(&buttons_box_entity).unwrap();
    buttons_box.add_child(child1);
    buttons_box.add_child(child2);
    let mut bottom_panel = ecs.write::<GuiComponent>(&bottom_panel_entity).unwrap();
    bottom_panel.add_child(buttons_box_entity);
    return bottom_panel_entity;
}

