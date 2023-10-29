#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![allow(non_snake_case)]

extern crate alloc;

pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod vga_buffer;
pub mod terminal;

use crate::terminal::Terminal;

use bootloader::BootInfo;

pub fn init(boot_info: &'static BootInfo) {
    gdt::init();
    let terminal: &Terminal = unsafe { &interrupts::init_idt() };
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    setup_allocation(boot_info);

    terminal.init();
}

fn setup_allocation(boot_info: &'static BootInfo)
{
    use memory::BootInfoFrameAllocator;
    use x86_64::VirtAddr;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
