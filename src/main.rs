#![no_std]
#![no_main]

use core::panic::PanicInfo;
use Tibi_OS::println;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    for i in 0..100+1{
       println!("Hello! {}",i); 
    }
    
    
    Tibi_OS::init();

    // x86_64::instructions::interrupts::int3();
    
    loop {}
}


//Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("\n!!! {} !!!", info);
    loop{}
}