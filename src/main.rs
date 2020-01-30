#![no_std]
#![no_main]
#![feature(const_raw_ptr_deref, const_mut_refs)]

mod panic;
mod vga;

use vga::{vga_buffer::VGA_WRITER, vga_char::VgaColour};

static HELLO: &[u8] = b"Hello World!";

// Since I'm in no_std, I don't have access to the Rust runtime or crt0, so I need my own starting
// runtime. This replaces `main`. `no_mangle` is used for linking. Also, the function returns `!`
// since this is called only by the bootloader/OS.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    VGA_WRITER.lock().write_string("Hello");
    VGA_WRITER
        .lock()
        .set_fg_bg(VgaColour::White, VgaColour::Red);
    VGA_WRITER.lock().write_string(" world!");
    // Returns `!` so why not lmao
    loop {}
}
