use std::collections::HashMap;

use crate::obj::{MtlFile, MtlMaterialDefinition};
use crate::shaders::ShaderUniform;

pub type MaterialKey = String;
pub type ShaderKey = String;

pub struct Material {
    name: String,
    shader: ShaderKey,
    ambient_color: [f32; 3],
    diffuse_color: [f32; 3],
    specular_color: [f32; 3],
    emission_color: [f32; 3],
    specular_highlights: f32,
    optical_density: f32,
    dissolve: f32,
    illumination: u8,
}

pub struct MaterialLibrary {
    pub materials: HashMap<MaterialKey, Material>,
}

impl MaterialLibrary {}

impl Material {
    pub fn from_mtl(mtl_material_definition: MtlMaterialDefinition) {
        // match mtl_material_definition.illumination {
        //     0 => {}
        //     1 => {}
        //     2 => {}
        //     3 => {}
        //     4 => {}
        //     5 => {}
        // }
    }
}
