//=================================================
// Imports
//=================================================

use crate::{
    print,
    println
};
use multiboot2::{BootInformation, MemoryMapTag};

//=================================================
// Standalone Functions
//=================================================

pub fn print_mem_area(memory_map_tag: &MemoryMapTag) {
    // let memory_map_tag = boot_info.memory_map_tag()
    //     .expect("Memory map tag required");

    println!("Memory Areas");
    for section in memory_map_tag.memory_areas() {
        println!("  start: 0x{:x}, length: 0x{:x}",
            section.start_address(), section.size());
    }
}

pub fn handle_elf_sections(boot_info: &BootInformation) -> (u64, u64) {
    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("ELF section tag required");

    println!("Kernel Sections");
    for section in elf_sections_tag.sections() {
        println!("  addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.start_address(), section.size(), section.flags());
    };

    let kstart = elf_sections_tag.sections()
        .map(|s| s.start_address())
        .min()
        .unwrap();

    let kend = elf_sections_tag.sections()
        .map(|s| s.end_address())
        .max()
        .unwrap();

    (kstart, kend)
}
