#![no_std]
#![no_main]
// Required since the test derive macro is only part of std, so we need to make our own testing
// framework.
#![feature(custom_test_frameworks)]
// Specify the path to our test runner.
#![test_runner(crate::test_runner::test_runner)]
// Testing generates a `main`, which is ignored because of the `!#[no_main]` above. To avoid this,
// we can change the name of the function that gets created in a testing context.
#![reexport_test_harness_main = "test_main"]

#[macro_use]
mod vga;
mod panic;
mod test_runner;

use vga::{
    vga_buffer::{global_vga_bg, global_vga_fgbg},
    vga_char::VgaColour,
};

// Since I'm in no_std, I don't have access to the Rust runtime or crt0 (C runtime), so I need my
// own starting runtime. This replaces `main`. `no_mangle` is used for linking. Also, the function
// returns `!` since this is called only by the bootloader/OS.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    global_vga_fgbg(VgaColour::Red, VgaColour::White);
    print!("Hello ");
    global_vga_fgbg(VgaColour::White, VgaColour::Red);
    println!("world!");
    global_vga_bg(VgaColour::Black);
    println!("Testing, testing, 123.");
    // Run `test_main()` in the case that we're testing things.
    #[cfg(test)]
    test_main();
    // Check panic handler, also returns `!` so I can lose the `loop` after it
    panic!("uh oh");
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
