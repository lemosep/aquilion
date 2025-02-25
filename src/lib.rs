#![no_std]
#![no_main]

//=================================================
// Imports
//=================================================

#[macro_use]
mod vga_buffer;
use core::panic::PanicInfo;

//=================================================
// Standalone Functions
//=================================================

#[no_mangle]
pub extern "C" fn kmain() {
    vga_buffer::clear_screen();
    println!("Hello, Aquilion!");
    loop{}
}

#[panic_handler] 
fn panic(_info: &PanicInfo) -> ! {
    loop{

    }
}