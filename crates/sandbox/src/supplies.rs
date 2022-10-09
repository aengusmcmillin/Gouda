pub struct Supplies {
    pub gold: i32,
    pub wood: i32,
    pub stone: i32,
}

impl Supplies {
    pub fn new() -> Supplies {
        Supplies {
            gold: 0,
            wood: 0,
            stone: 0,
        }
    }

    pub fn spend_supplies(&mut self, gold: i32, wood: i32, stone: i32) -> bool {
        if gold > self.gold || wood > self.wood || stone > self.stone {
            return false;
        }
        self.gold -= gold;
        self.wood -= wood;
        self.stone -= stone;
        return true;
    }

    pub fn _add_gold(&mut self, gold: i32) {
        self.gold += gold;
    }

    pub fn add_wood(&mut self, wood: i32) {
        self.wood += wood;
    }

    pub fn _add_stone(&mut self, stone: i32) {
        self.stone += stone;
    }
}
