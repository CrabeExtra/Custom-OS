use crate::lib::err::{gdt, interrupts};

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();     // new
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}