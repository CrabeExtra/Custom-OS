// defining my imports here.
#![no_std]
#![no_main]

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
