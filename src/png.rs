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
                println!("{}", chunk.chunk_type);
                i += 12 + chunk.length as usize;
                if chunk.chunk_type == "IEND" {
                    break;
                } else if chunk.chunk_type == "IDAT" {
                    data_chunks.push(chunk);
                } else if chunk.chunk_type == "IHDR" {
                    for byte in &chunk.chunk_data {
                        print!{"{:x}", byte};
                    }
                    let hdrdata = &chunk.chunk_data;
                    let width = u32_from_bytes([hdrdata[0], hdrdata[1], hdrdata[2], hdrdata[3]]);
                    let height = u32_from_bytes([hdrdata[4], hdrdata[5], hdrdata[6], hdrdata[7]]);
                    header.width = width;
                    header.height = height;
                    println!();
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
            match result {
                Err(E) => {
                    println!("Error: {}", E)
                }
                Ok(count) => {
                    println!("Count: {}", count)
                }
            }
            println!("Decoded length: {}", decompressed.len());

            let mut result_bytes = vec![];
            let row_len = (header.width * 4) + 1;
            for y in 0..header.height {
                for x in 0..header.width  {
                    let index = (y * (row_len) + 1 + x * 4) as usize;
                    result_bytes.push(decompressed[index]);
                    result_bytes.push(decompressed[index + 1]);
                    result_bytes.push(decompressed[index + 2]);
                    result_bytes.push(decompressed[index + 3]);
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