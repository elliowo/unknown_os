#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(unknown_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use unknown_os::{println, vga_buffer::initialise_buffer};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    initialise_buffer();
    println!("unknown_os");

    unknown_os::init();

    #[cfg(test)]
    test_main();

    println!("i did not crash :)");
    unknown_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    unknown_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unknown_os::test_panic_handler(_info);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
