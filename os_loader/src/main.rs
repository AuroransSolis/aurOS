#![no_std]
#![no_main]
#![feature(abi_efiapi)]

mod panic;
mod test_runner;

use uefi::data_types::Handle;
use uefi::table::{Boot, SystemTable};
use uefi::Status;

#[entry]
fn efi_main(image: Handle, st: SystemTable<Boot>) -> Status {
    #[cfg(test)]
    test_main();
    // to do:
    // set up paging
    // boot video mode?
}
