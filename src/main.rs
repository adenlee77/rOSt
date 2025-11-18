#![no_std] // don't link the rust standard library
#![no_main] // disable all rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function named '_start' by default
    vga_buffer::print_something();

    loop {}
}