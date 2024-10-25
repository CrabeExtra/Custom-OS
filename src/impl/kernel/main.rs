

use core::panic::PanicInfo;
use bootloader::{ BootInfo, entry_point };

use crate::data::print::{ print, set_colors };
use crate::data::print_data::PrintColor;
use crate::println;
use crate::lib::err::inits::{init, hlt_loop, initialise_heap};  
use crate::lib::threading::task::{Task, simple_executor::SimpleExecutor, executor::Executor};
use crate::lib::threading::task::keyboard;

extern crate alloc;

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
    display_os();
    print("Welcome to Vessel (vessel for some of my programming that is)");
    print("\n");
    init();
    
    initialise_heap(boot_info);

    let mut executor = Executor::new(); // new
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

fn display_os() {
    for _col in 0..34 {
        print(" ");
    }
    print("VESSEL OS\n");
}


async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}



