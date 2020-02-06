use core::panic::PanicInfo;

// Basic panic handler for now
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", info);
    // See notes for `_start`
    loop {}
}
