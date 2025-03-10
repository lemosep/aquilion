#![no_std]
#![no_main]

//=================================================
// Imports
//=================================================

extern crate multiboot2;
mod macros;
mod hal;
use hal::vga_buffer;
use hal::mbi;
use core::panic::PanicInfo;

//=================================================
// Standalone Functions
//=================================================

#[no_mangle]
pub extern "C" fn kmain(mbi_ptr: usize) {
    
    vga_buffer::clear_screen();
    println!("Hello, Aquilion!");

    mbi::get_mbi_tags(mbi_ptr);

    loop{}
}

#[panic_handler] 
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        
    }
}
