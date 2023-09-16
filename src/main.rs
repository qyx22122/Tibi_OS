#![no_std]
#![no_main]

use core::panic::PanicInfo;
use Tibi_OS::println;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    println!("Hello World{}", "!");

    Tibi_OS::init();

    println!("Doesn't crash!");
    
    Tibi_OS::hlt_loop();
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations
}

//Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("\n!!! {} !!!", info);
    Tibi_OS::hlt_loop();
}