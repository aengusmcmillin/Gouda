use gouda_ecs::ECS;
use gouda_rendering::Scene;

pub trait Layer {
    fn setup(&mut self, gouda: &ECS);
    fn update(&mut self, ecs: &ECS, dt: f32);
    fn render(&mut self, ecs: &mut ECS, scene: &mut Scene);
}
