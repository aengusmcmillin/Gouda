use gouda::rendering::drawable::TextureDrawable;
use gouda::ecs::ECS;
use gouda::bmp::debug_load_bmp;
use gouda::rendering::{Renderer, Scene};
use std::rc::Rc;
use crate::camera::Camera;
use gouda::types::WorldPosition;
use gouda::rendering::texture::RenderableTexture;

#[derive(Debug)]
pub struct Villager {
    drawable: TextureDrawable,
    x: f32,
    y: f32,
}

impl Villager {
    pub fn create(ecs: &mut ECS) {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let bmp = debug_load_bmp("bitmap/test_bmp.bmp");
        let texture = RenderableTexture::new(renderer, bmp.unwrap());
        let player_drawable = TextureDrawable::new(false, renderer, texture, [-4., -1., 0.], [0.3, 0.3, 1.], [0.; 3]);
        ecs.build_entity().add(Villager {drawable: player_drawable, x: -4., y: -1.});
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera) {
        self.drawable.draw_with_projection(&scene, &camera.projection_buffer)
    }

    pub fn set_pos(&mut self, renderer: &Renderer, new_x: f32, new_y: f32) {
        self.x = new_x;
        self.y = new_y;
        self.drawable.set_position(renderer, [self.x, self.y, 0.]);
    }

    pub fn move_pos(&mut self, renderer: &Renderer, dx: f32, dy: f32) {
        self.set_pos(renderer, self.x + dx, self.y + dy);
    }
}
