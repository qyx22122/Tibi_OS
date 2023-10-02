#![no_std]
#![no_main]

use core::panic::PanicInfo;
use Tibi_OS::println;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    println!("Hello World{}", "!");



    Tibi_OS::init();

    

    Tibi_OS::hlt_loop();
}


//Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("\n!!! {} !!!", info);
    Tibi_OS::hlt_loop();
}
