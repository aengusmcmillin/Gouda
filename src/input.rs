#[derive(Default, Clone)]
pub struct GameButtonState {
    pub half_transition_count: i32,
    pub ended_down: bool,
}

#[derive(Enum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LetterKeys {
    A, B, C, D, E, F, G, H, I,
    J, K, L, M, N, O, P, Q, R,
    S, T, U, V, W, X, Y, Z,
}

use self::LetterKeys::*;
use std::slice::Iter;

impl LetterKeys {
    pub fn iterator() -> Iter<'static, LetterKeys> {
        static LETTERS: [LetterKeys; 26] = [
            A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
        ];
        LETTERS.into_iter()
    }
}

#[derive(Enum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NumberKeys {
    ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE
}

use NumberKeys::*;
use enum_map::EnumMap;

impl NumberKeys {
    pub fn iterator() -> Iter<'static, NumberKeys> {
        static NUMBERS: [NumberKeys; 10] = [
            ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE
        ];
        NUMBERS.into_iter()
    }

    pub fn from_usize(val: usize) -> NumberKeys {
        static NUMBERS: [NumberKeys; 10] = [
            ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE
        ];
        assert!(val < 10, "Invalid Number");
        return NUMBERS[val];
    }
}

#[derive(Enum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SpecialKeys {
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    Enter,
    Space,
}

#[derive(Default, Clone)]
pub struct KeyboardInput {
    pub number_keys: EnumMap<NumberKeys, GameButtonState>,
    pub letter_keys: EnumMap<LetterKeys, GameButtonState>,
    pub special_keys: EnumMap<SpecialKeys, GameButtonState>,

    pub cmd_down: bool,
    pub alt_down: bool,
    pub ctrl_down: bool,
}

pub enum AnyKey {
    Letter(LetterKeys),
    Special(SpecialKeys),
    Number(NumberKeys),
}

impl KeyboardInput {
    pub fn from(old_keyboard: &KeyboardInput) -> KeyboardInput {
        let mut new_keyboard = KeyboardInput::default();
        for i in 0..10 {
            let num = NumberKeys::from_usize(i);
            new_keyboard.number_keys[num].ended_down = old_keyboard.number_keys[num].ended_down;
        }

        for key in LetterKeys::iterator() {
            new_keyboard.letter_keys[*key].ended_down = old_keyboard.letter_keys[*key].ended_down;
        }

        new_keyboard.special_keys[SpecialKeys::LeftArrow].ended_down =
            old_keyboard.special_keys[SpecialKeys::LeftArrow].ended_down;
        new_keyboard.special_keys[SpecialKeys::RightArrow].ended_down =
            old_keyboard.special_keys[SpecialKeys::RightArrow].ended_down;
        new_keyboard.special_keys[SpecialKeys::DownArrow].ended_down =
            old_keyboard.special_keys[SpecialKeys::DownArrow].ended_down;
        new_keyboard.special_keys[SpecialKeys::UpArrow].ended_down =
            old_keyboard.special_keys[SpecialKeys::UpArrow].ended_down;
        new_keyboard.special_keys[SpecialKeys::Space].ended_down =
            old_keyboard.special_keys[SpecialKeys::Space].ended_down;
        new_keyboard.special_keys[SpecialKeys::Enter].ended_down =
            old_keyboard.special_keys[SpecialKeys::Enter].ended_down;

        new_keyboard.cmd_down = old_keyboard.cmd_down;
        new_keyboard.alt_down = old_keyboard.alt_down;
        new_keyboard.ctrl_down = old_keyboard.ctrl_down;

        new_keyboard
    }

    pub fn number_pressed(&self, number: usize) -> bool {
        return self.number_keys[NumberKeys::from_usize(number)].ended_down
            && self.number_keys[NumberKeys::from_usize(number)].half_transition_count > 0;
    }

    pub fn letter_pressed(&self, letter: LetterKeys) -> bool {
        return self.letter_keys[letter].ended_down
            && self.letter_keys[letter].half_transition_count > 0;
    }

    pub fn letter_down(&self, letter: LetterKeys) -> bool {
        return self.letter_keys[letter].ended_down;
    }

    pub fn special_key_pressed(&self, special: SpecialKeys) -> bool {
        return self.special_keys[special].ended_down
            && self.special_keys[special].half_transition_count > 0;
    }
}

#[derive(Default, Clone)]
pub struct GameControllerInput {
    pub is_connected: bool,
    pub is_analog: bool,

    pub stick_average_x: f32,
    pub stick_average_y: f32,

    pub move_up: GameButtonState,
    pub move_down: GameButtonState,
    pub move_left: GameButtonState,
    pub move_right: GameButtonState,

    pub action_up: GameButtonState,
    pub action_down: GameButtonState,
    pub action_left: GameButtonState,
    pub action_right: GameButtonState,

    pub left_shoulder: GameButtonState,
    pub right_shoulder: GameButtonState,

    pub back: GameButtonState,
    pub start: GameButtonState,
}

impl GameControllerInput {
    pub fn from(old_controller: &GameControllerInput) -> GameControllerInput {
        let mut new_controller = GameControllerInput::default();
        new_controller.move_up.ended_down = old_controller.move_up.ended_down;
        new_controller.move_down.ended_down = old_controller.move_down.ended_down;
        new_controller.move_left.ended_down = old_controller.move_left.ended_down;
        new_controller.move_right.ended_down = old_controller.move_right.ended_down;

        new_controller.action_up.ended_down = old_controller.action_up.ended_down;
        new_controller.action_down.ended_down = old_controller.action_down.ended_down;
        new_controller.action_left.ended_down = old_controller.action_left.ended_down;
        new_controller.action_right.ended_down = old_controller.action_right.ended_down;

        new_controller.left_shoulder.ended_down = old_controller.left_shoulder.ended_down;
        new_controller.right_shoulder.ended_down = old_controller.right_shoulder.ended_down;

        new_controller.back.ended_down = old_controller.back.ended_down;
        new_controller.start.ended_down = old_controller.start.ended_down;

        new_controller
    }
}

#[derive(Default, Clone)]
pub struct Mouse {
    pub buttons: [GameButtonState; 5],
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Mouse {
    pub fn from(old_mouse: &Mouse) -> Mouse {
        let mut new_mouse = Mouse::default();

        new_mouse.x = old_mouse.x;
        new_mouse.y = old_mouse.y;
        new_mouse.z = old_mouse.z;

        new_mouse.buttons[0].ended_down = old_mouse.buttons[0].ended_down;
        new_mouse.buttons[1].ended_down = old_mouse.buttons[1].ended_down;
        new_mouse.buttons[2].ended_down = old_mouse.buttons[2].ended_down;
        new_mouse.buttons[3].ended_down = old_mouse.buttons[3].ended_down;
        new_mouse.buttons[4].ended_down = old_mouse.buttons[4].ended_down;

        new_mouse
    }
}

#[derive(Default, Clone)]
pub struct GameInput {
    pub seconds_to_advance_over_update: f32,
    pub mouse: Mouse,
    pub keyboard: KeyboardInput,
    pub controllers: [GameControllerInput; 5],
}

impl GameInput {
    pub fn new() -> Self {
        GameInput::default()
    }

    pub fn from(old_input: &GameInput) -> GameInput {
        GameInput {
            seconds_to_advance_over_update: old_input.seconds_to_advance_over_update,
            mouse: Mouse::from(&old_input.mouse),
            keyboard: KeyboardInput::from(&old_input.keyboard),
            controllers: [
                GameControllerInput::from(&old_input.controllers[0]),
                GameControllerInput::from(&old_input.controllers[1]),
                GameControllerInput::from(&old_input.controllers[2]),
                GameControllerInput::from(&old_input.controllers[3]),
                GameControllerInput::from(&old_input.controllers[4]),
            ]
        }
    }
}