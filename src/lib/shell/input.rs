use pc_keyboard::KeyCode;
use crate::print;
use crate::data::print::WRITER;
use crate::lib::shell::key_handlers;

pub fn handle_char_input(character: char) {
    match character {
        '\u{8}' => print!("Backspc"),
        '\t' => key_handlers::tab(),
        '\n' => key_handlers::enter(),
        _ => print!("{}", character)
    }
}

pub fn handle_key_input(key: KeyCode) {
    match key {
        KeyCode::ArrowDown => WRITER.lock().shift_downwards(),
        KeyCode::ArrowRight => WRITER.lock().shift_right(),
        KeyCode::ArrowLeft => WRITER.lock().shift_left(),
        KeyCode::ArrowUp => WRITER.lock().shift_upwards(),
        _ => print!("{:?}", key)
    }
}