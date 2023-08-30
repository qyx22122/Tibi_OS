#![no_std]
#![no_main]


use core::panic::PanicInfo;


mod vga_buffer;



#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello!");
    panic!("hello error");

    loop {}
}




#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    print!("\n!!! {} !!!\n", info);
    loop {}
}
