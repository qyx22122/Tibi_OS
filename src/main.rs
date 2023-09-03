#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(Tibi_OS::test_runner)]
#![reexport_test_harness_main = "test_main"]



use core::panic::PanicInfo;
use Tibi_OS::println;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    for i in 0..100+1{
       println!("Hello! {}",i); 
    }
    
    
    Tibi_OS::init();

    x86_64::instructions::interrupts::int3();


    #[cfg(test)]
    test_main();
    
    loop {}
}


//Panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("\n!!! {} !!!", info);
    loop{}
}

//Panic handler in test

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Tibi_OS::test_panic_handler(info)
}


