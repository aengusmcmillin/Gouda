use gouda::ecs::{ECS, Entity};
use gouda::gui::{GuiComponent, ActiveGui};
use gouda::types::Color;
use std::rc::Rc;
use gouda::rendering::Renderer;
use gouda::gui::constraints::{Constraint, GuiConstraints};


pub struct GameGui {

}

impl GameGui {
    pub fn create(ecs: &mut ECS) {
        println!("Creating gui");
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
            Color::from_u8(0x22, 0x22, 0x22, 0xFF));
        let mut sub_component = GuiComponent::new(
            renderer,
            Some(component.calculated_bounds),
            GuiConstraints::new(
                Constraint::CenterConstraint,
                Constraint::CenterConstraint,
                Constraint::PixelConstraint {size: -15},
                Constraint::PixelConstraint {size: -15},
            ),
            8.,
            Color::from_u8(0x44, 0x44, 0x44, 0xFF));
        sub_component.add_child(GuiComponent::new(
            renderer,
            Some(sub_component.calculated_bounds),
            GuiConstraints::new(
                Constraint::RelativeConstraint { size: 0.05 },
                Constraint::CenterConstraint,
                Constraint::AspectConstraint {aspect: 1.0},
                Constraint::PixelConstraint {size: -10},
            ),
            20.,
            Color::from_u8(0x88, 0x22, 0x33, 0xFF)));
        sub_component.add_child(GuiComponent::new(
            renderer,
            Some(sub_component.calculated_bounds),
            GuiConstraints::new(
                Constraint::RelativeConstraint { size: 0.25 },
                Constraint::CenterConstraint,
                Constraint::AspectConstraint {aspect: 1.0},
                Constraint::PixelConstraint {size: -10},
            ),
            20.,
            Color::from_u8(0x88, 0x22, 0x33, 0xFF)));
        component.add_child(sub_component);
        ecs.build_entity().add(component).add(ActiveGui{});

    }
}

fn create_top_panel(renderer: &Renderer) -> GuiComponent {
    let mut top = GuiComponent::new(
        renderer,
        None,
        GuiConstraints::new(
            Constraint::CenterConstraint,
            Constraint::PixelConstraint { size: -30 },
            Constraint::RelativeConstraint {size: 1.},
            Constraint::PixelConstraint {size: 30}
        ),
        0.,
        Color::from_u8(0x22, 0x22, 0x22, 0xFF));

    top.add_child(GuiComponent::new(
        renderer,
        Some(top.calculated_bounds),
        GuiConstraints::new(
            Constraint::RelativeConstraint {size: 0.05},
            Constraint::CenterConstraint,
            Constraint::PixelConstraint {size: 100},
            Constraint::RelativeConstraint {size: 0.9},
        ),
        15.,
        Color::from_u8(0x99, 0x99, 0x99, 0xFF)));

    return top;
}

