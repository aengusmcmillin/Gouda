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
            top_drawable: QuadDrawable::new(false, renderer, [0., 0., 0.], [0., 0., 0.], [0.45, 0.05, 0.4]),
            left_drawable: QuadDrawable::new(false, renderer, [0., 0., 0.], [0., 0., 0.], [0.05, 0.45, 0.4]),
            bottom_drawable: QuadDrawable::new(false, renderer, [0., 0., 0.], [0., 0., 0.], [0.45, 0.05, 0.4]),
            right_drawable: QuadDrawable::new(false, renderer, [0., 0., 0.], [0., 0., 0.], [0.05, 0.45, 0.4]),
        }
    }

    pub fn draw_at_pos(&self, renderer: &Renderer, scene: &Scene, camera: &Camera, pos: [f32; 3]) {
        self.top_drawable.translate(renderer, [pos[0], pos[1] + 0.4, pos[2]], [0.42, 0.02, 0.4]);
        self.left_drawable.translate(renderer, [pos[0] - 0.4, pos[1], pos[2]], [0.02, 0.42, 0.4]);
        self.bottom_drawable.translate(renderer, [pos[0], pos[1] - 0.4, pos[2]], [0.42, 0.02, 0.4]);
        self.right_drawable.translate(renderer, [pos[0] + 0.4, pos[1], pos[2]], [0.02, 0.42, 0.4]);

        self.top_drawable.draw_with_projection(&scene, &camera.projection_buffer);
        self.left_drawable.draw_with_projection(&scene, &camera.projection_buffer);
        self.bottom_drawable.draw_with_projection(&scene, &camera.projection_buffer);
        self.right_drawable.draw_with_projection(&scene, &camera.projection_buffer);
    }

    pub fn handle_click(&self, tile: &Tile) {

    }
}

