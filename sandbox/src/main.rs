use gouda::{Gouda, GameLogic};
use gouda::ecs::{ECS, Mutations, Mutation, Entity};
use gouda::rendering::{Scene, QuadDrawable};

#[derive(Debug)]
struct TestComponent {
    pub processed: bool,
}

struct TestMutation {
    entity: Entity,
}

impl Mutation for TestMutation {
    fn apply(&self, ecs: &mut ECS) {
        ecs.write::<TestComponent>(&self.entity).unwrap().processed = true;
        println!("Applied mutation");
    }
}

fn test_system(ecs: &ECS) -> Mutations {
    let mut mutations: Mutations = Vec::new();
    for (c, ent) in ecs.read1::<TestComponent>() {
        if !c.processed {
            mutations.push(Box::new(TestMutation {entity: ent}));
        }
    }
    return mutations;
}

#[derive(Debug)]
struct Tile {
    drawable: QuadDrawable,
}

impl Tile {
}

struct Player {
    drawable: QuadDrawable,
}



struct Game;

impl GameLogic for Game {
    fn register_components(&self, ecs: &mut ECS) {
        ecs.register_component_type::<TestComponent>()
    }

    fn register_systems(&self, ecs: &mut ECS) {
        ecs.add_system(Box::new(test_system));
    }

    fn setup(&self, ecs: &mut ECS) {
        ecs.build_entity().add(TestComponent{processed: false});
    }

    fn draw_scene(&self, ecs: &ECS, scene: &Scene) {
    }
}

fn main() {
    let mut gouda = Gouda::new(Game {});
    gouda.run();
}
