// defining my imports here.
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Your panic handling code here, maybe logging the panic info or halting the system
    loop {}
}

pub mod data {
    pub mod print_data;
}

pub mod r#impl {
    pub mod x86_64 {
        pub mod print;
    }
    pub mod kernel {
        pub mod main;
    }
}
