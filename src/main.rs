#![no_std]
#![no_main]
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


mod vga_buffer;




#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Version: Pre-alpha 0.0.0.a");
    println!("Check Back Later :)");
    loop {}
}