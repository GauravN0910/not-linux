#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Welcome to Not-Linux\n").unwrap();
    write!(vga_buffer::WRITER.lock(), "Printed using Rust FMT\n").unwrap();
    println!("Printed using println macro");
    loop {}
}
