use std::collections::HashMap;

use crate::material_library::Material;
use crate::obj::ObjFile;
use crate::shader_lib::obj_model_shader::obj_model_shader_layout;
use crate::Renderer;

use super::buffers::{IndexBuffer, VertexBuffer};

pub struct Model {
    pub mesh: Mesh,
    pub materials: Vec<Material>,
}

impl Model {}

pub struct Mesh {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer,
    pub submeshes: Vec<Submesh>,
}

pub struct Submesh {
    pub starting_index: u32,
    pub index_count: u32,
}

#[repr(packed(1))]
pub struct Vert {
    pub pos: [f32; 4],
    pub tex: [f32; 2],
    pub normal: [f32; 3],
}

impl Mesh {
    pub fn new_from_obj(renderer: &Renderer, obj_file: ObjFile) -> Mesh {
        let mut verts: Vec<Vert> = vec![];
        let mut no_material_indices: Vec<u16> = vec![];
        let mut index = 0;

        let mut indices_by_material: HashMap<String, Vec<u16>> = HashMap::new();
        for face in obj_file.faces {
            let numverts = face.verts.len();
            for vert in face.verts {
                let vertex = obj_file.vertices.get(vert.index - 1).unwrap();
                let normal = obj_file.vertex_normals.get(vert.normal_index - 1).unwrap();
                let uv = obj_file.tex_coords.get(vert.texcoord_index - 1).unwrap();

                verts.push(Vert {
                    pos: *vertex,
                    tex: [uv[0], uv[1]],
                    normal: *normal,
                });
            }

            let indices = match face.material {
                Some(material) => indices_by_material.entry(material).or_insert(vec![]),
                None => &mut no_material_indices,
            };

            for i in 0..(numverts - 2) {
                indices.push(index);
                indices.push(index + i as u16 + 1);
                indices.push(index + i as u16 + 2);
            }

            index = index + numverts as u16;
        }

        let submeshes: Vec<Submesh> = vec![];

        let vertex_buffer =
            VertexBuffer::new::<Vert>(renderer, obj_model_shader_layout(), 0, verts);

        let all_indices: Vec<u16> = vec![];
        let mut submesh_idx = 0;
        if no_material_indices.len() > 0 {
            let len = no_material_indices.len() as u32;
            submeshes.push(Submesh {
                starting_index: submesh_idx,
                index_count: len,
            });
            all_indices.append(&mut no_material_indices);

            submesh_idx += len;
        };

        indices_by_material
            .into_iter()
            .for_each(|(material, indices)| {
                let len = indices.len() as u32;
                submeshes.push(Submesh {
                    starting_index: submesh_idx,
                    index_count: len,
                });
                all_indices.append(&mut indices);
                submesh_idx += len;
            });
        let index_buffer = IndexBuffer::new(renderer, all_indices);

        return Mesh {
            vertex_buffer,
            index_buffer,
            submeshes,
        };
    }
}
