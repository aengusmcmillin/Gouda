use crate::hearth::Hearth;
use gouda::ecs::{Entity, ECS};
use gouda::mouse_capture::{ActiveCaptureLayer, MouseCaptureArea, MouseCaptureLayer};
use gouda::rendering::sprites::SpriteComponent;
use gouda::transform::TransformComponent;
use gouda::types::{Bounds, Direction};

#[derive(Debug)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub occupied: bool,
    neighbors: [Option<Entity>; 4],
}

impl Tile {
    pub fn create_image_tile(image_name: String, ecs: &mut ECS, x: usize, y: usize) -> Entity {
        Self::create_texture_tile(ecs, image_name, x, y)
    }

    pub fn _neighbor(&self, direction: Direction) -> Option<Entity> {
        self.neighbors[direction as usize]
    }

    fn create_texture_tile(ecs: &mut ECS, image_name: String, x: usize, y: usize) -> Entity {
        let sprite = SpriteComponent::new(ecs, image_name);
        let x = x as i32 - 5;
        let y = y as i32 - 3;
        let tile = Tile {
            occupied: false,
            x: x,
            y: y,
            neighbors: [None; 4],
        };
        let transform = TransformComponent::builder()
            .position((x) as f32, (y) as f32)
            .scale(1.0, 1.0)
            .build();
        ecs.build_entity()
            .add_component(tile)
            .add_component(sprite)
            .add_component(transform)
            .add_component(MouseCaptureArea::new(
                false,
                Bounds {
                    x: x as f32 - 0.5,
                    y: y as f32 - 0.5,
                    w: 1.,
                    h: 1.,
                },
            ))
            .entity()
    }
}

pub struct Tilemap {
    tiles: Vec<Vec<Entity>>,
    borders: Vec<Entity>,
}

fn set_neighbors(tile: &mut Tile, x: usize, y: usize, tiles: &Vec<Vec<Entity>>) {
    tile.neighbors = [
        if y > 0 { Some(tiles[x][y - 1]) } else { None },
        if x < (tiles.len() - 1) {
            Some(tiles[x + 1][y])
        } else {
            None
        },
        if y < (tiles[x].len() - 1) {
            Some(tiles[x][y + 1])
        } else {
            None
        },
        if x > 0 { Some(tiles[x - 1][y]) } else { None },
    ]
}

impl Tilemap {
    pub fn borders(&self) -> &Vec<Entity> {
        return &self.borders;
    }

    pub fn create(ecs: &mut ECS) {
        let mut tiles: Vec<Vec<Entity>> = vec![Vec::with_capacity(9); 11];
        let mut center_tile = None;
        let mut borders = vec![];
        for x in 0..11 {
            for y in 0..9 {
                let tile = if x == 0 || x == 10 || y == 0 || y == 8 {
                    let e = Tile::create_image_tile(
                        "./assets/bitmap/grass2.png".to_string(),
                        ecs,
                        x,
                        y,
                    );
                    borders.push(e);
                    e
                } else {
                    Tile::create_image_tile("./assets/bitmap/grass.png".to_string(), ecs, x, y)
                };
                if x == 5 && y == 4 {
                    center_tile = Some(tile.clone());
                }
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
        ecs.build_entity()
            .add_component(capture_area)
            .add_component(ActiveCaptureLayer {});
        let res = Tilemap { tiles, borders };
        ecs.add_res(res);

        Hearth::create(ecs);
        ecs.write::<Tile>(&center_tile.unwrap()).unwrap().occupied = true;
    }

    pub fn tile_at_pos(&self, x: usize, y: usize) -> Entity {
        self.tiles[x][y].clone()
    }

    pub fn pos_of_tile(&self, tile: Entity) -> (f32, f32) {
        let mut x = 0.;
        for column in &self.tiles {
            let mut y = 0.;
            for t in column {
                if tile == *t {
                    return (x - 5., y - 3.);
                }
                y += 1.;
            }
            x += 1.;
        }
        return (0., 0.);
    }
}
