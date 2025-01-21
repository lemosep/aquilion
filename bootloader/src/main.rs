#![no_std]
#![no_main]

use core::fmt::Write;
use uefi::prelude::*;
use uefi::entry;

#[entry]
fn uefi_main() -> Status {
    uefi::helpers::init().unwrap();
     
    uefi::system::with_stdout(|stdout| {
        stdout.write_str("Hello, uefi").unwrap();
    } );
    boot::stall(10_000_000);
    Status::SUCCESS
}
