[![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]

# portion-rs

A simple interval library inspired by Python's `portion`.

## Sample Usage
```rust
use portion_rs::Portion;

fn main() {
    let a = Portion::closed(2, 3);
    println!("{}", a);
}
```

[ci]: https://github.com/Elinvynia/portion-rs/actions?query=workflow%3ARust
[ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/portion-rs/Rust/master?style=flat-square
[docs]: https://docs.rs/portion-rs
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
[crate-link]: https://crates.io/crates/portion-rs
[crate-version]: https://img.shields.io/crates/v/portion-rs.svg?style=flat-square
