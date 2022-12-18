#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod qemu;
pub mod serial;
pub mod test;
pub mod vga;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::instructions;

pub fn test_panic_handler(info: &PanicInfo) -> ! {
  serial_println!("[failed]\n");
  serial_println!("Error: {}\n", info);

  qemu::exit(qemu::ExitCode::Failed);

  hlt_loop()
}

pub fn init() {
  gdt::init();
  interrupts::init_idt();
  unsafe {
    interrupts::PICS.lock().initialize();
  };
  instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
  loop {
    instructions::hlt();
  }
}

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(_bootinfo: &'static BootInfo) -> ! {
  init();
  test_main();

  hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  test_panic_handler(info)
}
