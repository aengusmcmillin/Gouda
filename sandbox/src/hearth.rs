use gouda::TransformComponent;
use gouda::rendering::sprites::SpriteComponent;
use gouda::ecs::{Entity, ECS};
use crate::tilemap::Tile;

#[derive(Debug)]
pub struct Hearth {}

impl Hearth {
    pub fn create(ecs: &mut ECS, tile: Entity) {
        let sprite = SpriteComponent::new(ecs, "bitmap/hearth.png".to_string());
        let tile = ecs.read::<Tile>(&tile).unwrap();
        let transform = TransformComponent::builder().location(tile.x as f32, tile.y as f32).scale(0.4, 0.4).build();
        let hearth = Hearth {};
        ecs.build_entity().add(hearth).add(sprite).add(transform);
    }
}

