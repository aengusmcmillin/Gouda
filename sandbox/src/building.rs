use gouda::ecs::{ECS, Entity};
use crate::tilemap::Tile;
use gouda::rendering::drawable::TextureDrawable;
use gouda::rendering::{Renderer, Scene};
use gouda::rendering::texture::RenderableTexture;
use std::rc::Rc;
use gouda::png::PNG;
use crate::camera::Camera;

#[derive(Debug)]
pub struct Turret {
    texture_drawable: TextureDrawable,
}

impl Turret {
    pub fn create(ecs: &mut ECS, tile: Entity) {
        let tile = ecs.read::<Tile>(&tile).unwrap();

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let texture = RenderableTexture::new_from_png(renderer, PNG::from_file("bitmap/turret.png").unwrap());
        let texture_drawable = TextureDrawable::new(false, renderer, texture, [tile.x as f32, tile.y as f32, 0.], [0.4, 0.4, 1.0]);
        let turret = Turret {
            texture_drawable,
        };
        ecs.build_entity().add(turret);
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera) {
        self.texture_drawable.draw_with_projection(scene, &camera.projection_buffer);
    }
}

