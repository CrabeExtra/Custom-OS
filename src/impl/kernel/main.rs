use crate::r#impl::x86_64::print;
use crate::data::print_data::PrintColor;

#[no_mangle]
pub extern "C" fn kernel_main() {
    // Your Rust kernel initialization code here
    print::print_str("TEST");
    print::print_clear();
    print::print_set_color(PrintColor::Yellow as u8, PrintColor::Black as u8);
    print::print_str("Welcome to Jude OS");
}
