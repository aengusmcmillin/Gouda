use std::rc::Rc;

use gouda_ecs::ECS;
use gouda_transform::TransformComponent;
use gouda_images::{png::PNG, spritesheet::Spritesheet};

use super::{Renderer, texture::RenderableTexture, Scene};

#[derive(Debug)]
pub struct SpriteComponent {
    texture: RenderableTexture,
}

impl SpriteComponent {
    pub fn new(ecs: &mut ECS, sprite_name: String) -> SpriteComponent {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let texture = RenderableTexture::new(renderer, &PNG::from_file(&sprite_name).unwrap().image(), false);
        return SpriteComponent {texture}
    }

    pub fn draw(&self, scene: &Scene, location: &TransformComponent) {
        scene.submit_texture(&self.texture, location.transform_matrix())
    }
}

#[derive(Debug)]
pub struct SpriteSheetComponent {
    textures: Vec<RenderableTexture>,
    pub active: usize,
}

impl SpriteSheetComponent {
    pub fn new(ecs: &mut ECS, spritesheet_name: String, rows: usize, columns: usize) -> SpriteSheetComponent {
        let renderer = ecs.read_res::<Rc<Renderer>>();

        let png = PNG::from_file(&spritesheet_name);
        let sheet = Spritesheet::new(rows, columns, png.unwrap().image());

        let mut all_textures = vec![];
        for i in 0..rows {
            for j in 0..columns {
                let texture = RenderableTexture::new(renderer, &sheet.sprite(j, i), false);
                all_textures.push(texture);
            }
        }
        return SpriteSheetComponent {
            textures: all_textures,
            active: 0,
        }
    }

    pub fn draw(&self, scene: &Scene, location: &TransformComponent) {
        let texture = self.textures.get(self.active).unwrap();
        scene.submit_texture(&texture, location.transform_matrix())
    }
}

#[derive(Debug)]
pub struct SpriteListComponent {
    textures: Vec<RenderableTexture>,
    active: usize,
}

impl SpriteListComponent {
    pub fn new(ecs: &mut ECS, sprite_names: Vec<String>) -> SpriteListComponent {
        let mut all_textures = vec![];
        for sprite_name in sprite_names {
            let renderer = ecs.read_res::<Rc<Renderer>>();
            let texture = RenderableTexture::new(renderer, &PNG::from_file(&sprite_name).unwrap().image(), false);
            all_textures.push(texture);
        }
        return SpriteListComponent {textures: all_textures, active: 0}
    }

    pub fn draw(&self, scene: &Scene, location: &TransformComponent) {
        let texture = self.textures.get(self.active).unwrap();
        scene.submit_texture(&texture, location.transform_matrix())
    }
}

#[derive(Debug)]
pub struct ColorBoxComponent {
    color: [f32; 4],
}

impl ColorBoxComponent {

    pub fn new(_ecs: &mut ECS, color: [f32; 3]) -> ColorBoxComponent {
        return ColorBoxComponent { color: [color[0], color[1], color[2], 1.] };
    }

    pub fn draw(&self, scene: &Scene, location: &TransformComponent) {
        scene.submit_shape_by_name("quad", "quad", location.transform_matrix(), self.color)
    }

}