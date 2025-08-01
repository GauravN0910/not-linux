#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(not_linux::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use not_linux::println;
use bootloader::{BootInfo, entry_point};

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
    use not_linux::memory;
    use not_linux::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::Page, VirtAddr};
    println!("Welcome to Not-Linux");
    not_linux::init();
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        memory::init(phys_mem_offset)  
    };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let page = Page::containing_address(VirtAddr::new(0xDEADBEEF));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe{
        page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)
    };
    // let ptr = 0x2051A1 as *mut u8;
    // unsafe{
    //     let _x = *ptr;
    // }
    // println!("Read Works");
    //
    // unsafe{
    //     *ptr = 96;
    // }
    // println!("Write Works");
    // unsafe {
        // *(0xDEADBEEF as *mut u8) = 42;
    // }

    // x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    not_linux::hlt_loop();
}


