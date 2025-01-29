#![no_std]
#![no_main]

use core::{fmt::Write, ptr::NonNull};
use uefi::{
    boot::{image_handle, AllocateType, MemoryType},
    entry,
    prelude::*,
    proto::media::file::{Directory, File, FileAttribute, FileMode, RegularFile}
};

#[entry]
fn uefi_main() -> Status {
    uefi::helpers::init().unwrap();

    uefi::system::with_stdout(|stdout| {
        stdout.write_str("Hello, uefi").unwrap();
    } );

//****** Attempt to load the kernel ******//
    match load_kernel() {
        Ok(kaddress) => {
            uefi::system::with_stdout(|stdout| {
                stdout.write_str("Successfully loaded kernel. Jumping...").unwrap();                
            });

            let kernel_entry: extern "C" fn () -> ! = unsafe {
                core::mem::transmute(kaddress)
            };
            kernel_entry();
        },
        Err(_) => {
            uefi::system::with_stdout(|stdout| {
                stdout.write_str("Failed to load kernel").unwrap();                
            })
        },
    }
    Status::SUCCESS
}

/// # Description
/// Loads the kernel located in the ESP.
/// 
/// By default, the UEFI file system sees the esp directory
/// as the root of the EFI system partition.
fn load_kernel() -> Result<*mut u8, uefi::Error>  {
    let image_handle: Handle = image_handle();
    let mut root: Directory = boot::get_image_file_system(image_handle)?.open_volume()?;
    let mut kfile: RegularFile = root.open
    (
        cstr16!("kernel"),
        FileMode::Read,
        FileAttribute::READ_ONLY
    )?.into_regular_file()
    .expect("kernel.bin not found");

//****** Read into buffer ******//
    let mut buf = [0u8; 1024 * 1024]; // 1 MB static buffer
    match kfile.read(&mut buf) {
        Ok(_) => (),
        Err(_) => return Err(uefi::Status::LOAD_ERROR.into()),
    }

//****** Allocate memory for the kernel ******//
let mut kernel_mem: NonNull<u8> = boot::allocate_pages(
    AllocateType::AnyPages,
    MemoryType::LOADER_DATA,
    (buf.len() + 0xFFF) / 0x1000,   // Convert size to pages (4 KB each)
)?;
    
unsafe {
    core::ptr::copy_nonoverlapping(buf.as_ptr(), kernel_mem.as_mut(), buf.len());
}

Ok(unsafe { kernel_mem.as_mut() })
}
