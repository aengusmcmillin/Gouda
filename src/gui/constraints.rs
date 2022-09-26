use crate::types::Bounds;

#[derive(Debug)]
pub struct GuiConstraints {
    x: Constraint,
    y: Constraint,
    width: Constraint,
    height: Constraint,
}

impl GuiConstraints {
    pub fn new(x: Constraint, y: Constraint, width: Constraint, height: Constraint) -> GuiConstraints {
        GuiConstraints {
            x,
            y,
            width,
            height
        }
    }

    pub fn calculate_bounds(&self, parent: Bounds) -> Bounds {
        let mut x = self.x.calculate(parent.x, parent.x + parent.w);
        let mut y = self.y.calculate(parent.y, parent.y + parent.h);
        let mut w = self.width.calculate(0., parent.w);
        let mut h = self.height.calculate(0., parent.h);

        if w == 0. && h == 0. {
            panic!("Bad width and height");
        } else if let Constraint::AspectConstraint {aspect} = self.width {
            w = h * aspect;
        } else if let Constraint::AspectConstraint {aspect} = self.height {
            h = w * aspect;
        }

        if let Constraint::CenterConstraint = self.x {
            x -= w * 0.5;
        }
        if let Constraint::CenterConstraint = self.y {
            y -= h * 0.5;
        }

        return Bounds {
            x,
            y,
            w,
            h
        }
    }
}

#[derive(Debug)]
pub enum Constraint {
    RelativeConstraint{size: f32},
    CenterConstraint,
    PixelConstraint{size: i32},
    AspectConstraint{aspect: f32},
}

impl Constraint {
    pub fn calculate(&self, min: f32, max: f32) -> f32 {
        match self {
            Constraint::RelativeConstraint { size } => {
                min + (max - min) * size
            }
            Constraint::CenterConstraint => {
                (min + max) / 2.
            }
            Constraint::PixelConstraint { size } => {
                if *size < 0 {
                    max - min + *size as f32
                } else {
                    *size as f32
                }
            }
            Constraint::AspectConstraint { aspect: _ } => {
                0.
            }
        }
    }
}
