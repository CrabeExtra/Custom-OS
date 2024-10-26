// a lot of the code functionally duplicates what is already written in src/impl/unsafe_print.rs
// but this code is far more memory safe and up to rust compiler standards.
// on the flip side, the C-like direct mem logic in 'unsafe_print' is likely faster.

// crediting https://os.phil-opp.com/vga-text-mode/ for this code.

use volatile::Volatile;
use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::data::print_data::PrintColor;

// constants, ("ooh, but you could just import them from unsafe_print.rs", grow up mate)
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

const TOTAL_HEIGHT: usize = 51; //60000; // approx bit less than ~10MB worth of RAM allocating here (assuming constant 80 width 2xu8 chars.)

const NUM_RESERVED_INDICES: usize = 1;

// Color code. This takes the color bits for fore and background,
// these are 4 bits each but Rust's smallest bitwise prim is u8
// background is shifted 4 to the left, then foreground is as is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: PrintColor, background: PrintColor) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8)) // first 4 bytes (larger vals) are background, and last 4 bytes are foreground
    }
}

// (ColorCode is literally u8)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    fn default() -> Self {
        ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode((0 as u8) << 4 | (7 as u8)),
        }
    }
}

// Heap memory allocated to total text array.
// Because this is static, it will use the same pre-allocated memory location for the entire duration that the operating system is online.
static mut TEXT_MEMORY: [[ScreenChar; BUFFER_WIDTH]; TOTAL_HEIGHT] = [[ScreenChar { ascii_character: 0, color_code: ColorCode(0) }; BUFFER_WIDTH]; TOTAL_HEIGHT];

// 2D Array Buffer, really nice way of representing the write position.
#[repr(transparent)]
struct GraphicsBuffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[repr(transparent)]
struct TextBuffer {
    data: &'static mut [[ScreenChar; BUFFER_WIDTH]; TOTAL_HEIGHT],
}
   

impl TextBuffer {
    fn new() -> Option<TextBuffer> {
        unsafe {
            Some(TextBuffer { data: &mut TEXT_MEMORY })
        }
    }
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut GraphicsBuffer,
    all_text: TextBuffer,
    row_window_upper: usize, 
    first_row: usize,
    text_heap_full: u8
}

impl Writer {
    // This is a bit inefficient as well
    // I could find the row position of the vga buffer by offsetting by the row upper window
    // TODO^^ but this is a bit of effort because of edge cases (it will mean I can set in all data, then 
    // set directly in the buffer, then only have to update the exact spots instead of refresh the entire buffer).
    pub fn update_cursor_color(&mut self, prev_col: usize, prev_row: usize) {
        self.all_text.data[prev_row][prev_col].color_code = self.color_code;
        self.all_text.data[self.row_position][self.column_position].color_code = ColorCode::new(PrintColor::Black, PrintColor::White);
        self.refresh_window();
    }

    // write a singular byte to the screen based on Writer cursor and byte.
    // TODO: I want to write another function that shifts the col to the right and wraps to the next line
    //       For cases when I want to be a ble to write a line and not overwrite the previous line.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                let color_code = self.color_code;

                // add the data to the entire text section.
                self.all_text.data[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };

                // add to the 'same' spot in the buffer
                self.buffer.chars[self.get_buffer_row()][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                    

                self.column_position += 1;
            }
        }
    }

    fn get_buffer_row(&mut self) -> usize {
        // case where it's within buffer
        if self.row_window_upper <= self.row_position {
            return self.row_position - self.row_window_upper;
        
            // case where upper window is below the row
        } else {
            // I did some maths here just trust it
            return self.row_position + TOTAL_HEIGHT - self.row_window_upper;
        }
    }

    fn new_line(&mut self) {
        self.row_position += 1;
        self.column_position = 0;

        // so if the text has reached the end of the screen
        if self.row_position > self.row_window_upper + (BUFFER_HEIGHT - 1) || self.text_heap_full == 1 {
            self.row_window_upper += 1; // shift the window down a row.

            self.handle_newline_gt_total_height();
        } 

        self.refresh_window(); 
    }

    /**
     * This fn just handles cases where a new line is created but one of the 
     * row variables exceeds the max memory allocated to the text array.
     */
    fn handle_newline_gt_total_height(&mut self) {
        // if the upper bound of the row window is equal to the total height
        // reset to 0
        if self.row_window_upper > TOTAL_HEIGHT - 1 {
            self.row_window_upper = 0;
            self.text_heap_full = 0;
        }

        // ensure first row is shifted to make room for new rows
        if self.text_heap_full == 1 {
            self.clear_mem_row(self.row_position); // clear the row
            self.first_row = self.row_position + 1; // safe increment first_row to be further than the row_position
        }

         // if end of buffer total height reached, 
        if self.row_position > TOTAL_HEIGHT - 1 {
            self.row_position = 0; // reset row position
            self.clear_mem_row(self.row_position); // clear the 0th row.
            self.first_row = 1; // first row is now index 1 as index 0 is now the last row in the circular array.
            self.text_heap_full = 1;
        }

        // ensure first row loops back.
        if self.first_row > TOTAL_HEIGHT - 1 {
            self.first_row = 0;
        }
    }

    /**
     * TODO this is really inefficient, I should think up a better solution.
     *      The problem is when my ~ <=10MB text heap reaches its max line limits 
     *      I need to safely loop it back to 0. and start again, wiping mem line-by-line
     *      procedurally. That process itself is efficient, but I have to refresh the window
     *      literally anytime a new line is added to manage the moving buffer window.
     * 
     * it's actually big O(1) efficiency standalone (max 25 rows x 80 cols), 
     * but let's say I want to print n lines. Then it's 25 * 80 * n,
     * when only really one line is changing. that adds 25*80 char changes
     * per n lines. I can tangibly see how slow it is compared with a no-print counter.
     * 
     * Anyway I'll come back to this.
     * 
     * 
     */
    fn refresh_window(&mut self) {
        // loop through the display bounds and display the window from all chars in bounds.
        for row in NUM_RESERVED_INDICES..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {

                // if array length is exceeded, offset by difference between
                // end of array and print that row.
                let local_row: usize = row + self.row_window_upper;
                
                if local_row > TOTAL_HEIGHT - 1 {
                    
                    let printable_row = local_row - TOTAL_HEIGHT;
                    let character = self.all_text.data[printable_row][col];
                    self.buffer.chars[row][col].write(character);
                } else {

                    let character = self.all_text.data[row + self.row_window_upper][col];
                    self.buffer.chars[row][col].write(character);
                }
            }
        }
    }

    /**
     * clear row in just the VGA bytes
     */
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
    /**
     * clear row in the larger mem heap
     */
    fn clear_mem_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.all_text.data[row][col]= blank;
        }
    }
    /**
     * just in case required.
     * Resets the heap, the VGA bytes, and all associated writer fields.
     */
    fn reset_heap(&mut self) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            for row in 0..TOTAL_HEIGHT {
                self.all_text.data[row][col] = blank;
            }
            for row in 0..BUFFER_HEIGHT {
                self.buffer.chars[row][col].write(blank);
            }
        }
        self.text_heap_full = 0;
        self.column_position = 0;
        self.first_row = 0;
        self.row_position = 0;
        self.row_window_upper = 0;
    }

    // literally just loop through the bytes in s string and write them if possible.    
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }
    /**
     * deletes a key.
     */
    pub fn backspace(&mut self) {

        // produce required local vars
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        let row = self.row_position;
        let col = self.column_position;
        
        // delete the current key
        self.all_text.data[row][col] = blank;

        //  shifts the window back 1
        self.shift_back();
    }

    /**
     * shifts back 1 char, handles edge cases.
     */
    pub fn shift_back(&mut self) {
        // if cursor not at leftmost column, shift left.
        self.shift_left();
        
        // if cursor not at topmost row, shift upwards.
        self.shift_upwards();
        
    }

    /**
     * shifts forwards 1 char
     */
    pub fn shift_forwards(&mut self) {

    }

    pub fn shift_left(&mut self) {
        let col = self.column_position;
        let row = self.row_position;
        if self.column_position > 0 {
            self.column_position -= 1;
        }
        self.update_cursor_color(col, row);
    }

    // TODO update this to shift the entire buffer window up the text buffer.
    pub fn shift_upwards(&mut self) {
        let col = self.column_position;
        let row = self.row_position;
        if self.row_position > (self.row_window_upper + NUM_RESERVED_INDICES) {
            self.row_position -= 1;
        }
        self.update_cursor_color(col, row);
    }

    pub fn shift_right(&mut self) {
        let col = self.column_position;
        let row = self.row_position;
        if self.column_position < BUFFER_WIDTH - 1 {
            self.column_position += 1;
        }
        self.update_cursor_color(col, row);
    }
    // Shifts the cursor downwards and handles edgecase:
    // buffer (only if the newLine exists. Don't loop around or enter negative space).
    pub fn shift_downwards(&mut self) {
        let col = self.column_position;
        let row = self.row_position;
        // shift cursor down if all good to do so.
        if self.row_position < (self.row_window_upper + (BUFFER_HEIGHT - 1)) {
            self.row_position += 1;

            // are we about to overtake the first_row? generate a newline.
            // ERROR: here, resolve.
        } else if self.first_row == (self.row_position + 1) || self.row_position == TOTAL_HEIGHT - 1 || self.column_position >= BUFFER_WIDTH - 1 {
            
            // do nothing
            self.write_string("end of buffer");
            // we're shifting the buffer window
        } else {
            self.write_string("shift");
            self.shift_window_down();
        }

        self.update_cursor_color(col, row);
    }

    pub fn shift_window_down(&mut self) {
        self.row_position += 1;

        self.row_window_upper += 1; // shift the window down a row.

        self.refresh_window(); 
    }
}

// This just allows easy formatting of writing so ints, floats etc. can be printed as strings.
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub fn print(s: &str) {
    WRITER.lock().write_str(s).unwrap();
    //write!(writer, "{}", 1.0/0.5).unwrap();
    //write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
} 

pub fn set_colors(foreground: PrintColor, background: PrintColor) {
    WRITER.lock().color_code = ColorCode::new(foreground, background)
}

pub fn clear_row(row: usize) {
    WRITER.lock().clear_row(row);
}

pub fn clear() {
    for row in 0..BUFFER_HEIGHT {
        WRITER.lock().clear_row(row);
    }
   
}

pub fn print_int_32(i: i32) {
    write!(WRITER.lock(), "{}", i).unwrap();
}

pub fn print_float_64(f: f64) {
    write!(WRITER.lock(), "{}", f).unwrap();
}

pub fn print_u_64(u: u64) {
    write!(WRITER.lock(), "{}", u).unwrap();
}

// statics are initialised at compile time, but other variables (e.g. enum PrintColor)
// are initialised during runtime, so lazy static is required to initialise this static
// later on (when PrintColor is compiled and exists.)
lazy_static! {
    // Mutex blocks threads when a resource is already. (synchronised interior mutability)
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: 0,
        color_code: ColorCode::new(PrintColor::Yellow, PrintColor::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut GraphicsBuffer) },
        all_text: TextBuffer::new().unwrap(), // I really have no clue if Mutex will prevent errors here, fingies crossed.
        row_window_upper: 0,
        first_row: 0,
        text_heap_full: 0
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::data::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts; 

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}