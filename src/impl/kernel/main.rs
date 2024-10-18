use crate::impls::x86_64::print;

#![no_std]
#![no_main] 
#[no_mangle]
pub extern "C" fn kernel_main() {
    // Your Rust kernel initialization code here
    print_str("TEST");
    print_clear();
    print_set_color(PRINT_COLOR_YELLOW, PRINT_COLOR_BLACK);
    print_str("Welcome to Jude OS");
}
