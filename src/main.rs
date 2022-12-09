#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

mod gdt;
mod interrupts;
mod memory;
#[cfg(test)]
mod qemu;
mod serial;
#[cfg(test)]
mod test;
mod vga;

use core::panic::PanicInfo;

fn init() {
  gdt::init();
  interrupts::init_idt();
}

fn main() -> ! {
  println!("Hello World!");

  loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);

  loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
  init();
  #[cfg(not(test))]
  main();
  #[cfg(test)]
  test::main();
}
