

use core::panic::PanicInfo;
use x86_64::instructions::hlt;

use crate::data::print::{clear, clear_row, print, print_int_32, set_colors};
use crate::data::print_data::PrintColor;
use crate::println;
use crate::lib::err::inits::{init, hlt_loop};

// syntax for compiling bytstrings; static HELLO: &[u8] = b"Hello World!";
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    set_colors(PrintColor::LightRed , PrintColor::White);
    println!("PANIC REACHED");
    println!("{}", info.message());
    set_colors(PrintColor::White, PrintColor::Black);
    println!("...");
    hlt_loop();
}   

// just the standard entrypoint for background assembly.
#[no_mangle]
pub extern "C" fn _start() -> ! {

    init();

    // invoke a breakpoint exception
    //x86_64::instructions::interrupts::int3();

    clear();
    // Your Rust kernel initialization code here
    display_os();
    print("Welcome to Vessel (vessel for some of my programming that is)");
    print("\n");
    hlt_loop();
}

fn display_os() {
    for _col in 0..34 {
        print(" ");
    }
    print("VESSEL OS\n");
}


