#![no_std]
#![no_main]
#![allow(non_snake_case)]

extern crate alloc;

use Tibi_OS::println;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {

    Tibi_OS::init(boot_info);

    Tibi_OS::hlt_loop();
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    Tibi_OS::hlt_loop();
}

