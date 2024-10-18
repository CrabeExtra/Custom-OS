use crate::data::printDat;

const NUM_COLS: usize = 80;
const NUM_ROWS: usize = 25;

struct Char {
    character: u8;
    color: u8;
}

const BUFFER: *mut Char = 0xb8000 as *mut Char;

static mut COL: usize = 0;
static mut ROW: usize = 0;

static mut COLOR: u8 = PrintColor::White | (PrintColor::Black << 4);

fn clear_row(row: usize) {
    unsafe {
        let empty = Char {
            character: b' ',
            color: COLOR,
        };

        for (col in 0..NUM_COLS) {
            *BUFFER.add(col + NUM_COLS * row) = empty;
        }
    }
    
}

fn print_clear() {
    unsafe {
        for (i in 0..NUM_ROWS) {
            clear_row(i);
        }
    }
}

fn print_newline() {
    // just noting here that I'm wrapping global var usages in unsafe to bypass rust compiler.
    unsafe {
        COL = 0;
        if ROW < (NUM_ROWS - 1) {
            ROW += 1;
            return;
        }
        
        for (row in 0..NUM_ROWS) {
            for (col in 0..NUM_COLS) {
                let character: Char;
                character = *BUFFER.add(col + NUM_COLS * row);
                *BUFFER.add(col + NUM_COLS * (row - 1)) = character;
            }
        }

        clear_row(NUM_COLS - 1);
    }
}

fn print_char(character: char) {
    if character == '\n' {
        print_newline();
        return;
    }
    unsafe {
        if (COL > NUM_COLS) {
            print_newline();
        }
    
        *BUFFER.add(COL + NUM_COLS * ROW) = Char {
            character: character as u8,
            color: COLOR,
        };
    
        COL++;
    }
}

fn print_str(str: &str) {
    for character in str.bytes() {

        if character == 0 {
            return;
        }

        print_char(character as char);
    }
}

fn print_set_color(foreground: u8, background: u8) {
    unsafe {
        COLOR = foreground + (background << 4);
    }
}