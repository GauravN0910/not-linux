#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[test_case]
fn sample_test() {
    print!("Sample Test = ");
    assert_eq!(1, 1);
    println!("Mudinch");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Welcome to Not-Linux\n").unwrap();
    write!(vga_buffer::WRITER.lock(), "Printed using Rust FMT\n").unwrap();
    println!("Printed using println macro");

    #[cfg(test)]
    test_main();
    loop {}
}


