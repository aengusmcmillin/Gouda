use gouda::rendering::{
    drawable::{QuadDrawable, TextureDrawable},
    Renderer,
    Scene,
    texture::RenderableTexture};
use std::rc::Rc;
use crate::camera::Camera;
use crate::tilemap::Tile;
use gouda::ecs::ECS;

pub struct Cursor {
    visible: bool,
    top_drawable: QuadDrawable,
    left_drawable: QuadDrawable,
    bottom_drawable: QuadDrawable,
    right_drawable: QuadDrawable,
}

impl Cursor {
    pub fn create(ecs: &mut ECS) {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let cursor = Cursor::new(renderer);
        ecs.add_res(cursor);
    }

    pub fn new(renderer: &Rc<Renderer>) -> Cursor {
        Cursor {
            visible: false,
            top_drawable: QuadDrawable::new(false, renderer, [0., 0., 0.], [0., 0., 0.], [0.42, 0.02, 0.4], [0.; 3]),
            left_drawable: QuadDrawable::new(false, renderer, [0., 0., 0.], [0., 0., 0.], [0.02, 0.42, 0.4], [0.; 3]),
            bottom_drawable: QuadDrawable::new(false, renderer, [0., 0., 0.], [0., 0., 0.], [0.42, 0.02, 0.4], [0.; 3]),
            right_drawable: QuadDrawable::new(false, renderer, [0., 0., 0.], [0., 0., 0.], [0.02, 0.42, 0.4], [0.; 3]),
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_pos(&mut self, renderer: &Renderer, pos: [f32; 3]) {
        self.top_drawable.set_position(renderer, [pos[0], pos[1] + 0.4, pos[2]]);
        self.left_drawable.set_position(renderer, [pos[0] - 0.4, pos[1], pos[2]]);
        self.bottom_drawable.set_position(renderer, [pos[0], pos[1] - 0.4, pos[2]]);
        self.right_drawable.set_position(renderer, [pos[0] + 0.4, pos[1], pos[2]]);
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera) {
        if !self.visible {
            return;
        }
        self.top_drawable.draw_with_projection(&scene, &camera.projection_buffer);
        self.left_drawable.draw_with_projection(&scene, &camera.projection_buffer);
        self.bottom_drawable.draw_with_projection(&scene, &camera.projection_buffer);
        self.right_drawable.draw_with_projection(&scene, &camera.projection_buffer);
    }

    pub fn handle_click(&self, tile: &Tile) {

    }
}

