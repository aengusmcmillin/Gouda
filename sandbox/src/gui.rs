use gouda::ecs::{ECS, Entity, Mutations, Mutation};
use gouda::gui::{GuiComponent, ActiveGui, GuiText, GuiImage};
use gouda::types::Color;
use std::rc::Rc;
use gouda::rendering::Renderer;
use gouda::gui::constraints::{Constraint, GuiConstraints};
use gouda::gui::constraints::Constraint::{RelativeConstraint, CenterConstraint};
use gouda::mouse_capture::{MouseCaptureLayer, MouseCaptureArea, ActiveCaptureLayer};
use gouda::font::Font;
use gouda::images::png::PNG;
use crate::supplies::Supplies;

pub fn change_stage_text(ecs: &mut ECS, text: &str) {
    let e = ecs.get2::<StageText, GuiText>().first().unwrap().clone();
    let font = ecs.read_res::<Rc<Font>>().clone();
    let renderer = ecs.read_res::<Rc<Renderer>>().clone();
    ecs.write::<GuiText>(&e).unwrap().change_text(&renderer, String::from(text), font);
}

pub fn change_gold_text(ecs: &mut ECS) {
    let e = ecs.get2::<GoldText, GuiText>().first().unwrap().clone();
    let font = ecs.read_res::<Rc<Font>>().clone();
    let renderer = ecs.read_res::<Rc<Renderer>>().clone();
    let gold = ecs.read_res::<Supplies>().gold;
    ecs.write::<GuiText>(&e).unwrap().change_text(&renderer, format!("GOLD: {}", gold), font);
}

pub fn change_wood_text(ecs: &mut ECS) {
    let e = ecs.get2::<WoodText, GuiText>().first().unwrap().clone();
    let font = ecs.read_res::<Rc<Font>>().clone();
    let renderer = ecs.read_res::<Rc<Renderer>>().clone();
    let gold = ecs.read_res::<Supplies>().wood;
    ecs.write::<GuiText>(&e).unwrap().change_text(&renderer, format!("WOOD: {}", gold), font);
}

pub fn change_stone_text(ecs: &mut ECS) {
    let e = ecs.get2::<StoneText, GuiText>().first().unwrap().clone();
    let font = ecs.read_res::<Rc<Font>>().clone();
    let renderer = ecs.read_res::<Rc<Renderer>>().clone();
    let gold = ecs.read_res::<Supplies>().stone;
    ecs.write::<GuiText>(&e).unwrap().change_text(&renderer, format!("STONE: {}", gold), font);
}

pub struct UpdateResourceTextMutation {

}

impl Mutation for UpdateResourceTextMutation {
    fn apply(&self, ecs: &mut ECS) {
        change_gold_text(ecs);
        change_wood_text(ecs);
        change_stone_text(ecs);
    }
}

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
    mutations.push(Box::new(UpdateResourceTextMutation {}));
    for (capture, _, entity) in ecs.read2::<MouseCaptureArea, GuiComponent>() {
        mutations.push(Box::new(GuiHoveredMutation{entity, hovered: capture.is_hovered}));

        if capture.clicked_buttons[0] {
            println!("Clicked a button");
        }
    }
    return mutations;
}

#[derive(Debug)]
pub struct StageText {}

#[derive(Debug)]
pub struct GoldText {}

#[derive(Debug)]
pub struct WoodText {}

#[derive(Debug)]
pub struct StoneText {}


pub struct GameGui {}

impl GameGui {
    pub fn create(ecs: &mut ECS) {
        let mouse_layer = ecs.build_entity().add(MouseCaptureLayer {sort_index: 1, capture_areas: vec![]}).add(ActiveCaptureLayer {}).entity();
        let bottom_panel = create_bottom_panel(ecs, mouse_layer);
        let top_bar = create_top_bar(ecs);

        ecs.add_component(&bottom_panel, ActiveGui {});
        ecs.add_component(&top_bar, ActiveGui {});

    }
}

fn create_top_bar(ecs: &mut ECS) -> Entity {
    const TOP_BAR_PERCENT_HEIGHT: f32 = 0.03;
    let top_bar = GuiComponent::create_hoverable(
        ecs,
        None,
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

    let bounds = ecs.read::<GuiComponent>(&top_bar).unwrap().calculated_bounds;
    let font = ecs.read_res::<Rc<Font>>().clone();
    let gold_text = GuiText::create(
        ecs,
        Some(bounds),
        String::from("GOLD:"),
        font.clone(),
        false,
        true,
        16.,
        GuiConstraints::new(
            RelativeConstraint {size: 0.},
            CenterConstraint,
            RelativeConstraint { size: 0.12 },
            RelativeConstraint { size: 1. },
        ),
        Color::from_u8(0xEE, 0xD7, 0x00, 0xFF),
    );
    let wood_text = GuiText::create(
        ecs,
        Some(bounds),
        String::from("WOOD:"),
        font.clone(),
        false,
        true,
        16.,
        GuiConstraints::new(
            RelativeConstraint {size: 0.12},
            CenterConstraint,
            RelativeConstraint { size: 0.12 },
            RelativeConstraint { size: 1. },
        ),
        Color::from_u8(0xC1, 0x9A, 0x6B, 0xFF),
    );
    let stone_text = GuiText::create(
        ecs,
        Some(bounds),
        String::from("STONE:"),
        font.clone(),
        false,
        true,
        16.,
        GuiConstraints::new(
            RelativeConstraint {size: 0.24},
            CenterConstraint,
            RelativeConstraint { size: 0.12 },
            RelativeConstraint { size: 1. },
        ),
        Color::from_u8(0x88, 0x88, 0x88, 0xFF),
    );
    let stage_text = GuiText::create(
        ecs,
        Some(bounds),
        String::from("Stage"),
        font.clone(),
        true,
        true,
        16.,
        GuiConstraints::new(
            CenterConstraint,
            CenterConstraint,
            RelativeConstraint {size: 1.},
            RelativeConstraint {size: 1.}),
        Color::from_u8(0xFF, 0xFF, 0xFF, 0xFF),
    );
    ecs.add_component(&stage_text, StageText {});
    ecs.add_component(&gold_text, GoldText {});
    ecs.add_component(&wood_text, WoodText {});
    ecs.add_component(&stone_text, StoneText {});

    ecs.write::<GuiComponent>(&top_bar).unwrap()
        .add_text(stage_text)
        .add_text(gold_text)
        .add_text(wood_text)
        .add_text(stone_text);

    return top_bar;
}

fn create_bottom_panel(ecs: &mut ECS, mouse_layer: Entity) -> Entity {
    let bottom_panel_entity = GuiComponent::create(
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
    let buttons_box_entity = GuiComponent::create(
        ecs,
        None,
        Some(bottom_panel.calculated_bounds),
        GuiConstraints::new(
            Constraint::CenterConstraint,
            Constraint::CenterConstraint,
            Constraint::PixelConstraint {size: -15},
            Constraint::PixelConstraint {size: -15},
        ),
        2.,
        Color::from_u8(0x88, 0x88, 0x88, 0xFF));
    let buttons_box = ecs.read::<GuiComponent>(&buttons_box_entity).unwrap();
    let buttons_box_bounds = buttons_box.calculated_bounds.clone();
    let child1_entity = GuiComponent::create_hoverable(
        ecs,
        Some(mouse_layer),
        Some(buttons_box_bounds),
        GuiConstraints::new(
            Constraint::RelativeConstraint { size: 0.05 },
            Constraint::CenterConstraint,
            Constraint::AspectConstraint {aspect: 1.0},
            Constraint::PixelConstraint {size: -10},
        ),
        10.,
        Color::from_u8(0x55, 0x55, 0x55, 0xFF),
        Color::from_u8(0x38, 0x38, 0x38, 0xFF),
    );
    ecs.write::<MouseCaptureLayer>(&mouse_layer).unwrap().capture_areas.push(child1_entity);
    let buttons_box = ecs.write::<GuiComponent>(&buttons_box_entity).unwrap();
    buttons_box.add_child(child1_entity);
    let bottom_panel = ecs.write::<GuiComponent>(&bottom_panel_entity).unwrap();
    bottom_panel.add_child(buttons_box_entity);

    let child1 = ecs.read::<GuiComponent>(&child1_entity).unwrap();
    let child1_bounds = child1.calculated_bounds.clone();
    let image = PNG::from_file("bitmap/turret2.png").unwrap().image();
    let child_image1 = GuiImage::create(
        ecs,
        Some(child1_bounds),
        image,
        GuiConstraints::new(
            RelativeConstraint {size: 0.5},
            RelativeConstraint {size: 0.5},
            RelativeConstraint {size: 0.6},
            RelativeConstraint {size: 0.6},
        ),
    );
    let child1 = ecs.write::<GuiComponent>(&child1_entity).unwrap();
    child1.add_image(child_image1);
    return bottom_panel_entity;
}

