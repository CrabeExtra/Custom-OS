#![no_std]
#![no_main]

// defining my imports here.

pub mod data {
    pub mod print_data;
    pub mod print;
}

pub mod lib {
    pub mod core {
        pub mod timing;
    }
}

pub mod r#impl {
    pub mod unsafe_print;
    
    pub mod kernel {
        pub mod main;
    }
}
