use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use crate::rendering::buffers::{VertexBuffer, FragmentConstantBuffer};
use std::rc::Rc;
use crate::rendering::texture::RenderableTexture;
use crate::rendering::Scene;
use crate::png::PNG;
use crate::rendering::Renderer;
use crate::rendering::shader::Shader;


pub struct TextMeshCreator {

}

pub struct TextMeshData {
    pub positions: VertexBuffer<f32>,
    pub texture_coords: VertexBuffer<f32>,
}

#[derive(Debug)]
pub struct TextDrawable {
    pub text: String,
    pub font_size: u32,
    pub font: Rc<Font>,
    pub vertices: VertexBuffer<[f32; 6]>,
    pub shader: Shader,
    pub color: FragmentConstantBuffer<[f32; 4]>,
}

impl TextDrawable {
    pub fn new(renderer: &Renderer, position: [f32; 2], font: Rc<Font>, text: String, font_size: f32) -> Self {
        let mut vertices = vec![];

        let mut cursor = 0.;
        let mut base = position[1];
        let start = position[0];

        let max_line_length = 450.;
        let line_height = 100.;
        let scaling = 1./font.base_size * font_size / font.size;
        for text_char in text.chars() {
            let char_val = text_char as u32;
            let character = &font.characters[&char_val];

            let top = base - character.y_offset as f32 * scaling;
            let bottom = base - character.y_offset as f32 * scaling - character.height as f32 * scaling;
            let left = start + cursor + character.x_offset as f32 * scaling;
            let right = start + cursor + character.x_offset as f32 * scaling + character.width as f32 * scaling;

            vertices.push([left, top, character.x as f32, character.y as f32]);
            vertices.push([right, top, (character.x + character.width) as f32, character.y as f32]);
            vertices.push([left, bottom, character.x as f32, (character.y + character.height) as f32]);
            vertices.push([left, bottom, character.x as f32, (character.y + character.height) as f32]);
            vertices.push([right, top, (character.x + character.width) as f32, character.y as f32]);
            vertices.push([right, bottom, (character.x + character.width) as f32, (character.y + character.height) as f32]);

            cursor += character.xadvance as f32 * scaling;

            if cursor > max_line_length {
                base -= font.line_height;
                cursor = 0.;
            }
        }

        let mut adjusted_vertices = vec![];
        for [x, y, u, v] in vertices {
            println!("{} {}", u / 512., v / 512.);
            adjusted_vertices.push([x, y, 0.0, 1.0, u / 512., v / 512.]);
        }

        let vertices = VertexBuffer::new(renderer, 0, adjusted_vertices);

        let shader = Shader::new(renderer, true, "shaders/fontVertexShader.txt", "shaders/fontFragmentShader.txt");

        let color = FragmentConstantBuffer::new(renderer, 0, vec!([1.0, 1.0, 1.0, 0.0]));

        return TextDrawable {
            text,
            font_size: font_size as u32,
            font,
            vertices,
            shader,
            color,
        }
    }

    pub fn draw(&self, scene: &Scene) {
        self.shader.bind(scene);
        self.vertices.bind(scene);
        self.font.texture.bind(scene);
        self.color.bind(scene);
        scene.draw_triangles((self.text.len() * 6) as u64);
    }
}

pub struct RenderableFont {
    characters: HashMap<u32, RenderableCharacter>,
}

pub struct RenderableCharacter {
}

#[derive(Debug)]
pub struct Font {
    texture: RenderableTexture,
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

    pub fn new(renderer: &Renderer, font_file_path: &str, font_png_path: &str) -> Font {
        let font_file = File::open(font_file_path).unwrap();
        let mut font_file_reader = BufReader::new(font_file);

        let mut font_entries = Vec::new();
        let mut line_iter = font_file_reader.lines().filter_map(|result| result.ok());

        let info_line = line_iter.next().unwrap();
        let common_line = line_iter.next().unwrap();
        let page_line = line_iter.next().unwrap();
        let chars_line = line_iter.next().unwrap();

        let info = collect_elements(&info_line);
        let common = collect_elements(&common_line);

        while let Some(line) = line_iter.next() {
            font_entries.push(collect_elements(&line));
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

        let texture = PNG::from_file(font_png_path).unwrap();
        let texture = RenderableTexture::new_from_png(renderer, texture);

        Font {
            texture,
            characters,
            base_size: common.get("scaleW").unwrap().to_string().parse::<i32>().unwrap() as f32,
            size: info.get("size").unwrap().to_string().parse::<i32>().unwrap() as f32,
            line_height: common.get("lineHeight").unwrap().to_string().parse::<i32>().unwrap() as f32,
        }
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