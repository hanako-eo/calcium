#![no_std]
#![no_main]

use calcium::{qemu, serial_print, serial_println};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  serial_println!("[ok]");

  qemu::exit(qemu::ExitCode::Success);

  calcium::hlt_loop()
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
  should_fail();
  serial_println!("[test did not panic]");

  qemu::exit(qemu::ExitCode::Failed);

  calcium::hlt_loop()
}

fn should_fail() {
  serial_print!("calcium::should_panic::should_fail...\t");
  assert_eq!(0, 1);
}
