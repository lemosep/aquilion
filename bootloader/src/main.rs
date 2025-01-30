#![no_std]
#![no_main]

use core::ptr::NonNull;
use log::info;
use uefi::{
    boot::{image_handle, AllocateType, MemoryType},
    entry,
    prelude::*,
    proto::media::file::{Directory, File, FileAttribute, FileMode, RegularFile}
};

#[entry]
fn uefi_main() -> Status {
    uefi::helpers::init().unwrap();

    info!("Starting...");

//****** Attempt to load the kernel ******//
    match load_kernel() {
        Ok(kaddress) => {
            info!("Successfully loaded kernel. Jumping...");

            let kernel_entry: extern "C" fn () -> ! = unsafe {
                core::mem::transmute(kaddress)
            };
            kernel_entry();
        },
        Err(_) => {
            info!("Failed to load kernel");
        },
    }
    Status::SUCCESS
}

fn load_kernel() -> Result<*mut u8, uefi::Error> {
    let image_handle: Handle = image_handle();
    let mut root: Directory = boot::get_image_file_system(image_handle)?.open_volume()?;
    let mut kfile: RegularFile = root
        .open(
            cstr16!("kernel"),
            FileMode::Read,
            FileAttribute::READ_ONLY,
        )?
        .into_regular_file()
        .expect("kernel not found");

    // ****** Allocate memory for the buffer ****** //
    let buffer_size: usize = 1024 * 1024; // 1 MB
    let mut buf: NonNull<u8> = boot::allocate_pages(
        AllocateType::AnyPages,
        MemoryType::LOADER_DATA,
        (buffer_size + 0xFFF) / 0x1000, // Convert size to pages
    )?;
    
    let buf_ptr: *mut u8 = unsafe { buf.as_ptr() };

    // ****** Read into dynamically allocated buffer ****** //
    match kfile.read(unsafe { core::slice::from_raw_parts_mut(buf_ptr, buffer_size) }) {
        Ok(_) => (),
        Err(_) => {
            unsafe { boot::free_pages(buf, (buffer_size + 0xFFF) / 0x1000)?; }
            return Err(uefi::Status::LOAD_ERROR.into());
        }
    }

    // ****** Allocate memory for the kernel ****** //
    let mut kernel_mem: NonNull<u8> = boot::allocate_pages(
        AllocateType::AnyPages,
        MemoryType::LOADER_DATA,
        (buffer_size + 0xFFF) / 0x1000, // Convert size to pages
    )?;

    unsafe {
        core::ptr::copy_nonoverlapping(buf_ptr, kernel_mem.as_mut(), buffer_size);
        boot::free_pages(buf, (buffer_size + 0xFFF) / 0x1000)?; // Free temporary buffer
    }

    Ok(unsafe { kernel_mem.as_mut() })
}

