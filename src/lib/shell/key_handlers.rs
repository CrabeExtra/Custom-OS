use crate::data::print::WRITER;
use crate::data::print::BUFFER_WIDTH;
use crate::print;

pub fn tab() {
    print!("    ");
}

pub fn enter() {
    let line = WRITER.lock().get_line_contents();
    for c in line {
        print!("{}", c);
    }
    print!("\n"); // TODO: change this to use the inbuilt print newline functionality.
    
}