use crate::*;

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

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  serial_println!("[failed]\n");
  serial_println!("Error: {}\n", info);

  qemu::exit(qemu::ExitCode::Failed);
  loop {}
}

pub fn main() -> ! {
  test_main();
  loop {}
}

pub fn runner(tests: &[&dyn Testable]) {
  serial_println!("Running {} tests", tests.len());
  for test in tests {
    test.run();
  }

  qemu::exit(qemu::ExitCode::Success)
}
