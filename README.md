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
