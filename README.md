# Rust Calculator

A minimal and fast math calculator implemented in Rust (It is a learning project).
It uses an `enum` to define basic operations (`Add`, `Subtract`, `Multiply`, `Divide`) and a single `calculate()` function to evaluate a slice of `f64` values.

## Features

* Basic operations: addition, subtraction, multiplication, division
* Optimized for performance (`opt-level = 3`, `lto = "thin"`, `codegen-units = 1`, `strip = true`)

## Build and Run

```bash
cargo build --release
cargo run --release
```

## Example Output

```
Values: [2.2, 3.0, 4.4, 5.0]
Add: 14.60
Subtract: -10.20
Multiply: 145.20
Divide: 0.03
```

## Roadmap

* [ ] Modularize operations into `math::{add, sub, mul, div}`
* [ ] Add unit tests
* [ ] Integrate with a GUI (e.g. `egui`, `slint`, or `Flutter + FFI`)
