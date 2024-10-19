

use core::panic::PanicInfo;
use crate::r#impl::x86_64::print;
use crate::data::print_data::PrintColor;
// syntax for compiling bytstrings; static HELLO: &[u8] = b"Hello World!";
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Your Rust kernel initialization code here
    print::print_str("TEST" as &str);
    print::print_clear();
    print::print_set_color(PrintColor::Yellow as u8, PrintColor::Black as u8);
    print::print_str("Welcome to Jude OS" as &str);

    loop {}
}