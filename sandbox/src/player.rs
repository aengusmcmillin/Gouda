use gouda::rendering::{
    drawable::{QuadDrawable, TextureDrawable},
    Renderer,
    Scene,
    texture::RenderableTexture};
use std::rc::Rc;
use gouda::ecs::{ECS, Mutations, Entity, Mutation};
use gouda::input::{GameInput, LetterKeys};
use crate::camera::Camera;
use gouda::bmp::{Bitmap, debug_load_bmp};
use crate::tilemap::{Tilemap, Tile};

#[derive(Debug)]
pub struct Player {
    drawable: TextureDrawable,
    selected_drawable: QuadDrawable,
    x: i32,
    y: i32,
    pub current_tile: Entity,
    is_selected: bool,
}

impl Player {
    pub fn create(ecs: &mut ECS) {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let bmp = debug_load_bmp("bitmap/test_bmp.bmp");
        let texture = RenderableTexture::new(renderer, bmp.unwrap());
        let player_drawable = TextureDrawable::new(false, renderer, texture, [-4., -1., 0.], [0.3, 0.3, 1.]);
        let selected_drawable = QuadDrawable::new(false, renderer, [0.8, 0.8, 0.8], [-4., -1., 0.], [0.4, 0.4, 1.]);

        let tile = ecs.read_res::<Tilemap>().tile_at_pos(1, 2);
        ecs.build_entity().add(Player {drawable: player_drawable, selected_drawable, x: -4, y: -1, current_tile: tile, is_selected: false});
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.is_selected = selected;
    }
    pub fn draw(&self, scene: &Scene, camera: &Camera) {
        if self.is_selected {
            self.selected_drawable.draw_with_projection(&scene, &camera.projection_buffer);
        }
        self.drawable.draw_with_projection(&scene, &camera.projection_buffer)
    }

    pub fn set_pos(&mut self, tile: Entity, renderer: &Renderer, new_x: i32, new_y: i32) {
        self.x = new_x;
        self.y = new_y;
        self.current_tile = tile;
        self.selected_drawable.translate(renderer, [self.x as f32, self.y as f32, 0.], [0.4, 0.4, 1.]);
        self.drawable.translate(renderer, [self.x as f32, self.y as f32, 0.], [0.3, 0.3, 1.]);
    }

    pub fn move_pos(&mut self, tile: Entity, renderer: &Renderer, dx: i32, dy: i32) {
        self.set_pos(tile, renderer, self.x + dx, self.y + dy);
    }
}

struct MoveMutation {
    entity: Entity,
    dx: i32,
    dy: i32,
}

impl Mutation for MoveMutation {
    fn apply(&self, ecs: &mut ECS) {
        let renderer = ecs.read_res::<Rc<Renderer>>().clone();
        let player = ecs.read::<Player>(&self.entity).unwrap();
        let tilemap = ecs.read_res::<Tilemap>();
        let tile = tilemap.tile_at_pos((player.x + self.dx + 5) as usize, (player.y + self.dy + 3) as usize);

        let player = ecs.write::<Player>(&self.entity).unwrap();
        player.move_pos(tile, &renderer, self.dx, self.dy);
    }
}

pub fn player_move_system(ecs: &ECS) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let mut mutations: Mutations = Vec::new();
    for (p, ent) in ecs.read1::<Player>() {
        if input.keyboard.letter_pressed(LetterKeys::A) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: -1, dy: 0}))
        } else if input.keyboard.letter_pressed(LetterKeys::D) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: 1, dy: 0}))
        }
        if input.keyboard.letter_pressed(LetterKeys::W) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: 0, dy: 1}))
        } else if input.keyboard.letter_pressed(LetterKeys::S) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: 0, dy: -1}))
        }
    }
    return mutations;
}
