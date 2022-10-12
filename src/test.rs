use crate::*;

/// Panic handler in test mode.
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
  test_main();
  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  serial_println!("[failed]\n");
  serial_println!("Error: {}\n", info);

  qemu::exit(qemu::ExitCode::Failed);
  loop {}
}

pub trait Testable {
  fn run(&self) -> ();
}

impl<T> Testable for T
where
  T: Fn()
{
  fn run(&self) {
    serial_print!("{}...\t", core::any::type_name::<T>());
    self();
    serial_println!("[ok]");
  }
}

pub fn runner(tests: &[&dyn Testable]) {
  serial_println!("Running {} tests", tests.len());
  for test in tests {
    test.run();
  }

  qemu::exit(qemu::ExitCode::Success)
}
