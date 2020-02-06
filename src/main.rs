#![no_std]
#![no_main]

#[macro_use]
mod vga;
mod panic;

use vga::{
    vga_buffer::{global_vga_bg, global_vga_fgbg, VGA_WRITER},
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
    // Check panic handler
    panic!("uh oh");
    // Returns `!` so why not lmao
    loop {}
}
