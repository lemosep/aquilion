//=================================================
// Imports
//=================================================

use crate::{print, println};
use multiboot2::{BootInformation, BootInformationHeader};
use spin::Mutex;

//=================================================
// Static 
//=================================================

static KERNEL_SIZE: Mutex<Option<usize>> = Mutex::new(None);

//=================================================
// Standalone Functions
//=================================================

/// # Description
/// The Multiboot Information Structure (MBI) contains
/// a series of useful tags to fetch memory mapping,
/// ELF sections, Framebuffer info, etc...
pub fn get_mbi_tags(mbi_ptr: usize) {
    
    let boot_info: BootInformation<'_> = unsafe {
        multiboot2::BootInformation::load(mbi_ptr as *const BootInformationHeader)
            .expect("failed to load multiboot information header")
    };

    let memory_map_tag: &multiboot2::MemoryMapTag = boot_info.memory_map_tag()
        .expect("Memory map tag required");
    
    let elf_sections_tag: &multiboot2::ElfSectionsTag = boot_info.elf_sections_tag()
        .expect("ELF section tag required");
    
    let size = compute_kernel_size(elf_sections_tag);
    *KERNEL_SIZE.lock() = Some(size);


    print_memory_map_tag(memory_map_tag);
    print_kernel_elf_tag(elf_sections_tag);
}

/// # Description
/// Returns the kernel size in bytes.
pub fn get_kernel_size() -> usize {
    KERNEL_SIZE.lock().unwrap_or(0)
}

fn compute_kernel_size(elf_sections_tag: &multiboot2::ElfSectionsTag) -> usize {
    let kstart = elf_sections_tag.sections().map(|s| s.start_address())
        .min().unwrap();

    let kend = elf_sections_tag.sections().map(|s| s.end_address())
        .max().unwrap();

    (kend - kstart) as usize
}

fn print_memory_map_tag(memory_map_tag: &multiboot2::MemoryMapTag) {
    println!("Memory Areas:");
    for area in memory_map_tag.memory_areas() {
        println!("  start: 0x{:x}, length: 0x{:x}",
        area.start_address(), area.size());
    }
}

fn print_kernel_elf_tag(elf_sections_tag: &multiboot2::ElfSectionsTag) {
    println!("Kernel Sections");
    for section in elf_sections_tag.sections() {
        println!("  addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.start_address(), section.size(), section.flags());
    }
}
