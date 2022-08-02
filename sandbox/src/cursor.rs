use gouda::rendering::{
    drawable::{QuadDrawable, TextureDrawable},
    Renderer,
    Scene,
    texture::RenderableTexture};
use std::rc::Rc;
use gouda::camera::Camera;
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
        let mut res = Cursor {
            visible: false,
            top_drawable: QuadDrawable::new(false, renderer, [0., 0., 0.]),
            left_drawable: QuadDrawable::new(false, renderer, [0., 0., 0.]),
            bottom_drawable: QuadDrawable::new(false, renderer, [0., 0., 0.]),
            right_drawable: QuadDrawable::new(false, renderer, [0., 0., 0.]),
        };
        res.top_drawable.set_scale([0.42, 0.02, 0.4]);
        res.left_drawable.set_scale([0.02, 0.42, 0.4]);
        res.bottom_drawable.set_scale([0.42, 0.02, 0.4]);
        res.right_drawable.set_scale([0.02, 0.42, 0.4]);
        res
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_pos(&mut self, pos: [f32; 3]) {
        self.top_drawable.set_position([pos[0], pos[1] + 0.4, pos[2]]);
        self.left_drawable.set_position([pos[0] - 0.4, pos[1], pos[2]]);
        self.bottom_drawable.set_position([pos[0], pos[1] - 0.4, pos[2]]);
        self.right_drawable.set_position([pos[0] + 0.4, pos[1], pos[2]]);
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

