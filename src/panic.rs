use core::panic::PanicInfo;

// Basic panic handler for now
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // See notes for `_start`
    loop {}
}