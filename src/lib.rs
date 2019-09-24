pub trait GameLogic {
}

pub struct Gouda<T: GameLogic> {
    game_logic: T,
}

impl <T: GameLogic> Gouda<T> {
    pub fn new(game_logic: T) -> Self {
        Gouda {
            game_logic,
        }
    }

    pub fn run(&self) {
        loop {

        }
    }
}