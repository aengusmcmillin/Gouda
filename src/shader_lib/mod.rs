use std::collections::HashMap;

use crate::rendering::Scene;
use crate::rendering::{shader::Shader, Renderer};

use self::basic_shader::basic_shader;
use self::font_shader::font_shader;
use self::gui_shader::gui_shader;
use self::quad_shader::quad_shader;
use self::texture_shader::texture_shader;
use self::hex_shader::hex_shader;

pub mod basic_shader;
pub mod font_shader;
pub mod gui_shader;
pub mod quad_shader;
pub mod texture_shader;
pub mod hex_shader;


#[derive(Debug)]
pub struct ShaderLibrary {
    shaders: HashMap<&'static str, Shader>
}

impl ShaderLibrary {
    pub fn new() -> ShaderLibrary {
        return ShaderLibrary { shaders: HashMap::new() };
    }

    pub fn construct(renderer: &Renderer) -> ShaderLibrary {
        let mut lib = ShaderLibrary::new();
        lib.add("basic", basic_shader(renderer));
        lib.add("font", font_shader(renderer));
        lib.add("gui", gui_shader(renderer));
        lib.add("quad", quad_shader(renderer));
        lib.add("texture", texture_shader(renderer));
        lib.add("hex", hex_shader(renderer));
        return lib;
    }

    pub fn add(&mut self, name: &'static str, shader: Shader) {
        self.shaders.insert(name, shader);
    }

    pub fn get(&self, name: &'static str) -> Option<&Shader> {
        return self.shaders.get(&name);
    }

    pub fn bind_shader(&self, scene: &Scene, name: &'static str) {
        self.get(name).unwrap().bind(scene);
    }
}
