#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
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
