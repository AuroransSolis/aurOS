#![no_std]
#![no_main]
#![feature(abi_efiapi)]

mod panic;
mod test_runner;

use uefi::data_types::Handle;
use uefi::Status;
use uefi::table::{Boot, SystemTable};

#[entry]
fn efi_main(image: Handle, st: SystemTable<Boot>) -> Status {
    #[cfg(test)]
    test_main();
    
}
