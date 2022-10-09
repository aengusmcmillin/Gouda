use crate::input::{GameButtonState, KeyboardInput, SpecialKeys, LetterKeys, NumberKeys};
use winapi::um::winuser::{VK_UP, VK_LEFT, VK_DOWN, VK_RIGHT, VK_SPACE, VK_RETURN};

pub fn win32_process_keyboard(keyboard: &mut KeyboardInput, vkcode: i32, was_down: bool, is_down: bool) {
    if was_down != is_down {
        if vkcode == VK_UP {
            win32_process_keyboard_message(
                &mut keyboard.special_keys[SpecialKeys::UpArrow],
                is_down,
            );
        } else if vkcode == VK_LEFT {
            win32_process_keyboard_message(
                &mut keyboard.special_keys[SpecialKeys::LeftArrow],
                is_down,
            );
        } else if vkcode == VK_DOWN {
            win32_process_keyboard_message(
                &mut keyboard.special_keys[SpecialKeys::DownArrow],
                is_down,
            );
        } else if vkcode == VK_RIGHT {
            win32_process_keyboard_message(
                &mut keyboard.special_keys[SpecialKeys::RightArrow],
                is_down,
            );
        } else if vkcode == VK_RETURN {
            win32_process_keyboard_message(
                &mut keyboard.special_keys[SpecialKeys::Enter],
                is_down,
            );
        } else if vkcode == VK_SPACE {
            win32_process_keyboard_message(
                &mut keyboard.special_keys[SpecialKeys::Space],
                is_down,
            );
        } else {
            let key_char = vkcode as u8 as char;
            match key_char {
                'a' | 'A' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::A], is_down
                ),
                'b' | 'B' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::B], is_down
                ),
                'c' | 'C' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::C], is_down
                ),
                'd' | 'D' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::D], is_down
                ),
                'e' | 'E' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::E], is_down
                ),
                'f' | 'F' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::F], is_down
                ),
                'g' | 'G' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::G], is_down
                ),
                'h' | 'H' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::H], is_down
                ),
                'i' | 'I' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::I], is_down
                ),
                'j' | 'J' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::J], is_down
                ),
                'k' | 'K' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::K], is_down
                ),
                'l' | 'L' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::L], is_down
                ),
                'm' | 'M' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::M], is_down
                ),
                'n' | 'N' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::N], is_down
                ),
                'o' | 'O' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::O], is_down
                ),
                'p' | 'P' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::P], is_down
                ),
                'q' | 'Q' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::Q], is_down
                ),
                'r' | 'R' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::R], is_down
                ),
                's' | 'S' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::S], is_down
                ),
                't' | 'T' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::T], is_down
                ),
                'u' | 'U' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::U], is_down
                ),
                'v' | 'V' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::V], is_down
                ),
                'w' | 'W' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::W], is_down
                ),
                'x' | 'X' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::X], is_down
                ),
                'y' | 'Y' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::Y], is_down
                ),
                'z' | 'Z' => win32_process_keyboard_message(
                    &mut keyboard.letter_keys[LetterKeys::Z], is_down
                ),
                '0' => win32_process_keyboard_message(&mut keyboard.number_keys[NumberKeys::ZERO], is_down),
                '1' => win32_process_keyboard_message(&mut keyboard.number_keys[NumberKeys::ONE], is_down),
                '2' => win32_process_keyboard_message(&mut keyboard.number_keys[NumberKeys::TWO], is_down),
                '3' => win32_process_keyboard_message(&mut keyboard.number_keys[NumberKeys::THREE], is_down),
                '4' => win32_process_keyboard_message(&mut keyboard.number_keys[NumberKeys::FOUR], is_down),
                '5' => win32_process_keyboard_message(&mut keyboard.number_keys[NumberKeys::FIVE], is_down),
                '6' => win32_process_keyboard_message(&mut keyboard.number_keys[NumberKeys::SIX], is_down),
                '7' => win32_process_keyboard_message(&mut keyboard.number_keys[NumberKeys::SEVEN], is_down),
                '8' => win32_process_keyboard_message(&mut keyboard.number_keys[NumberKeys::EIGHT], is_down),
                '9' => win32_process_keyboard_message(&mut keyboard.number_keys[NumberKeys::NINE], is_down),
                _ => {
                    println!("c: '{}'", key_char);
                }
            }
        }
    }
}

pub fn win32_process_keyboard_message(new_state: &mut GameButtonState, is_down: bool) {
    if new_state.ended_down != is_down {
        new_state.ended_down = is_down;
        new_state.half_transition_count += 1;
    }
}

fn win32_process_xinput_digital_button(
    xinput_button_state: u16,
    old_state: &GameButtonState,
    new_state: &mut GameButtonState,
    button_bit: u16,
) {
    new_state.ended_down = (xinput_button_state & button_bit) == button_bit;
    new_state.half_transition_count = if old_state.ended_down != new_state.ended_down {
        1
    } else {
        0
    };
}

fn win32_process_xinput_stick_values(value: f32, dead_zone_threshold: f32) -> f32 {
    if value < -dead_zone_threshold {
        return (value + dead_zone_threshold) / (32768f32 - dead_zone_threshold);
    } else if value > dead_zone_threshold {
        return (value - dead_zone_threshold) / (32767f32 - dead_zone_threshold);
    } else {
        return 0f32;
    }
}
