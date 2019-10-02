use gouda::{Gouda, GameLogic};
use gouda::ecs::{ECS, Mutations, Mutation, Entity};
use gouda::rendering::{Scene, QuadDrawable, Renderer};
use std::rc::Rc;
use std::ops::Deref;
use gouda::input::{LetterKeys, GameInput};

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

#[derive(Debug)]
struct Player {
    drawable: QuadDrawable,
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct GuiElement {
    drawable: QuadDrawable,
}

struct MoveMutation {
    entity: Entity,
    dx: f32,
    dy: f32,
}

impl Mutation for MoveMutation {
    fn apply(&self, ecs: &mut ECS) {
        let player = ecs.write::<Player>(&self.entity).unwrap();
        player.x += self.dx;
        player.y += self.dy;
        player.drawable.translate([player.x, player.y, 0.], [0.05, 0.05, 1.]);
    }
}

fn player_move_system(ecs: &ECS) -> Mutations {
    let input = ecs.read_res::<GameInput>();
    let mut mutations: Mutations = Vec::new();
    for (p, ent) in ecs.read1::<Player>() {
        if input.keyboard.letter_pressed(LetterKeys::A) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: -0.182, dy: 0.}))
        } else if input.keyboard.letter_pressed(LetterKeys::D) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: 0.182, dy: 0.}))
        }
        if input.keyboard.letter_pressed(LetterKeys::W) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: 0., dy: 0.179}))
        } else if input.keyboard.letter_pressed(LetterKeys::S) {
            mutations.push(Box::new(MoveMutation {entity: ent, dx: 0., dy: -0.179}))
        }
    }
    return mutations;
}



struct Game {
}

impl Game {
    pub fn new() -> Self {
        Game {
        }
    }
}

impl GameLogic for Game {
    fn register_components(&self, ecs: &mut ECS) {
        ecs.register_component_type::<TestComponent>();
        ecs.register_component_type::<Tile>();
        ecs.register_component_type::<Player>();
        ecs.register_component_type::<GuiElement>();
    }

    fn register_systems(&self, ecs: &mut ECS) {
        ecs.add_system(Box::new(test_system));
        ecs.add_system(Box::new(player_move_system));
    }

    fn setup(&mut self, ecs: &mut ECS) {
        ecs.build_entity().add(TestComponent{processed: false});

        for x in 0..11 {
            for y in 0..9 {
                let color = if x == 5 && y == 4 {
                    [0.5, 0.2, 0.2]
                } else if x == 0 || x == 10 || y == 0 || y == 8 {
                    [0.5, 0.5, 0.5]
                } else {
                    [0.2, 0.4, 0.3]
                };
                let renderer = ecs.read_res::<Rc<Renderer>>();
                let quad = QuadDrawable::new(renderer, color, [-0.91 + x as f32 * 0.182, -0.52 + y as f32 * 0.179, 0.], [0.08, 0.08, 1.]);
                let tile = Tile {
                    drawable: quad,
                };
                ecs.build_entity().add(tile);
            }
        }

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let player_drawable = QuadDrawable::new(renderer, [0.7, 0.7, 0.7], [-0.364, -0.165, 0.], [0.05, 0.05, 1.]);
        ecs.build_entity().add(Player {drawable: player_drawable, x: -0.364, y: -0.165});

        let renderer = ecs.read_res::<Rc<Renderer>>();
        let bottom = QuadDrawable::new(renderer, [0.1, 0.1, 0.1], [-0., -0.80, 0.], [0.99, 0.19, 1.]);
        ecs.build_entity().add(GuiElement {drawable: bottom});
    }

    fn draw_scene(&self, ecs: &ECS, scene: &Scene) {
        for (tile, e) in ecs.read1::<Tile>() {
            tile.drawable.draw(&scene);
        }

        for (player, e) in ecs.read1::<Player>() {
            player.drawable.draw(&scene);
        }

        for (gui, e) in ecs.read1::<GuiElement>() {
            gui.drawable.draw(&scene);
        }
    }
}

fn main() {
    let mut gouda = Gouda::new(Game::new());
    gouda.run();
}
