#![no_std]
#![no_main]

mod panic;

static HELLO: &[u8] = b"Hello World!";

// Since I'm in no_std, I don't have access to the Rust runtime or crt0, so I need my own starting
// runtime. This replaces `main`. `no_mangle` is used for linking. Also, the function returns `!`
// since this is called only by the bootloader/OS.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    // Returns `!` so why not lmao
    loop {}
}
