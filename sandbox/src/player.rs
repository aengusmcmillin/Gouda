use gouda::rendering::{
    drawable::{QuadDrawable, TextureDrawable},
    Renderer,
    Scene,
    texture::RenderableTexture};
use std::rc::Rc;
use gouda::ecs::{ECS, Mutations, Entity, Mutation};
use gouda::input::{GameInput, LetterKeys};
use gouda::png::PNG;
use gouda::camera::Camera;
use crate::tilemap::{Tilemap};
use gouda::images::spritesheet::Spritesheet;
use gouda::types::Direction;
use gouda::types::Direction::{Right, Left, Down, Top};

#[derive(Debug)]
pub struct Player {
    drawables: [TextureDrawable; 4],
    selected_drawable: QuadDrawable,
    x: i32,
    y: i32,
    pub current_tile: Entity,
    is_selected: bool,
    direction: Direction,
}

impl Player {
    pub fn create(ecs: &mut ECS) {
        let renderer = ecs.read_res::<Rc<Renderer>>();

        let png = PNG::from_file("bitmap/spritesheet.png");
        let sheet = Spritesheet::new(1, 4, png.unwrap().image());

        let texture = RenderableTexture::new(renderer, &sheet.sprite(0, 0));
        let player_drawable_down = TextureDrawable::new(false, renderer, texture);
        let texture = RenderableTexture::new(renderer, &sheet.sprite(1, 0));
        let player_drawable_left = TextureDrawable::new(false, renderer, texture);
        let texture = RenderableTexture::new(renderer, &sheet.sprite(2, 0));
        let player_drawable_up = TextureDrawable::new(false, renderer, texture);
        let texture = RenderableTexture::new(renderer, &sheet.sprite(3, 0));
        let player_drawable_right = TextureDrawable::new(false, renderer, texture);
        let selected_drawable = QuadDrawable::new(false, renderer, [0.8, 0.8, 0.8]);

        let tile = ecs.read_res::<Tilemap>().tile_at_pos(1, 2);
        let drawables = [
            player_drawable_down,
            player_drawable_left,
            player_drawable_up,
            player_drawable_right
        ];
        ecs.build_entity().add(Player {drawables, selected_drawable, x: -4, y: -1, current_tile: tile, is_selected: false, direction: Direction::Down});
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.is_selected = selected;
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera) {
        if self.is_selected {
            self.selected_drawable.draw_with_projection(&scene, &camera.projection_buffer);
        }
        if self.direction == Down {
            self.drawables[0].draw_with_projection(&scene, &camera.projection_buffer)
        } else if self.direction == Left {
            self.drawables[1].draw_with_projection(&scene, &camera.projection_buffer)
        } else if self.direction == Top {
            self.drawables[2].draw_with_projection(&scene, &camera.projection_buffer)
        } else if self.direction == Right {
            self.drawables[3].draw_with_projection(&scene, &camera.projection_buffer)
        }
    }

    pub fn set_pos(&mut self, tile: Entity, renderer: &Renderer, new_x: i32, new_y: i32) {
        self.x = new_x;
        self.y = new_y;
        self.current_tile = tile;
        self.selected_drawable.translate(renderer, [self.x as f32, self.y as f32, 0.], [0.4, 0.4, 1.]);
        for drawable in &mut self.drawables {
            drawable.set_position([self.x as f32, self.y as f32, 0.]);
        }
    }

    pub fn move_pos(&mut self, tile: Entity, renderer: &Renderer, dx: i32, dy: i32) {
        self.set_pos(tile, renderer, self.x + dx, self.y + dy);

        if dx > 0 {
            self.direction = Right;
        } else if dx < 0 {
            self.direction = Left;
        } else if dy < 0 {
            self.direction = Down;
        } else if dy > 0 {
            self.direction = Top;
        }
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
    for (_, ent) in ecs.read1::<Player>() {
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
