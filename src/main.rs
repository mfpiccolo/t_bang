#![feature(core_intrinsics)]

#[macro_export]
macro_rules! t {
  ($from:expr) => (print_type_of($from));
}

fn print_type_of<'a, T>(_: T) -> &'a str {
  let type_name = unsafe { std::intrinsics::type_name::<T>() } as &str;
  type_name
}


fn main() {
  let t = t!(vec![1,2,3]);
  println!("{:?}", t);
}
