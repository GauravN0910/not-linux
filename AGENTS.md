# AGENTS.md

Not-Linux is a small x86_64 hobby kernel written in Rust (`no_std`, `no_main`). It boots through `bootloader` 0.9 and runs under QEMU.

## Toolchain and setup

- `rust-toolchain.toml` pins the **nightly** channel, with the `rust-src` and `llvm-tools-preview` components. Plain `cargo` commands use it automatically, so don't add `+nightly`. The project won't build on stable.
- `.cargo/config.toml` sets the custom target `x86_64-not-linux.json` and the `[unstable]` options `build-std` (core, compiler_builtins, alloc) and `json-target-spec`.
- Required host tools: `cargo install bootimage` and QEMU (`brew install qemu` on macOS).
- Nightly Rust changes often break this project. Known breakages so far:
  - The target spec JSON format changed: widths must be integers, and the ABI value is `"rustc-abi": "softfloat"`.
  - The `x86_64` crate's `step_trait` feature no longer compiles. `Cargo.toml` keeps it disabled.
  - Older `bootloader` 0.9.x releases ship an outdated target spec. Use 0.9.35 or later.

## Commands

- `cargo build`: builds the kernel ELF.
- `cargo bootimage`: builds a bootable disk image at `target/x86_64-not-linux/debug/bootimage-not-linux.bin`.
- `cargo run`: boots the kernel in a QEMU window. It never exits on its own, because the kernel loops on `hlt`.
- `cargo test`: runs every test binary in QEMU without a window. Test output goes to serial, then to stdout.
- `cargo test --test <name>`: runs one integration test binary, such as `heap_allocation`.

To check the kernel boots without opening a window, run it headless and dump the VGA screen through the QEMU monitor (`-display none -monitor stdio`, then send `screendump out.ppm`).

## Layout

- `src/main.rs`: kernel entry point (`kernel_main`, set with `entry_point!`). It sets up paging and the heap and prints to VGA.
- `src/lib.rs`: shared kernel init (GDT, IDT, PICs, interrupts), the custom test framework (`Testable`, `test_runner`), the QEMU exit device and the panic handlers.
- `src/vga_buffer.rs`: VGA text-mode writer and the `print!`/`println!` macros.
- `src/serial.rs`: UART 16550 serial port and the `serial_print!`/`serial_println!` macros, used for test output.
- `src/gdt.rs`: GDT and TSS, including a separate stack for double faults.
- `src/interrupts.rs`: IDT, CPU exception handlers, the 8259 PIC, and the timer and keyboard interrupt handlers.
- `src/memory.rs`: page table access through physically mapped memory, and the boot-info frame allocator.
- `src/allocator.rs`: maps the heap at `HEAP_START` (`0x4444_4444_0000`, 100 KiB) and sets the global allocator.
- `src/allocator/`: three allocator designs (`bump`, `linked_list`, `fixed_size_block`). `FixedSizeBlockAllocator` is the active one.
- `tests/`: integration tests. Each file is a separate kernel image. `should_panic` and `stack_overflow` set `harness = false` in `Cargo.toml`.

## Testing conventions

- Tests use `#[test_case]` through `custom_test_frameworks`, not the standard `#[test]`.
- A test passes when the kernel writes `0x10` to the `isa-debug-exit` port (`iobase=0xf4`), which QEMU reports as exit code 33. Tests time out after 300 s. These settings live in `[package.metadata.bootimage]` in `Cargo.toml`.
- A new integration test that must not use the test harness needs a `[[test]]` entry with `harness = false`.
- Run `cargo test` after any change to interrupts, memory or the allocators.

## Commit messages

When suggesting a commit message:

- Write exactly **one line**, with no body.
- Do **not** add a `Co-Authored-By` trailer or any other attribution line.
- Match the existing history: a short, capitalized summary in title case with no trailing period, such as `Heap Allocator Designs` or `Handling Double Faults`.
