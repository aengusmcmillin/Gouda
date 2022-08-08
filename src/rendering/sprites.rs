use std::rc::Rc;

use crate::{png::PNG, ecs::ECS, camera::Camera, TransformComponent, images::spritesheet::Spritesheet, math::Vec2};

use super::{Renderer, texture::RenderableTexture, drawable::{TextureDrawable, QuadDrawable}, Scene};

#[derive(Debug)]
pub struct SpriteComponent {
    texture_drawable: TextureDrawable,
}

impl SpriteComponent {
    pub fn new(ecs: &mut ECS, sprite_name: String) -> SpriteComponent {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let texture = RenderableTexture::new(renderer, &PNG::from_file(&sprite_name).unwrap().image());
        let texture_drawable = TextureDrawable::new(false, renderer, texture);
        return SpriteComponent {texture_drawable}
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera, location: &TransformComponent) {
        self.texture_drawable.apply_transform([location.x, location.y, 0.], [location.scale_x, location.scale_y, 1.0], [location.rot_x, location.rot_y, 0.]);
        self.texture_drawable.draw_with_projection(scene, &camera.projection_buffer);
    }
}

#[derive(Debug)]
pub struct SpriteSheetComponent {
    texture_drawables: Vec<TextureDrawable>,
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
                let texture = RenderableTexture::new(renderer, &sheet.sprite(j, i));
                let texture_drawable = TextureDrawable::new(false, renderer, texture);
                all_textures.push(texture_drawable);
            }
        }
        return SpriteSheetComponent {
            texture_drawables: all_textures,
            active: 0,
        }
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera, location: &TransformComponent) {
        let texture_drawable = self.texture_drawables.get(self.active).unwrap();
        texture_drawable.apply_transform([location.x, location.y, 0.], [location.scale_x, location.scale_y, 1.0], [location.rot_x, location.rot_y, 0.]);
        texture_drawable.draw_with_projection(scene, &camera.projection_buffer);
    }
}

#[derive(Debug)]
pub struct SpriteListComponent {
    texture_drawables: Vec<TextureDrawable>,
    active: usize,
}

impl SpriteListComponent {
    pub fn new(ecs: &mut ECS, sprite_names: Vec<String>) -> SpriteListComponent {
        let mut all_textures = vec![];
        for sprite_name in sprite_names {
            let renderer = ecs.read_res::<Rc<Renderer>>();
            let texture = RenderableTexture::new(renderer, &PNG::from_file(&sprite_name).unwrap().image());
            let texture_drawable = TextureDrawable::new(false, renderer, texture);
            all_textures.push(texture_drawable);
        }
        return SpriteListComponent {texture_drawables: all_textures, active: 0}
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera, location: &TransformComponent) {
        let texture_drawable = self.texture_drawables.get(self.active).unwrap();
        texture_drawable.apply_transform([location.x, location.y, 0.], [location.scale_x, location.scale_y, 1.0], [location.rot_x, location.rot_y, 0.]);
        texture_drawable.draw_with_projection(scene, &camera.projection_buffer);
    }
}

#[derive(Debug)]
pub struct ColorBoxComponent {
    quad_drawable: QuadDrawable,
}

impl ColorBoxComponent {

    pub fn new(ecs: &mut ECS, color: [f32; 3]) -> ColorBoxComponent {

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let quad_drawable = QuadDrawable::new(false, renderer, color);
        return ColorBoxComponent { quad_drawable: quad_drawable };
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera, location: &TransformComponent) {
        self.quad_drawable.apply_transform([location.x, location.y, 0.], [location.scale_x, location.scale_y, 1.0], [location.rot_x, location.rot_y, 0.]);
        self.quad_drawable.draw_with_projection(scene, &camera.projection_buffer);
    }

}