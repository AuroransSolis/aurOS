#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_efiapi)]

mod panic;
mod test_runner;
mod types;

#[no_mangle]
pub extern "efiapi" fn efi_main(/* stuff */) /*->  stuff */ {
    #[cfg(test)]
    test_main();
    // more stuff
}
