extern crate core;
use crate::utils::{u16_from_bytes, u32_from_bytes};


use std::fs::File;
use std::io::prelude::*;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl Color {
    pub fn raw(self) -> u32 {
        u32_from_bytes([self.a, self.r, self.g, self.b])
    }
}

#[repr(packed)]
#[derive(Debug)]
pub struct BitmapHeader {
    pub file_type: u16,
    pub file_size: u32,
    pub r1: u16,
    pub r2: u16,
    pub bmp_offset: u32,

    pub size: u32,
    pub width: u32,
    pub height: u32,
    pub planes: u16,
    pub bpp: u16,
    pub compression: u32,
    pub size_of_bmp: u32,
    pub horz_res: i32,
    pub vert_res: i32,
    pub colors_used: u32,
    pub colors_imp: u32,

    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub alpha_mask: u32,
}

#[derive(Debug)]
pub struct Bitmap {
    pub header: BitmapHeader,
    pub contents: Vec<Color>,
}

impl Bitmap {
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.contents[(self.header.width * y + x) as usize]
    }
}

pub fn debug_load_bmp<'c>(path: &str) -> Option<Bitmap> {
    let mut file = File::open(path);
    if let Ok(mut file) = file {
        let mut c = Vec::new();
        file.read_to_end(&mut c);
        let header = BitmapHeader {
            file_type: u16_from_bytes([c[1], c[0]]),
            file_size: u32_from_bytes([c[5], c[4], c[3], c[2]]),
            r1: u16_from_bytes([c[7], c[6]]),
            r2: u16_from_bytes([c[9], c[8]]),
            bmp_offset: u32_from_bytes([c[13], c[12], c[11], c[10]]),
            size: u32_from_bytes([c[17], c[16], c[15], c[14]]),
            width: u32_from_bytes([c[21], c[20], c[19], c[18]]),
            height: u32_from_bytes([c[25], c[24], c[23], c[22]]),
            planes: u16_from_bytes([c[27], c[26]]),
            bpp: u16_from_bytes([c[29], c[28]]),
            compression: u32_from_bytes([c[33], c[32], c[31], c[30]]),
            size_of_bmp: u32_from_bytes([c[37], c[36], c[35], c[34]]),
            horz_res: u32_from_bytes([c[41], c[40], c[39], c[38]]) as i32,
            vert_res: u32_from_bytes([c[45], c[44], c[43], c[42]]) as i32,
            colors_used: u32_from_bytes([c[49], c[48], c[47], c[46]]),
            colors_imp: u32_from_bytes([c[53], c[52], c[51], c[50]]),

            red_mask: u32_from_bytes([c[57], c[56], c[55], c[54]]),
            green_mask: u32_from_bytes([c[61], c[60], c[59], c[58]]),
            blue_mask: u32_from_bytes([c[65], c[64], c[63], c[62]]),
            alpha_mask: u32_from_bytes([c[69], c[68], c[67], c[66]]),
        };

        let offset = header.bmp_offset as usize;

        let mut res = Vec::new();
        for i in (0..header.size_of_bmp - 2).filter(|&x| x % 3 == 0) {
            res.push(Color {
                r: c[offset + i as usize + 2],
                g: c[offset + i as usize + 1],
                b: c[offset + i as usize + 0],
                a: 255,
            });
        }

        let mut flipped = Vec::new();
        for y in (0..header.height).rev() {
            for x in 0..header.width {
                flipped.push(res[(y * header.width + x) as usize]);
            }
        }

        return Some(Bitmap {
            header,
            contents: flipped,
        });
    }
    return None;
}
