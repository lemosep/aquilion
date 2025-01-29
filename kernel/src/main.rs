#![no_std]
#![no_main]

use core::panic::PanicInfo;
static GREET: &[u8] = b"Hello, Aquilion!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] 
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb800 as *mut u8;

    for(i, &byte) in GREET.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}