#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

pub mod gdt;
pub mod memory;
pub mod qemu;
pub mod serial;
pub mod test;
pub mod vga;

use core::panic::PanicInfo;
use x86_64::instructions;

pub fn test_panic_handler(info: &PanicInfo) -> ! {
  serial_println!("[failed]\n");
  serial_println!("Error: {}\n", info);

  qemu::exit(qemu::ExitCode::Failed);

  hlt_loop()
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
  test_main();

  hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  test_panic_handler(info)
}

pub fn hlt_loop() -> ! {
  loop {
    instructions::hlt();
  }
}
