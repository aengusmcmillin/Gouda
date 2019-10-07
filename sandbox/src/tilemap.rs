use gouda::rendering::{QuadDrawable, Renderer, Scene};
use gouda::ecs::{ECS, Entity, GenIndex};
use std::rc::Rc;
use crate::camera::Camera;

const GRASS_COLOR: [f32; 3] = [0.2, 0.4, 0.3];
const HEARTH_COLOR: [f32; 3] = [0.5, 0.2, 0.2];
const BORDER_COLOR: [f32; 3] = [0.5, 0.5, 0.5];

#[derive(Debug)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    drawable: QuadDrawable,
}

impl Tile {
    pub fn create_grass(ecs: &mut ECS, x: usize, y: usize) -> Entity {
        Self::create_tile(ecs, GRASS_COLOR, x, y)
    }

    pub fn create_border(ecs: &mut ECS, x: usize, y: usize) -> Entity {
        Self::create_tile(ecs, BORDER_COLOR, x, y)
    }

    pub fn create_hearth(ecs: &mut ECS, x: usize, y: usize) -> Entity {
        Self::create_tile(ecs, HEARTH_COLOR, x, y)
    }

    fn create_tile(ecs: &mut ECS, color: [f32; 3], x: usize, y: usize) -> Entity {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let quad = QuadDrawable::new(false, renderer, color, [-5. + x as f32 * 1., -3. + y as f32 * 1., 0.], [0.45, 0.45, 1.]);
        let tile = Tile {
            drawable: quad,
            x: x as i32 - 5,
            y: y as i32 - 3,
        };
        ecs.build_entity().add(tile).entity()
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera) {
        self.drawable.draw_with_projection(&scene, &camera.projection_buffer);
    }
}

pub struct Tilemap {
    tiles: Vec<Vec<Entity>>,
}

impl Tilemap {
    pub fn create(ecs: &mut ECS) -> Self {
        let mut tiles: Vec<Vec<Entity>> = vec![Vec::with_capacity(9); 11];
        for x in 0..11 {
            for y in 0..9 {
                let tile = if x == 5 && y == 4 {
                    Tile::create_hearth(ecs, x, y)
                } else if x == 0 || x == 10 || y == 0 || y == 8 {
                    Tile::create_border(ecs, x, y)
                } else {
                    Tile::create_grass(ecs, x, y)
                };
                tiles[x].push(tile);
            }
        }
        return Tilemap {
            tiles
        };
    }
}

