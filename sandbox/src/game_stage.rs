pub struct NightStage {

}

impl NightStage {
    pub fn next_day(&self) {

    }
}

pub struct DayStage {

}

pub enum GameStage {
    Day(DayStage),
    Night(NightStage)
}

impl GameStage {
    pub fn next_stage(&self) -> GameStage {
        match self {
            GameStage::Day(daystage) => {
                GameStage::Night(NightStage {})
            },
            GameStage::Night(nigthstage) => {
                GameStage::Day(DayStage {})
            }
        }
    }
}

