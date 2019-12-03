use std::io::Read;
use crate::tilemap::Tile;
use gouda::ecs::{ECS, Entity};
use gouda::rendering::{Renderer, Scene};
use std::rc::Rc;
use gouda::png::PNG;
use gouda::rendering::drawable::TextureDrawable;
use gouda::rendering::texture::RenderableTexture;
use crate::camera::Camera;

#[derive(Debug)]
pub struct Tree {
    texture_drawable: TextureDrawable,
    wood: i32,
    pub x: f32,
    pub y: f32,
}

impl Tree {
    pub fn create(ecs: &mut ECS, tile: Entity) {
        ecs.write::<Tile>(&tile).unwrap().occupied = true;
        let tile = ecs.read::<Tile>(&tile).unwrap();

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let texture = RenderableTexture::new(renderer, &PNG::from_file("bitmap/tree.png").unwrap().image());
        let texture_drawable = TextureDrawable::new(false, renderer, texture, [tile.x as f32, tile.y as f32, 0.], [0.4, 0.4, 1.0], [0., 0., 0.]);
        let tree = Tree {
            texture_drawable,
            wood: 10,
            x: tile.x as f32,
            y: tile.y as f32,
        };
        ecs.build_entity().add(tree);
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera) {
        self.texture_drawable.draw_with_projection(scene, &camera.projection_buffer);
    }
    pub fn harvest(&mut self) -> i32 {
        self.wood -= 10;
        return 10;
    }
}