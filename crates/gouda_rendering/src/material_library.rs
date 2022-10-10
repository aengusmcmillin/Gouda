use std::collections::HashMap;

pub type MaterialKey = String;
pub type ShaderKey = String;

pub struct Material {
    shader: ShaderKey,
}

pub struct MaterialLibrary {
    pub materials: HashMap<MaterialKey, Material>,
}
