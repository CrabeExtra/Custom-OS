#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

// defining my imports here.

pub mod data {
    pub mod print_data;
    pub mod print;
}

pub mod lib {
    pub mod core {
        pub mod timing;
    }
    pub mod err {
        pub mod gdt;
        pub mod interrupts;
        pub mod inits;
    }
}


pub mod r#impl {
    pub mod unsafe_print;
    
    pub mod kernel {
        pub mod main;
    }
}


