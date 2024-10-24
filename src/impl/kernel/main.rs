

use core::panic::PanicInfo;
use bootloader::{ BootInfo, entry_point };
use x86_64::structures::paging::Translate;

use crate::data::print::{ print, set_colors };
use crate::data::print_data::PrintColor;
use crate::println;
use crate::lib::err::inits::{init, hlt_loop};

entry_point!(kernel_main);

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
pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::{structures::paging::Page, VirtAddr}; // new import
    use crate::lib::core::memory;
    init();
    
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


