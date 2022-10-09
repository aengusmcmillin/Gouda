use cgmath::{SquareMatrix, Vector4};

use gouda_rendering::camera::{OrthographicCamera, Camera};
use gouda_ecs::{Mutations, ECS, Entity, Mutation};
use crate::input::GameInput;
use gouda_types::Bounds;

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
    pub is_gui: bool,
    pub down_buttons: [bool; 5],
    pub clicked_buttons: [bool; 5],
    pub released_buttons: [bool; 5],
    pub bounds: Bounds,
}

impl MouseCaptureArea {
    pub fn new(is_gui: bool, bounds: Bounds) -> MouseCaptureArea {
        MouseCaptureArea {
            is_hovered: false,
            is_gui: is_gui,
            down_buttons: [false; 5],
            clicked_buttons: [false; 5],
            released_buttons: [false; 5],
            bounds,
        }
    }
}

#[derive(Debug)]
pub struct HexMouseCaptureArea {
    pub is_hovered: bool,
    pub down_buttons: [bool; 5],
    pub clicked_buttons: [bool; 5],
    pub released_buttons: [bool; 5],
    pub center: [f32; 2],
    pub size: f32,
}

impl HexMouseCaptureArea {
    pub fn new(center: [f32; 2], size: f32) -> HexMouseCaptureArea {
        HexMouseCaptureArea {
            is_hovered: false,
            down_buttons: [false; 5],
            clicked_buttons: [false; 5],
            released_buttons: [false; 5],
            center,
            size
        }
    }

    pub fn overlaps_mouse(&self, mouse_pos: [f32; 2]) -> bool {
        if (mouse_pos[0] - self.center[0]).abs() > self.size {
            return false;
        }
        if (mouse_pos[1] - self.center[1]).abs() > self.size {
            return false;
        }
        return true;
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
        let area = ecs.write::<MouseCaptureArea>(&self.area);
        if let Some(area) = area {
            area.is_hovered = true;
            area.down_buttons = self.down_buttons;
            area.clicked_buttons = self.clicked_buttons;
            area.released_buttons = self.released_buttons;
        }

        let hex_area = ecs.write::<HexMouseCaptureArea>(&self.area);
        if let Some(hex_area) = hex_area {
            hex_area.is_hovered = true;
            hex_area.down_buttons = self.down_buttons;
            hex_area.clicked_buttons = self.clicked_buttons;
            hex_area.released_buttons = self.released_buttons;
        }
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

        let to_clear: Vec<Entity> = ecs.get1::<HexMouseCaptureArea>().iter().filter(|&&e| {
            if let Some(excluded) = self.excluded {
                return e != excluded;
            }
            return true;
        }).map(|&e| e.clone()).collect();

        for e in to_clear {
            let area = ecs.write::<HexMouseCaptureArea>(&e).unwrap();
            area.is_hovered = false;
            area.down_buttons = [false; 5];
            area.clicked_buttons = [false; 5];
            area.released_buttons = [false; 5];
        }
    }
}


pub fn mouse_capture_system(ecs: &ECS, _dt: f32) -> Mutations {
    let camera = ecs.read1::<OrthographicCamera>()[0].0;

    let mut layers = ecs.read2::<MouseCaptureLayer, ActiveCaptureLayer>();
    layers.sort_by(|a, b| b.0.sort_index.cmp(&a.0.sort_index));


    let input = ecs.read_res::<GameInput>();
    let mouse_x = input.mouse.x;
    let mouse_y = 900 - input.mouse.y;
    let screen_mouse_x = mouse_x as f32 / 450. - 1.;
    let screen_mouse_y = mouse_y as f32 / 450. - 1.;
    let mut mouse_world_pos = camera.get_view_projection_matrix().invert().unwrap() * Vector4::new(screen_mouse_x, screen_mouse_y, 0., 1.);
    mouse_world_pos.w = 1.0 / mouse_world_pos.w;
    mouse_world_pos.x /= mouse_world_pos.w;
    mouse_world_pos.y /= mouse_world_pos.w;
    mouse_world_pos.z /= mouse_world_pos.w;

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

    for (layer, _, _) in layers {
        for area_e in &layer.capture_areas {
            let area = ecs.read::<MouseCaptureArea>(&area_e);
            if let Some(area) = area {
                let (x, y) = if area.is_gui {
                    (mouse_x as f32, mouse_y as f32)
                } else {
                    (mouse_world_pos.x, mouse_world_pos.y)
                };
                if area.bounds.contains_point(x, y) {
                    return vec![
                        Box::new(MouseCaptureMutation {area: area_e.clone(), down_buttons, clicked_buttons, released_buttons}), 
                        Box::new(ClearOthersMutation {excluded: Some(area_e.clone())})
                    ];
                }
            }

            let hex_area = ecs.read::<HexMouseCaptureArea>(&area_e);
            if let Some(hex_area) = hex_area {
                if hex_area.overlaps_mouse([mouse_world_pos.x as f32, mouse_world_pos.y as f32]) {
                    return vec![
                        Box::new(MouseCaptureMutation {area: area_e.clone(), down_buttons, clicked_buttons, released_buttons}), 
                        Box::new(ClearOthersMutation {excluded: Some(area_e.clone())})
                    ];
                }
            }
        }
    }

    return vec![Box::new(ClearOthersMutation {excluded: None})];
}