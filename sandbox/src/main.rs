use gouda::{Gouda, GameLogic};

struct Game;

impl GameLogic for Game {

}

fn main() {
    let gouda = Gouda::new(Game {});
    gouda.run();
}
