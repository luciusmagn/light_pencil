# light_pencil (A Sharp Pencil fork (A Pencil fork))

[![Build Status](https://travis-ci.org/luciusmagn/light_pencil.svg?branch=master)](https://travis-ci.org/luciusmagn/light_pencil) [![Crates.io Version](https://img.shields.io/crates/v/pencil.svg)](https://crates.io/crates/pencil/) [![Crates.io LICENSE](https://img.shields.io/crates/l/light_pencil.svg)](https://crates.io/crates/light_pencil/)

A microframework for Rust inspired by Flask.

```rust
extern crate light_pencil;
use light_pencil::{Pencil, Request, Response, PencilResult};
fn hello(_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello World!"))
}
fn main() {
    let mut app = Pencil::new("/web/hello");
    app.get("/", "hello", hello);
    app.run("127.0.0.1:5000");
}
```

One simple guide: https://fengsp.github.io/blog/2016/3/introducing-pencil/
Documentation: https://fengsp.github.io/pencil/pencil/
