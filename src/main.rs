#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

mod memory;
#[cfg(test)]
mod qemu;
mod serial;
#[cfg(test)]
mod test;
mod vga;

use core::panic::PanicInfo;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);

  loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World!");

  loop {}
}
