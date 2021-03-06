use crate::ecs::{Mutations, ECS, Entity, Mutation};
use crate::input::GameInput;
use crate::types::Bounds;

#[derive(Debug)]
pub struct ActiveCaptureLayer {}

#[derive(Debug)]
pub struct MouseCaptureLayer {
    pub sort_index: u32,
    pub capture_areas: Vec<Entity>,
}

#[derive(Debug)]
pub struct MouseCaptureArea {
    pub is_hovered: bool,
    pub down_buttons: [bool; 5],
    pub clicked_buttons: [bool; 5],
    pub released_buttons: [bool; 5],
    pub bounds: Bounds,
}

impl MouseCaptureArea {
    pub fn new(bounds: Bounds) -> MouseCaptureArea {
        MouseCaptureArea {
            is_hovered: false,
            down_buttons: [false; 5],
            clicked_buttons: [false; 5],
            released_buttons: [false; 5],
            bounds,
        }
    }
}

pub struct MouseCaptureMutation {
    area: Entity,
    down_buttons: [bool; 5],
    clicked_buttons: [bool; 5],
    released_buttons: [bool; 5],
}

impl Mutation for MouseCaptureMutation {
    fn apply(&self, ecs: &mut ECS) {
        let area = ecs.write::<MouseCaptureArea>(&self.area).unwrap();
        area.is_hovered = true;
        area.down_buttons = self.down_buttons;
        area.clicked_buttons = self.clicked_buttons;
        area.released_buttons = self.released_buttons;
    }
}

pub struct ClearOthersMutation {
    excluded: Option<Entity>,
}

impl Mutation for ClearOthersMutation {
    fn apply(&self, ecs: &mut ECS) {
        let to_clear: Vec<Entity> = ecs.get1::<MouseCaptureArea>().iter().filter(|&&e| {
            if let Some(excluded) = self.excluded {
                return e != excluded;
            }
            return true;
        }).map(|&e| e.clone()).collect();

        for e in to_clear {
            let area = ecs.write::<MouseCaptureArea>(&e).unwrap();
            area.is_hovered = false;
            area.down_buttons = [false; 5];
            area.clicked_buttons = [false; 5];
            area.released_buttons = [false; 5];
        }
    }
}


pub fn mouse_capture_system(ecs: &ECS) -> Mutations {
    let mut layers = ecs.read2::<MouseCaptureLayer, ActiveCaptureLayer>();
    layers.sort_by(|a, b| b.0.sort_index.cmp(&a.0.sort_index));

    let input = ecs.read_res::<GameInput>();
    let mouse_x = input.mouse.x;
    let mouse_y = 900 - input.mouse.y;
    let down_buttons = [
        input.mouse.buttons[0].ended_down,
        input.mouse.buttons[1].ended_down,
        input.mouse.buttons[2].ended_down,
        input.mouse.buttons[3].ended_down,
        input.mouse.buttons[4].ended_down,
    ];
    let clicked_buttons = [
        input.mouse.buttons[0].ended_down && input.mouse.buttons[0].half_transition_count > 0,
        input.mouse.buttons[1].ended_down && input.mouse.buttons[1].half_transition_count > 0,
        input.mouse.buttons[2].ended_down && input.mouse.buttons[2].half_transition_count > 0,
        input.mouse.buttons[3].ended_down && input.mouse.buttons[3].half_transition_count > 0,
        input.mouse.buttons[4].ended_down && input.mouse.buttons[4].half_transition_count > 0,
    ];
    let released_buttons = [
        !input.mouse.buttons[0].ended_down && input.mouse.buttons[0].half_transition_count > 0,
        !input.mouse.buttons[1].ended_down && input.mouse.buttons[1].half_transition_count > 0,
        !input.mouse.buttons[2].ended_down && input.mouse.buttons[2].half_transition_count > 0,
        !input.mouse.buttons[3].ended_down && input.mouse.buttons[3].half_transition_count > 0,
        !input.mouse.buttons[4].ended_down && input.mouse.buttons[4].half_transition_count > 0,
    ];

    for (layer, _, e) in layers {
        for area_e in &layer.capture_areas {
            let area = ecs.read::<MouseCaptureArea>(&area_e).unwrap();
            if area.bounds.contains_point(mouse_x, mouse_y) {
                return vec![Box::new(MouseCaptureMutation {area: area_e.clone(), down_buttons, clicked_buttons, released_buttons}), Box::new(ClearOthersMutation {excluded: Some(area_e.clone())})];
            }
        }
    }

    return vec![Box::new(ClearOthersMutation {excluded: None})];
}