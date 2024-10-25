

use core::panic::PanicInfo;
use bootloader::{ BootInfo, entry_point };

use crate::data::print::{ print, set_colors };
use crate::data::print_data::PrintColor;
use crate::println;
use crate::lib::err::inits::{init, hlt_loop};
use x86_64::VirtAddr;
use crate::lib::threading::task::{Task, simple_executor::SimpleExecutor};

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

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
    use crate::lib::core::allocator; // new import
    use crate::lib::core::memory::{self, BootInfoFrameAllocator};
    
    init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let heap_value = Box::new(41);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

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


async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

