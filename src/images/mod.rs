use crate::types::Color;

pub mod bmp;
pub mod png;
pub mod spritesheet;

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>,
}

impl Image {
    pub fn raw_pixels(&self) -> Vec<u8> {
        let mut raw = vec![];
        for color in &self.data {
            raw.push((color.r * 255.) as u8);
            raw.push((color.g * 255.) as u8);
            raw.push((color.b * 255.) as u8);
            raw.push((color.a * 255.) as u8);
        }
        return raw;
    }
}

