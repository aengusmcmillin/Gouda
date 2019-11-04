extern crate core;
use crate::utils::{u16_from_bytes, u32_from_bytes};


use std::fs::File;
use std::io::prelude::*;

pub struct Chunk {
    length: u32,
    chunk_type: String,
    chunk_data: Vec<u8>,
    crc: u32,
}

pub struct PNGHeader {
    pub width: u32,
    pub height: u32,
}

pub struct PNG {
    pub header_chunk: PNGHeader,
    pub data: Vec<u8>,
}

impl PNG {
    pub fn from_file(path: &str) -> Option<PNG> {
        let mut file = File::open(path);
        if let Ok(mut file) = file {
            let mut c = Vec::new();
            file.read_to_end(&mut c);

            let mut i = 8;

            let mut header = PNGHeader {width: 0, height: 0};
            let mut data_chunks = vec![];
            loop {
                let chunk = parse_chunk(&c, i);
                i += 12 + chunk.length as usize;
                if chunk.chunk_type == "IEND" {
                    break;
                } else if chunk.chunk_type == "IDAT" {
                    data_chunks.push(chunk);
                } else if chunk.chunk_type == "IHDR" {
                    let hdrdata = &chunk.chunk_data;
                    let width = u32_from_bytes([hdrdata[0], hdrdata[1], hdrdata[2], hdrdata[3]]);
                    let height = u32_from_bytes([hdrdata[4], hdrdata[5], hdrdata[6], hdrdata[7]]);
                    header.width = width;
                    header.height = height;
                }
            }

            let mut data_bytes = vec![];

            for chunk in data_chunks {
                for byte in chunk.chunk_data {
                    data_bytes.push(byte);
                }
            }

            let mut decoder = compress::zlib::Decoder::new(data_bytes.as_slice());
            let mut decompressed = Vec::new();
            let result = decoder.read_to_end(&mut decompressed);
            if let Err(E) = result {
                println!("Error: {}", E);
            }

            let mut result_bytes = vec![];
            let row_len = (header.width * 4) + 1;
            let result_row_len = (header.width * 4) as usize;
            for y in 0..header.height {
                let filter_method = decompressed[(y * row_len) as usize];
                for x in 0..header.width  {
                    let index = (y * (row_len) + 1 + x * 4) as usize;
                    let rindex = (y * (result_row_len as u32) + x * 4) as usize;
                    let bpp = 4;
                    let raw_r = decompressed[index];
                    let raw_g = decompressed[index + 1];
                    let raw_b = decompressed[index + 2];
                    let raw_a = decompressed[index + 3];

                    let (old_raw_r, old_raw_g, old_raw_b, old_raw_a) = if x > 0 {
                        (result_bytes[rindex - bpp], result_bytes[rindex + 1 - bpp], result_bytes[rindex + 2 - bpp], result_bytes[rindex + 3 - bpp])
                    } else {
                        (0, 0, 0, 0)
                    };

                    let (prior_r, prior_g, prior_b, prior_a) = if y > 0 {
                        (
                            result_bytes[rindex - result_row_len],
                            result_bytes[rindex + 1 - result_row_len],
                            result_bytes[rindex + 2 - result_row_len],
                            result_bytes[rindex + 3 - result_row_len]
                        )
                    } else {
                        (0, 0, 0, 0)
                    };

                    if filter_method == 1 {
                        result_bytes.push(raw_r.wrapping_add(old_raw_r));
                        result_bytes.push(raw_g.wrapping_add(old_raw_g));
                        result_bytes.push(raw_b.wrapping_add(old_raw_b));
                        result_bytes.push(raw_a.wrapping_add(old_raw_a));
                    } else if filter_method == 2 {
                        result_bytes.push(raw_r.wrapping_add(prior_r));
                        result_bytes.push(raw_g.wrapping_add(prior_g));
                        result_bytes.push(raw_b.wrapping_add(prior_b));
                        result_bytes.push(raw_a.wrapping_add(prior_a));
                    } else if filter_method == 3 {
                        let f_r = ((old_raw_r.wrapping_add(prior_r)) as f32 / 2.0).floor();
                        result_bytes.push(f_r as u8);
                        let f_g = ((old_raw_g.wrapping_add(prior_g)) as f32 / 2.0).floor();
                        result_bytes.push(f_g as u8);
                        let f_b = ((old_raw_b.wrapping_add(prior_b)) as f32 / 2.0).floor();
                        result_bytes.push(f_b as u8);
                        let f_a = ((old_raw_a.wrapping_add(prior_a)) as f32 / 2.0).floor();
                        result_bytes.push(f_a as u8);
                    } else {
                        result_bytes.push(raw_r);
                        result_bytes.push(raw_g);
                        result_bytes.push(raw_b);
                        result_bytes.push(raw_a);
                    }
                }
            }
            return Some(PNG {
                header_chunk: header,
                data: result_bytes,
            });
        }
        return None;
    }
}

fn parse_chunk(c: &Vec<u8>, i: usize) -> Chunk {
    let length = u32_from_bytes([c[i], c[i + 1], c[i + 2], c[i + 3]]);
    let mut chunk_type = String::from("");
    chunk_type.push(c[i + 4].into());
    chunk_type.push(c[i + 5].into());
    chunk_type.push(c[i + 6].into());
    chunk_type.push(c[i + 7].into());

    let mut bytes = vec![];
    for j in 0..length {
        let byte = c[i + 8 + j as usize];
        bytes.push(byte);
    }

    return Chunk {
        length,
        chunk_type,
        chunk_data: bytes,
        crc: 0,
    }
}