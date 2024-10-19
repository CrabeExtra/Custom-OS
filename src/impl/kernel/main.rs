

use core::panic::PanicInfo;
use core::fmt::Write;
use crate::r#impl::x86_64::print;
use crate::data::print_data::PrintColor;
// syntax for compiling bytstrings; static HELLO: &[u8] = b"Hello World!";
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print::print_str("PANIC REACHED");
    print::print_str(_info.message().as_str().unwrap() as &str);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Your Rust kernel initialization code here
    print::print_clear();
    print::print_set_color(PrintColor::Yellow as u8, PrintColor::Black as u8);
    print::print_str("Welcome to Vessel (vessel for some of my programming that is)" as &str);
    print::print_str("\n");

    loop {}
}