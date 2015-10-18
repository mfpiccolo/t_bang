t_bang
============

## Description

This crate gives you some handy macros to return or print out the type of a variable,
parameter or literal.

## Examples

Declare the dependency for cargo

```toml
[dependencies]
t_bang = "0.1.2"
```

Then you can import and use the macros
```rust
#[macro_use] extern crate t_bang;
use t_bang::*;

fn main() {
  let x = 5;
  let x_type = t!(x);
  println!("{:?}", x_type);  // prints out: "i32"
  pt!(x);                    // prints out: "i32"
  pt!(5);                    // prints out: "i32"
}
```

t! will return the type of the resource as &str
pt! will prints out the type of the resource
