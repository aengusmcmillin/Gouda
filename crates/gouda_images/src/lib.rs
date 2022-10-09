use gouda_types::Color;

pub mod bmp;
pub mod png;
pub mod spritesheet;
pub mod utils;

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

    pub fn data_from_raw_pixels(w: usize, h: usize, data: Vec<u8>) -> Vec<Color> {
        let mut res = vec![];
        for y in 0..h {
            for x in 0..w {
                let r = data[y * w * 4 + x * 4];
                let g = data[y * w * 4 + x * 4 + 1];
                let b = data[y * w * 4 + x * 4 + 2];
                let a = data[y * w * 4 + x * 4 + 3];
                res.push(Color::from_u8(r, g, b, a));
            }
        }
        return res;
    }
}
