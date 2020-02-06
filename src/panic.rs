use core::panic::PanicInfo;

// Basic panic handler for now
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    // See notes for `_start`
    loop {}
}
