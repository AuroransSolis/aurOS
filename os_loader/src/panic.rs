use core::panic::PanicInfo;

// Basic panic handler for now
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // See notes for `_start`
    loop {}
}
