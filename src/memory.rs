pub struct Volatile<T>(T);

impl<T> Volatile<T> {
  pub fn read(&self) -> T {
    unsafe { core::ptr::read_volatile(&self.0) }
  }

  pub fn write(&mut self, value: T) {
    // UNSAFE: Safe, as we know that our internal value exists.
    unsafe { core::ptr::write_volatile(&mut self.0, value) }
  }
}
