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

pub fn runner(tests: &[&dyn Testable]) {
  serial_println!("Running {} tests", tests.len());
  for test in tests {
    test.run();
  }

  qemu::exit(qemu::ExitCode::Success)
}
