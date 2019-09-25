use crate::ecs::ECS;

pub mod ecs;

pub trait GameLogic {
    fn register_components(&self, ecs: &mut ECS);
    fn register_systems(&self, ecs: &mut ECS);
    fn setup(&self, ecs: &mut ECS);
}

pub struct Gouda<T: GameLogic> {
    game_logic: T,
    ecs: ECS,
}

impl <T: GameLogic> Gouda<T> {
    pub fn new(game_logic: T) -> Self {
        Gouda {
            game_logic,
            ecs: ECS::new(),
        }
    }

    pub fn run(&mut self) {
        self.game_logic.register_components(&mut self.ecs);
        self.game_logic.register_systems(&mut self.ecs);

        self.game_logic.setup(&mut self.ecs);

        loop {
            self.ecs.run_systems();
        }
    }
}