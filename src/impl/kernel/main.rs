

use core::panic::PanicInfo;
use crate::data::print::{clear, clear_row, print, print_int_32, set_colors};
use crate::data::print_data::PrintColor;
use crate::println;


// syntax for compiling bytstrings; static HELLO: &[u8] = b"Hello World!";
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    set_colors(PrintColor::LightRed, PrintColor::White);
    println!("PANIC REACHED");
    println!("{}", info.message());
    set_colors(PrintColor::White, PrintColor::Black);
    println!("...");
    loop {}
}   

// just the standard entrypoint for background assembly.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("test");
    clear();
    // Your Rust kernel initialization code here
    display_os();
    print("Welcome to Vessel (vessel for some of my programming that is)");
    print("\n");
    let mut count: i32 = 1;
    while count <= 20000  {
        println!("Hello World{}", count);
        count += 1;
    }
    
    loop {}
}

fn display_os() {
    for _col in 0..34 {
        print(" ");
    }
    print("VESSEL OS\n");
}