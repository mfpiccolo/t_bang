#![feature(core_intrinsics)]

#[macro_export]
macro_rules! t {
  ($from:expr) => (type_of($from));
}

#[macro_export]
macro_rules! pt {
  ($from:expr) => (println!(type_of($from)));
}

pub extern fn type_of<'a, T>(_: T) -> &'a str {
  unsafe { std::intrinsics::type_name::<T>() }
}
