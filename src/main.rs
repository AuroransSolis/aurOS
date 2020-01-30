#![no_std]
#![no_main]

mod panic;
mod vga;

static HELLO: &[u8] = b"Hello World!";

// Since I'm in no_std, I don't have access to the Rust runtime or crt0, so I need my own starting
// runtime. This replaces `main`. `no_mangle` is used for linking. Also, the function returns `!`
// since this is called only by the bootloader/OS.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::vga_buffer::vga_test();
    // Returns `!` so why not lmao
    loop {}
}
