use gouda::ecs::ECS;
use gouda::rendering::sprites::SpriteComponent;
use gouda::transform::TransformComponent;

#[derive(Debug)]
pub struct Hearth {}

impl Hearth {
    pub fn create(ecs: &mut ECS) {
        let sprite = SpriteComponent::new(ecs, "./assets/bitmap/hearth.png".to_string());
        let transform = TransformComponent::builder()
            .position(0., 1.)
            .scale(0.8, 0.8)
            .build();
        let hearth = Hearth {};
        ecs.build_entity().add_component(hearth).add_component(sprite).add_component(transform);
    }
}
