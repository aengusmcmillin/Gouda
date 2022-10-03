use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use crate::rendering::buffers::{VertexBuffer, FragmentConstantBuffer};
use crate::shader_lib::font_shader::font_shader_layout;
use std::rc::Rc;
use crate::rendering::texture::RenderableTexture;
use crate::rendering::Scene;
use crate::png::PNG;
use crate::rendering::Renderer;

pub struct TextMeshCreator {

}

pub struct TextMeshData {
    pub positions: VertexBuffer,
    pub texture_coords: VertexBuffer
}

#[derive(Debug)]
pub struct TextDrawable {
    pub text: String,
    pub font_size: u32,
    pub font: &'static str,
    pub vertices: VertexBuffer,
    pub color: FragmentConstantBuffer,
}

impl TextDrawable {
    pub fn new(renderer: &Renderer, position: [f32; 2], size: [f32; 2], center_x: bool, center_y: bool, font_name: &'static str, color: [f32; 3], text: String, font_size: f32) -> Self {
        let font = renderer.font_lib.as_ref().unwrap().get(font_name).unwrap();
        let mut vertices = vec![];

        let scaling = 1./font.base_size * font_size / font.size;
        let mut cursor = 0.;
        let mut base = position[1] + size[1];
        let start = position[0];

        let max_line_length = size[0];
        let mut line_start_index = 0;
        let mut current_char_index = 0;
        let line_height = font.line_height * scaling;
        for text_char in text.chars() {
            let char_val = text_char as u32;
            let character = &font.characters[&char_val];

            let top = base - character.y_offset as f32 * scaling;
            let bottom = top - character.height as f32 * scaling;
            let left = start + cursor + character.x_offset as f32 * scaling;
            let right = left + character.width as f32 * scaling;

            vertices.push([left, top, character.x as f32, character.y as f32]);
            vertices.push([right, top, (character.x + character.width) as f32, character.y as f32]);
            vertices.push([left, bottom, character.x as f32, (character.y + character.height) as f32]);
            vertices.push([left, bottom, character.x as f32, (character.y + character.height) as f32]);
            vertices.push([right, top, (character.x + character.width) as f32, character.y as f32]);
            vertices.push([right, bottom, (character.x + character.width) as f32, (character.y + character.height) as f32]);

            current_char_index += 6;

            cursor += character.xadvance as f32 * scaling;

            if cursor > max_line_length {
                if center_x {
                    let line_width = cursor;
                    let offset = size[0] - line_width;
                    let offset = offset / 2.;

                    for i in line_start_index..current_char_index {
                        vertices[i][0] += offset;
                    }
                }

                line_start_index = current_char_index;

                base -= line_height;
                cursor = 0.;
            }
        }

        if center_x {
            let line_width = cursor;
            let offset = size[0] - line_width;
            let offset = offset / 2.;

            for i in line_start_index..current_char_index {
                vertices[i][0] += offset;
            }
        }

        if center_y {
            let text_height = position[1] + size[1] - base + line_height;
            let offset = size[1] - text_height;
            let offset = offset / 2.;

            for i in 0..vertices.len() {
                vertices[i][1] -= offset;
            }
        }

        let mut adjusted_vertices = vec![];
        for [x, y, u, v] in vertices {
            adjusted_vertices.push([x, y, 0.0, 1.0, u / 512., v / 512.]);
        }

        let vertices = VertexBuffer::new(renderer, font_shader_layout(), 0, adjusted_vertices);

        let color = FragmentConstantBuffer::new(renderer, 0, vec!([color[0], color[1], color[2], 0.0]));

        return TextDrawable {
            text,
            font_size: font_size as u32,
            font: font_name,
            vertices,
            color,
        }
    }

    pub fn draw(&self, scene: &Scene) {
        scene.bind_shader("font");
        self.vertices.bind(scene);
        scene.bind_font(self.font);
        self.color.bind(scene);
        scene.draw_triangles((self.text.len() * 6) as u64);
    }
}

pub struct RenderableCharacter {
}

#[derive(Debug)]
pub struct Font {
    pub texture: RenderableTexture,
    characters: HashMap<u32, Character>,
    base_size: f32,
    size: f32,
    line_height: f32,
}

fn collect_elements(line: &String) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let lines: Vec<&str> = line.split_whitespace().collect();
    for line in lines {
        let split: Vec<&str> = line.split("=").collect();
        if split.len() == 2 {
            result.insert(String::from(split[0]), String::from(split[1]));
        }
    }
    return result;
}

impl Font {
    fn new_impl(texture: RenderableTexture, line_iter: &mut dyn Iterator<Item = String>) -> Font {
        let mut font_entries = Vec::new();
        let info_line = line_iter.next().unwrap();
        let common_line = line_iter.next().unwrap();
        line_iter.next().unwrap();
        line_iter.next().unwrap();

        let info = collect_elements(&info_line.to_string());
        let common = collect_elements(&common_line.to_string());

        while let Some(line) = line_iter.next() {
            font_entries.push(collect_elements(&line.to_string()));
        }

        let mut characters = HashMap::new();
        for entry in font_entries.iter() {
            let character = Character {
                id: entry.get("id").unwrap().to_string().parse::<u32>().unwrap(),
                x: entry.get("x").unwrap().to_string().parse::<i32>().unwrap(),
                y: entry.get("y").unwrap().to_string().parse::<i32>().unwrap(),
                width: entry.get("width").unwrap().to_string().parse::<i32>().unwrap(),
                height: entry.get("height").unwrap().to_string().parse::<i32>().unwrap(),
                x_offset: entry.get("xoffset").unwrap().to_string().parse::<i32>().unwrap(),
                y_offset: entry.get("yoffset").unwrap().to_string().parse::<i32>().unwrap(),
                xadvance: entry.get("xadvance").unwrap().to_string().parse::<i32>().unwrap(),
            };
            characters.insert(character.id, character);
        }

        Font {
            texture,
            characters,
            base_size: common.get("scaleW").unwrap().to_string().parse::<i32>().unwrap() as f32,
            size: info.get("size").unwrap().to_string().parse::<i32>().unwrap() as f32,
            line_height: common.get("lineHeight").unwrap().to_string().parse::<i32>().unwrap() as f32,
        }
    }

    pub fn new_from_contents(renderer: &Renderer, font_img: &[u8], font_file: &str) -> Font {
        let texture = PNG::from_buffer(font_img).unwrap().image();
        let texture = RenderableTexture::new(renderer, &texture);

        let mut line_iter = font_file.lines().map(|item| item.to_string());
        return Font::new_impl(texture, &mut line_iter);
    }

    pub fn new(renderer: &Renderer, font_file_path: &str, font_png_path: &str) -> Font {
        let font_file = File::open(font_file_path).unwrap();
        let font_file_reader = BufReader::new(font_file);
        
        let texture = PNG::from_file(font_png_path).unwrap().image();
        let texture = RenderableTexture::new(renderer, &texture);

        let mut line_iter = font_file_reader.lines().filter_map(|result| result.ok());
        return Font::new_impl(texture, &mut line_iter);
    }
}

#[derive(Debug)]
pub struct Character {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub xadvance: i32,
}