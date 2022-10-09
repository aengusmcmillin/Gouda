// use gouda::rendering::drawable::TextureDrawable;
// use gouda::ecs::ECS;
// use gouda::rendering::{Renderer, Scene};
// use std::rc::Rc;
// use gouda::rendering::texture::RenderableTexture;
// use gouda::camera::Camera;
// use gouda::images::bmp::Bitmap;

// #[derive(Debug)]
// pub struct Villager {
//     drawable: TextureDrawable,
//     x: f32,
//     y: f32,
// }

// impl Villager {
//     pub fn create(ecs: &mut ECS) {
//         let renderer = ecs.read_res::<Rc<Renderer>>();
//         let bmp = Bitmap::new("bitmap/test_bmp.bmp");
//         let texture = RenderableTexture::new(renderer, &bmp.unwrap().image());
//         let player_drawable = TextureDrawable::new(false, renderer, texture);
//         ecs.build_entity().add(Villager {drawable: player_drawable, x: -4., y: -1.});
//     }

//     pub fn draw(&self, scene: &Scene, camera: &Camera) {
//         self.drawable.draw_with_projection(&scene, &camera.projection_buffer)
//     }

//     pub fn set_pos(&mut self, renderer: &Renderer, new_x: f32, new_y: f32) {
//         self.x = new_x;
//         self.y = new_y;
//         self.drawable.set_position([self.x, self.y, 0.]);
//     }

//     pub fn move_pos(&mut self, renderer: &Renderer, dx: f32, dy: f32) {
//         self.set_pos(renderer, self.x + dx, self.y + dy);
//     }
// }
