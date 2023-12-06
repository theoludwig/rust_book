# 1. Getting Started

To install Rust, we use `rustup`, a command line tool for managing Rust versions and associated tools.

## Hello, world

Rust use `snake_case` as convention for naming variables, functions, modules, etc.

Filename: `main.rs`

```rust
fn main() {
    println!("Hello, world!");
}
```

Compile and run:

```bash
rustc main.rs
./main
```

## Cargo commands

=> Cargo as Convention.

- `cargo new hello_world` to create a new project called `hello_world`: `--bin` to create a binary project (default), `--lib` to create a library project
- `cargo build` to compile (development)
- `./target/debug/hello_world` to execute (development)
- `cargo run` to compile and execute (development)
- `cargo test` to run tests
- `cargo doc --open` to generate documentation provided by all your dependencies locally and open it in your browser
- `cargo check` checks your code to make sure it compiles but doesn't produce an executable
- `cargo clippy` runs the clippy (official) linter (is a superset of `cargo check`, so it also checks for compilation errors), `cargo clippy --fix` to automatically fix some of the errors/warnings
- `cargo fmt` to format the code
- `cargo build --release` to compile with optimizations (production)
- `./target/release/hello_world` to execute (production)
- `cargo run --release` to compile and run (production)

## Typical CI pipeline for Rust with Cargo

- `cargo build --verbose`
- `cargo test --verbose`
- `cargo clippy --verbose -- -D warnings`
- `cargo fmt -- --check`
