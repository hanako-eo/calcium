#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(calcium::test::runner)]
#![reexport_test_harness_main = "test_main"]

mod gdt;
mod interrupts;
mod memory;
mod serial;
mod vga;

use core::panic::PanicInfo;

fn init() {
  gdt::init();
  interrupts::init_idt();
  unsafe {
    interrupts::PICS.lock().initialize();
  };
  x86_64::instructions::interrupts::enable();
}

fn main() {
  println!("Hello World!");
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);

  calcium::hlt_loop()
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
  init();

  if cfg!(not(test)) {
    main();
  } else {
    #[cfg(test)]
    test_main();
  }

  calcium::hlt_loop()
}
