#![no_std]
#![no_main]

//=================================================
// Imports
//=================================================

extern crate multiboot2;
mod macros;
mod hal;
use hal::mem::FrameAllocator;
use hal::vga_buffer;
use hal::mbi;
use multiboot2::BootInformationHeader;
use core::panic::PanicInfo;

//=================================================
// Standalone Functions
//=================================================

#[no_mangle]
pub extern "C" fn kmain(mbi_ptr: usize) {
    use hal::mem::frame_allocator; 

    vga_buffer::clear_screen();
    println!("Hello, Aquilion!");

    let boot_info = unsafe {
        multiboot2::BootInformation::load(mbi_ptr as *const BootInformationHeader)
            .expect("failed to load multiboot information header")
    }; 

    let memory_map_tag = boot_info.memory_map_tag()
    .expect("Memory map tag required");

    mbi::print_mem_area(&memory_map_tag);
    
    let kernel_metrics= mbi::handle_elf_sections(&boot_info);

    let mb_start = mbi_ptr;
    let mb_end = mb_start + (boot_info.total_size() as usize);

    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    let mut frame_allocator = frame_allocator::AreaFrameAllocator::new(
        kernel_metrics.0 as usize,
        kernel_metrics.1 as usize,
        mb_start,
        mb_end,
        memory_map_tag.memory_areas()
    );
    
    for i in 0.. {
        if let None = frame_allocator.allocate_frame() {
            println!("allocated {} frames", i);
            break;
        }
    }

    loop{}
}

#[panic_handler] 
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        
    }
}
