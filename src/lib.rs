#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]



extern crate alloc;

use core::panic::PanicInfo;


pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod vga_buffer;
pub mod terminal;

use crate::terminal::Terminal;

pub fn init() {
    gdt::init();
    let terminal: &Terminal = unsafe { &interrupts::init_idt() };
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    terminal.init();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
