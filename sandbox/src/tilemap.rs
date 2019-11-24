use gouda::rendering::{drawable::{TextureDrawable, QuadDrawable}, Renderer, Scene, texture::RenderableTexture};
use gouda::ecs::{ECS, Entity, GenIndex};
use std::rc::Rc;
use crate::camera::Camera;
use gouda::bmp::{Bitmap, debug_load_bmp};
use gouda::mouse_capture::{MouseCaptureArea, MouseCaptureLayer, ActiveCaptureLayer};
use gouda::types::{Bounds, Direction};

const GRASS_COLOR: [f32; 3] = [0.2, 0.4, 0.3];
const HEARTH_COLOR: [f32; 3] = [0.5, 0.2, 0.2];
const BORDER_COLOR: [f32; 3] = [0.5, 0.5, 0.5];

#[derive(Debug)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    neighbors: [Option<Entity>; 4],
    color_drawable: Option<QuadDrawable>,
    texture_drawable: Option<TextureDrawable>,
}

impl Tile {

    pub fn create_grass(ecs: &mut ECS, x: usize, y: usize) -> Entity {
        let grass = debug_load_bmp("bitmap/grass.bmp");
        Self::create_texture_tile(ecs, grass.unwrap(), x, y)
    }

    pub fn create_border(ecs: &mut ECS, x: usize, y: usize) -> Entity {
        let grass = debug_load_bmp("bitmap/grass2.bmp");
        Self::create_texture_tile(ecs, grass.unwrap(), x, y)
    }

    pub fn create_hearth(ecs: &mut ECS, x: usize, y: usize) -> Entity {
        let hearth = debug_load_bmp("bitmap/hearth.bmp");
        Self::create_texture_tile(ecs, hearth.unwrap(), x, y)
    }

    pub fn neighbor(&self, direction: Direction) -> Option<Entity> {
        self.neighbors[direction as usize]
    }

    fn create_texture_tile(ecs: &mut ECS, bmp: Bitmap, x: usize, y: usize) -> Entity {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let drawable = TextureDrawable::new(false, renderer, RenderableTexture::new(renderer, bmp), [-5. + x as f32, -3. + y as f32, 0.], [0.52, 0.52, 1.], [0.; 3]);
        let tile = Tile {
            color_drawable: None,
            texture_drawable: Some(drawable),
            x: x as i32 - 5,
            y: y as i32 - 3,
            neighbors: [None; 4],
        };
        ecs.build_entity().add(tile).add(MouseCaptureArea::new(Bounds{x: x as i32 * 80, y: y as i32 * 80 + 160, w: 80, h: 80})).entity()
    }

    fn create_tile(ecs: &mut ECS, color: [f32; 3], x: usize, y: usize) -> Entity {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let quad = QuadDrawable::new(false, renderer, color, [-5. + x as f32 * 1., -3. + y as f32 * 1., 0.], [0.5, 0.5, 1.], [0.; 3]);
        let tile = Tile {
            color_drawable: Some(quad),
            texture_drawable: None,
            x: x as i32 - 5,
            y: y as i32 - 3,
            neighbors: [None; 4],
        };
        ecs.build_entity().add(tile).add(MouseCaptureArea::new(Bounds{x: x as i32 * 80, y: y as i32 * 80 + 160, w: 80, h: 80})).entity()
    }

    pub fn draw(&self, scene: &Scene, camera: &Camera) {
        if let Some(drawable) = &self.color_drawable {
            drawable.draw_with_projection(&scene, &camera.projection_buffer);
        } else if let Some(drawable) = &self.texture_drawable {
            drawable.draw_with_projection(&scene, &camera.projection_buffer);
        }
    }
}

pub struct Tilemap {
    tiles: Vec<Vec<Entity>>,
}

fn set_neighbors(tile: &mut Tile, x: usize, y: usize, tiles: &Vec<Vec<Entity>>) {
    tile.neighbors = [
        if y > 0 { Some(tiles[x][y - 1]) } else { None },
        if x < (tiles.len() - 1) { Some(tiles[x + 1][y]) } else { None },
        if y < (tiles[x].len() - 1) { Some(tiles[x][y + 1]) } else { None },
        if x > 0 { Some(tiles[x - 1][y]) } else { None },
    ]
}

impl Tilemap {
    pub fn create(ecs: &mut ECS) {
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
        let mut all_tiles = vec![];
        for tiles in &tiles {
            for tile in tiles {
                all_tiles.push(tile.clone());
            }
        }

        for x in 0..11 {
            for y in 0..9 {
                let t = ecs.write::<Tile>(&tiles[x][y]).unwrap();
                set_neighbors(t, x, y, &tiles);
            }
        }
        let capture_area = MouseCaptureLayer {
            sort_index: 0,
            capture_areas: all_tiles,
        };
        ecs.build_entity().add(capture_area).add(ActiveCaptureLayer {});
        let res = Tilemap {
            tiles
        };
        ecs.add_res(res);
    }

    pub fn tile_at_pos(&self, x: usize, y: usize) -> Entity {
        self.tiles[x][y].clone()
    }
}

