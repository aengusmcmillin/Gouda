use crate::tilemap::Tile;
use gouda::ecs::{Entity, ECS};
use gouda::rendering::sprites::SpriteComponent;
use gouda::transform::TransformComponent;

pub fn create_tree(ecs: &mut ECS, tile: Entity) {
    ecs.write::<Tile>(&tile).unwrap().occupied = true;

    let sprite = SpriteComponent::new(ecs, "./assets/bitmap/tree.png".to_string());
    let tile = ecs.read::<Tile>(&tile).unwrap();
    let loc = TransformComponent::builder()
        .position(tile.x as f32, tile.y as f32)
        .scale(0.4, 0.4)
        .build();
    let tree = TreeComponent { wood: 10 };
    ecs.build_entity().add_component(tree).add_component(sprite).add_component(loc);
}

#[derive(Debug)]
pub struct TreeComponent {
    wood: i32,
}

impl TreeComponent {
    pub fn harvest(&mut self) -> i32 {
        self.wood -= 10;
        return 10;
    }
}
