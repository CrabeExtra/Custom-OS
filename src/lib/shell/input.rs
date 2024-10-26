use pc_keyboard::KeyCode;
use crate::print;
use crate::data::print::WRITER;


pub fn handle_char_input(character: char) {
    
    match character {
        '\u{8}' => print!("Backspc"),
        '\t' => handle_tab(),
        _ => print!("{}", character)
    }
}

fn handle_tab() {
    print!("    ");
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