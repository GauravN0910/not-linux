#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(not_linux::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use not_linux::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    not_linux::test_panic_handler(info)
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    not_linux::vga_buffer::WRITER.lock().write_str("Welcome to Not-Linux\n").unwrap();
    write!(not_linux::vga_buffer::WRITER.lock(), "Printed using Rust FMT\n").unwrap();
    println!("Printed using println macro");

    #[cfg(test)]
    test_main();
    loop {}
}


