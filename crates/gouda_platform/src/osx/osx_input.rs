use crate::input::*;

pub fn osx_process_keyboard_message(new_state: &mut GameButtonState, is_down: bool) {
    if new_state.ended_down != is_down {
        new_state.ended_down = is_down;
        new_state.half_transition_count += 1;
    }
}

pub fn osx_process_key(keyboard: &mut KeyboardInput, u16_char: u16, key_down: bool) {
    match u16_char {
        0xF700 => {
            osx_process_keyboard_message(&mut keyboard.special_keys[SpecialKeys::UpArrow], key_down)
        }
        0xF701 => osx_process_keyboard_message(
            &mut keyboard.special_keys[SpecialKeys::DownArrow],
            key_down,
        ),
        0xF702 => osx_process_keyboard_message(
            &mut keyboard.special_keys[SpecialKeys::LeftArrow],
            key_down,
        ),
        0xF703 => osx_process_keyboard_message(
            &mut keyboard.special_keys[SpecialKeys::RightArrow],
            key_down,
        ),
        _ => {
            let decoded_char = std::char::decode_utf16(std::iter::once(u16_char))
                .next()
                .unwrap();
            match decoded_char {
                Ok(c) => match c {
                    ' ' => osx_process_keyboard_message(
                        &mut keyboard.special_keys[SpecialKeys::Space],
                        key_down,
                    ),
                    'a' | 'A' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::A],
                        key_down,
                    ),
                    'b' | 'B' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::B],
                        key_down,
                    ),
                    'c' | 'C' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::C],
                        key_down,
                    ),
                    'd' | 'D' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::D],
                        key_down,
                    ),
                    'e' | 'E' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::E],
                        key_down,
                    ),
                    'f' | 'F' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::F],
                        key_down,
                    ),
                    'g' | 'G' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::G],
                        key_down,
                    ),
                    'h' | 'H' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::H],
                        key_down,
                    ),
                    'i' | 'I' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::I],
                        key_down,
                    ),
                    'j' | 'J' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::J],
                        key_down,
                    ),
                    'k' | 'K' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::K],
                        key_down,
                    ),
                    'l' | 'L' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::L],
                        key_down,
                    ),
                    'm' | 'M' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::M],
                        key_down,
                    ),
                    'n' | 'N' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::N],
                        key_down,
                    ),
                    'o' | 'O' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::O],
                        key_down,
                    ),
                    'p' | 'P' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::P],
                        key_down,
                    ),
                    'q' | 'Q' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::Q],
                        key_down,
                    ),
                    'r' | 'R' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::R],
                        key_down,
                    ),
                    's' | 'S' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::S],
                        key_down,
                    ),
                    't' | 'T' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::T],
                        key_down,
                    ),
                    'u' | 'U' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::U],
                        key_down,
                    ),
                    'v' | 'V' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::V],
                        key_down,
                    ),
                    'w' | 'W' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::W],
                        key_down,
                    ),
                    'x' | 'X' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::X],
                        key_down,
                    ),
                    'y' | 'Y' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::Y],
                        key_down,
                    ),
                    'z' | 'Z' => osx_process_keyboard_message(
                        &mut keyboard.letter_keys[LetterKeys::Z],
                        key_down,
                    ),
                    '0' => osx_process_keyboard_message(
                        &mut keyboard.number_keys[NumberKeys::ZERO],
                        key_down,
                    ),
                    '1' => osx_process_keyboard_message(
                        &mut keyboard.number_keys[NumberKeys::ONE],
                        key_down,
                    ),
                    '2' => osx_process_keyboard_message(
                        &mut keyboard.number_keys[NumberKeys::TWO],
                        key_down,
                    ),
                    '3' => osx_process_keyboard_message(
                        &mut keyboard.number_keys[NumberKeys::THREE],
                        key_down,
                    ),
                    '4' => osx_process_keyboard_message(
                        &mut keyboard.number_keys[NumberKeys::FOUR],
                        key_down,
                    ),
                    '5' => osx_process_keyboard_message(
                        &mut keyboard.number_keys[NumberKeys::FIVE],
                        key_down,
                    ),
                    '6' => osx_process_keyboard_message(
                        &mut keyboard.number_keys[NumberKeys::SIX],
                        key_down,
                    ),
                    '7' => osx_process_keyboard_message(
                        &mut keyboard.number_keys[NumberKeys::SEVEN],
                        key_down,
                    ),
                    '8' => osx_process_keyboard_message(
                        &mut keyboard.number_keys[NumberKeys::EIGHT],
                        key_down,
                    ),
                    '9' => osx_process_keyboard_message(
                        &mut keyboard.number_keys[NumberKeys::NINE],
                        key_down,
                    ),
                    _ => {
                        println!("c: '{}'", c);
                    }
                },
                Err(hex) => {
                    println!("hex: {:X}", hex.unpaired_surrogate());
                }
            }
        }
    }
}
