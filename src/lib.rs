#![feature(core_intrinsics)]

#[macro_export]
macro_rules! t {
  ($resource:expr) => (type_of($resource));
  ($resource:ident) => (type_of($resource));
}

#[macro_export]
macro_rules! pt {
  ($resource:expr) => (println!("{:?}", type_of($resource)));
  ($resource:ident) => (println!("{:?}", type_of($resource)));
}

pub extern fn type_of<'a, T>(_: T) -> &'a str {
  unsafe { std::intrinsics::type_name::<T>() }
}
