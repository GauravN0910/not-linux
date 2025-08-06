#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(not_linux::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use not_linux::println;
use bootloader::{BootInfo, entry_point};
extern crate alloc;
use alloc::{boxed::Box, vec::Vec};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    not_linux::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    not_linux::test_panic_handler(info)
}


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use not_linux::allocator;
    use not_linux::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;
    println!("Welcome to Not-Linux");

    not_linux::init();
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        memory::init(phys_mem_offset)  
    };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap allocation failed");
    let x = Box::new(41);
    println!("value x is at {:p}", x);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vector is at {:p}", vec.as_slice());

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    not_linux::hlt_loop();
}


