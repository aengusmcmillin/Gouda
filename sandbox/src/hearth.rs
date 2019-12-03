use gouda::rendering::drawable::{QuadDrawable, TextureDrawable};
use gouda::rendering::texture::RenderableTexture;
use gouda::ecs::{Entity, ECS};
use crate::tilemap::Tile;
use std::rc::Rc;
use gouda::rendering::{Renderer, Scene};
use gouda::png::PNG;
use crate::camera::Camera;

#[derive(Debug)]
pub struct Hearth {
    drawable: TextureDrawable,
}

impl Hearth {
    pub fn create(ecs: &mut ECS, tile: Entity) {
        let tile = ecs.read::<Tile>(&tile).unwrap();

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let texture = RenderableTexture::new(renderer, &PNG::from_file("bitmap/hearth.png").unwrap().image());
        let drawable = TextureDrawable::new(false, renderer, texture, [tile.x as f32, tile.y as f32, 0.], [0.4, 0.4, 1.0], [0.; 3]);
        let hearth = Hearth {
            drawable,
        };
        ecs.build_entity().add(hearth);
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera) {
        self.drawable.draw_with_projection(scene, &camera.projection_buffer);
    }
}

