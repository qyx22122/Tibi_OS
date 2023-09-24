#![no_std]
#![feature(abi_x86_interrupt)]


pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod terminal;

use crate::terminal::Terminal;

pub fn init(){
    gdt::init();

    let terminal: &Terminal = unsafe { &interrupts::init_idt() };

    // Initialize PIC
    unsafe { interrupts::PICS.lock().initialize() };

    // Enable Interrupts
    x86_64::instructions::interrupts::enable();

    terminal.init();
}

// CPU hlt loop
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}