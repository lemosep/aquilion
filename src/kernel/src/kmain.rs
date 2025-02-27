#![no_std]
#![no_main]

//=================================================
// Imports
//=================================================

mod hal;
use hal::vga_buffer;
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