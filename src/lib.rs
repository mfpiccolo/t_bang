#![feature(core_intrinsics)]

#[macro_export]
macro_rules! t {
  ($from:expr) => (unsafe { std::intrinsics::type_name::<T>() } as &str);
}

fn print_type_of<'a, T>(_: T) -> &'a str {
  unsafe { std::intrinsics::type_name::<T>() } as &str;
}

pub extern fn something() {

}
