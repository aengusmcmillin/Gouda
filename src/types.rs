#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    Top,
    Right,
    Down,
    Left
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r as f32 / 255.,
            g: g as f32 / 255.,
            b: b as f32 / 255.,
            a: a as f32 / 255.,
        }
    }
    pub fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {r, g, b, a}
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Bounds {
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct ScreenPosition {
    
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub w: f32,
    pub h: f32,
    pub d: f32,
}
