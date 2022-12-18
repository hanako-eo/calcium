#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(calcium::test::runner)]

use bootloader::{entry_point, BootInfo};
use calcium::println;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);

  calcium::hlt_loop()
}

entry_point!(kernel_main);

fn kernel_main(_bootinfo: &'static BootInfo) -> ! {
  calcium::init();
  println!("Hello World!");

  #[cfg(test)]
  test_main();

  calcium::hlt_loop()
}
