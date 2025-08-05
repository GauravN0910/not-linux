#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(not_linux::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use alloc::boxed::Box;
use alloc::vec::Vec;
use not_linux::allocator::HEAP_SIZE;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use not_linux::allocator;
    use not_linux::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    not_linux::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        memory::init(phys_mem_offset)
    };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    test_main();
    loop {}
}

#[test_case]
fn simple_allocation() {
    let value1 = Box::new(22);
    let value2 = Box::new(7);
    assert_eq!(*value1, 22);
    assert_eq!(*value2, 7);
}

#[test_case]
fn large_vector() {
    let n = 1000;
    let mut nums = Vec::new();
    for idx in 0..n {
        nums.push(idx);
    }   
    assert_eq!(nums.iter().sum::<u64>(), (n*(n-1))/2);
}

#[test_case]
fn many_boxes() {
    for idx in 0..HEAP_SIZE {
        let x = Box::new(idx);
        assert_eq!(*x, idx);
    }
}

#[test_case]
fn many_boxes_long_lived() {
    let long_lived = Box::new(1);
    for idx in 0..HEAP_SIZE {
        let x = Box::new(idx);
        assert_eq!(*x, idx);
    }
    assert_eq!(*long_lived, 1);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    not_linux::test_panic_handler(info);
}
