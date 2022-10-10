use std::collections::HashMap;

use crate::shaders::Shader;
use crate::{Renderer, Scene};

use self::basic_shader::basic_shader;
use self::font_shader::font_shader;
use self::gui_shader::gui_shader;
use self::hex_shader::hex_shader;
use self::imgui_shader::imgui_shader;
use self::obj_model_shader::obj_model_shader;
use self::quad_shader::quad_shader;
use self::texture_shader::texture_shader;

pub mod basic_shader;
pub mod font_shader;
pub mod gui_shader;
pub mod hex_shader;
pub mod imgui_shader;
pub mod obj_model_shader;
pub mod quad_shader;
pub mod texture_shader;

#[derive(Debug)]
pub struct ShaderLibrary {
    shaders: HashMap<&'static str, Shader>,
}

impl ShaderLibrary {
    pub fn new() -> ShaderLibrary {
        return ShaderLibrary {
            shaders: HashMap::new(),
        };
    }

    pub fn initialize(&mut self, renderer: &mut Renderer) {
        self.add("basic", basic_shader(renderer));
        self.add("font", font_shader(renderer));
        self.add("gui", gui_shader(renderer));
        self.add("imgui", imgui_shader(renderer));
        self.add("quad", quad_shader(renderer));
        self.add("texture", texture_shader(renderer));
        self.add("hex", hex_shader(renderer));
        self.add("obj_model", obj_model_shader(renderer));
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
