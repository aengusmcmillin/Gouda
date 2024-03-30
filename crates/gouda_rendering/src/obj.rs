use std::collections::HashMap;
use std::error::Error;
use std::fs::{self};
use std::io::BufRead;

use crate::shader_lib::obj_model_shader::obj_model_shader_layout;
use crate::Renderer;

use super::buffers::{IndexBuffer, VertexBuffer};

#[repr(packed(1))]
pub struct Vert {
    pub pos: [f32; 4],
    pub tex: [f32; 2],
    pub normal: [f32; 3],
}

pub struct ObjMeshSubset {
    pub index_buffer: IndexBuffer,
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
}

pub struct ObjMesh {
    pub vertex_buffer: VertexBuffer,
    pub no_material_index_buffer: Option<IndexBuffer>,
    pub submeshes: Vec<ObjMeshSubset>,
}

impl ObjMesh {
    pub fn new(renderer: &Renderer, obj_file: ObjFile, mtl_file: MtlFile) -> ObjMesh {
        let mut verts: Vec<Vert> = vec![];
        let mut no_material_indices: Vec<u16> = vec![];
        let mut index = 0;

        let mut indices_by_material: HashMap<String, Vec<u16>> = HashMap::new();
        for face in obj_file.faces {
            let numverts = face.verts.len();
            for vert in face.verts {
                let vertex = obj_file.vertices[vert.index - 1];
                let normal = obj_file.vertex_normals[vert.normal_index - 1];
                let uv = obj_file.tex_coords[vert.texcoord_index - 1];

                verts.push(Vert {
                    pos: vertex,
                    tex: [uv[0], uv[1]],
                    normal,
                });
            }

            let indices = match face.material {
                Some(material) => indices_by_material.entry(material).or_default(),
                None => &mut no_material_indices,
            };

            for i in 0..(numverts - 2) {
                indices.push(index);
                indices.push(index + i as u16 + 1);
                indices.push(index + i as u16 + 2);
            }

            index += numverts as u16;
        }

        let vertex_buffer =
            VertexBuffer::new::<Vert>(renderer, obj_model_shader_layout(), 0, verts);
        let index_buffer = if !no_material_indices.is_empty() {
            Some(IndexBuffer::new(renderer, no_material_indices))
        } else {
            None
        };

        let objs = indices_by_material
            .into_iter()
            .filter_map(|(material, indices)| {
                if let Some(material) = mtl_file.materials.get(&material) {
                    let index_buffer = IndexBuffer::new(renderer, indices);
                    Some(ObjMeshSubset {
                        index_buffer,
                        ambient: material.ambient_color,
                        diffuse: material.diffuse_color,
                    })
                } else {
                    None
                }
            })
            .collect();

        ObjMesh {
            vertex_buffer,
            no_material_index_buffer: index_buffer,
            submeshes: objs,
        }
    }
}

pub struct ObjFile {
    pub vertices: Vec<[f32; 4]>,
    pub tex_coords: Vec<[f32; 3]>,
    pub vertex_normals: Vec<[f32; 3]>,
    pub faces: Vec<ObjFace>,
}

pub struct ObjFace {
    pub material: Option<String>,
    pub verts: Vec<ObjVert>,
}

pub struct ObjVert {
    pub index: usize,
    pub texcoord_index: usize,
    pub normal_index: usize,
}

pub fn load_obj_file(path: &'static str) -> Result<ObjFile, Box<dyn Error>> {
    let file = fs::read(path)?;

    let mut material_file_names = vec![];
    let mut vertices = vec![];
    let mut tex_coords = vec![];
    let mut vertex_normals = vec![];
    let mut active_material = None;
    let mut faces = vec![];

    file.lines().for_each(|l| if let Ok(line) = l {
        if line.is_empty() || line.starts_with('#') {
            return;
        }

        let line: Vec<&str> = line.split(' ').collect();

        match line[0] {
            "v" => {
                let x = line[1].parse::<f32>().unwrap();
                let y = line[2].parse::<f32>().unwrap();
                let z = line[3].parse::<f32>().unwrap();
                vertices.push([x, y, z, 1.]);
            }
            "vt" => {
                let u = line[1].parse::<f32>().unwrap();
                let v = if line.len() > 2 {
                    line[2].parse::<f32>().unwrap()
                } else {
                    0.
                };
                let w = if line.len() > 3 {
                    line[3].parse::<f32>().unwrap()
                } else {
                    0.
                };
                tex_coords.push([u, v, w])
            }
            "vn" => {
                let x = line[1].parse::<f32>().unwrap();
                let y = line[2].parse::<f32>().unwrap();
                let z = line[3].parse::<f32>().unwrap();
                vertex_normals.push([x, y, z]);
            }
            "vp" => {}
            "f" => {
                let num_verts = line.len() - 1;
                let mut verts = vec![];
                for i in 0..num_verts {
                    let vert: Vec<&str> = line[i + 1].split('/').collect();
                    let index = vert[0].parse::<u32>().unwrap();
                    let texcoord_index = if vert.len() > 1 {
                        vert[1].parse::<u32>().unwrap()
                    } else {
                        0
                    };
                    let normal_index = if vert.len() > 2 {
                        vert[2].parse::<u32>().unwrap()
                    } else {
                        0
                    };
                    let vert = ObjVert {
                        index: index as usize,
                        normal_index: normal_index as usize,
                        texcoord_index: texcoord_index as usize,
                    };
                    verts.push(vert);
                }
                let face = ObjFace {
                    verts,
                    material: active_material.clone(),
                };
                faces.push(face);
            }
            "usemtl" => {
                active_material = Some(line[1].to_string());
            }
            "mtllib" => {
                material_file_names.push(line[1].to_string());
            }
            _default => {}
        }
    });

    Ok(ObjFile {
        vertices,
        tex_coords,
        vertex_normals,
        faces,
    })
}

pub struct MtlFile {
    pub materials: HashMap<String, MtlMaterialDefinition>,
}

pub struct MtlMaterialDefinition {
    pub name: String,
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub emission_color: [f32; 3],
    pub specular_highlights: f32,
    pub optical_density: f32,
    pub dissolve: f32,
    pub illumination: u8,
}

pub fn load_mtl_file(path: &'static str) -> Result<MtlFile, Box<dyn Error>> {
    let file = fs::read(path)?;

    let mut materials: Vec<MtlMaterialDefinition> = vec![];

    let mut name: Option<String> = None;
    let mut ambient_color = [0.; 3];
    let mut diffuse_color = [0.; 3];
    let mut specular_color = [0.; 3];
    let mut emission_color = [0.; 3];
    let mut specular_highlights = 0.;
    let mut optical_density = 0.;
    let mut dissolve = 0.;
    let mut illumination = 0;

    file.lines().for_each(|l| if let Ok(line) = l {
        if line.is_empty() || line.starts_with('#') {
            return;
        }

        let line: Vec<&str> = line.split(' ').collect();

        match line[0] {
            "newmtl" => {
                if let Some(name) = &name {
                    materials.push(MtlMaterialDefinition {
                        name: name.clone(),
                        ambient_color,
                        diffuse_color,
                        specular_color,
                        emission_color,
                        specular_highlights,
                        optical_density,
                        dissolve,
                        illumination,
                    });
                    ambient_color = [0.; 3];
                    diffuse_color = [0.; 3];
                    specular_color = [0.; 3];
                    specular_highlights = 0.;
                    optical_density = 0.;
                    dissolve = 0.;
                    illumination = 0;
                }
                name = Some(line[1].to_string());
            }
            "Ns" => {
                specular_highlights = line[1].parse::<f32>().unwrap();
            }
            "Ka" => {
                ambient_color = [
                    line[1].parse::<f32>().unwrap(),
                    line[2].parse::<f32>().unwrap(),
                    line[3].parse::<f32>().unwrap(),
                ];
            }
            "Kd" => {
                diffuse_color = [
                    line[1].parse::<f32>().unwrap(),
                    line[2].parse::<f32>().unwrap(),
                    line[3].parse::<f32>().unwrap(),
                ];
            }
            "Ks" => {
                specular_color = [
                    line[1].parse::<f32>().unwrap(),
                    line[2].parse::<f32>().unwrap(),
                    line[3].parse::<f32>().unwrap(),
                ];
            }
            "Ke" => {
                emission_color = [
                    line[1].parse::<f32>().unwrap(),
                    line[2].parse::<f32>().unwrap(),
                    line[3].parse::<f32>().unwrap(),
                ];
            }
            "Ni" => {
                optical_density = line[1].parse::<f32>().unwrap();
            }
            "d" => {
                dissolve = line[1].parse::<f32>().unwrap();
            }
            "illum" => {
                illumination = line[1].parse::<u8>().unwrap();
            }
            _default => {}
        }
    });
    if let Some(name) = name {
        materials.push(MtlMaterialDefinition {
            name: name.clone(),
            ambient_color,
            diffuse_color,
            specular_color,
            emission_color,
            specular_highlights,
            optical_density,
            dissolve,
            illumination,
        });
    }

    let material_definitions: HashMap<_, _> = materials
        .into_iter()
        .map(|item| (item.name.clone(), item))
        .collect();
    Ok(MtlFile {
        materials: material_definitions,
    })
}
