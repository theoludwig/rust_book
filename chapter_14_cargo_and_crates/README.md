# 14. More About Cargo and Crates.io

## Customizing Builds with Release Profiles

=> Release profiles are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code.

Cargo has 2 main profiles:

- `dev`: for development, to compile faster (used by default with `cargo build`)
- `release`: for release (used with `cargo build --release`)

We can customize the `release` profile by adding a section to the `Cargo.toml` file:

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## Making Useful Documentation Comments

**Documentation comments** use three slashes, **`///`**, instead of two, `//`, and **support Markdown** notation for formatting the text. They needs to be placed before the item they are documenting.

There are 4 sections that can be used in documentation comments:

- `# Examples`: to show examples of how to use the function (`cargo test` will run the examples).
- `# Panics`: to describe under what conditions the function will panic.
- `# Errors`: if the function returns a `Result`, then this section is used to describe the kinds of errors that might occur and in what situations.
- `# Safety`: if the function is `unsafe` to call, then this section is used to explain why it is unsafe and what invariants the function expects callers to uphold.

We don't necessarily need all of these sections, but this is a checklist to remind you of the aspects of your code users will be interested in knowing about.

For example, documenting a `add_one` function in a crate named `my_crate`:

```rs
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

We can generate the documentation with `cargo doc` and open the generated documentation with `cargo doc --open`.

### Commenting Contained Items

The style of doc comment `//!` adds documentation to the item that contains the comments rather than to the items following the comments (typically used at the beginning of a crate or module).

For example, to add documentation that describes the purpose of the `my_crate` crate that contains the `add_one` function:

```rs
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
```

## Publishing a Crate to Crates.io

Packages can be published to [crates.io](https://crates.io/), the Rust community’s package registry.

To publish a crate, we need an account on [crates.io](https://crates.io/) and get an API token, then with the token, we can login:

```sh
cargo login <api-token>
```

### Package Metadata

Before publishing, we need to add some metadata in the `[package]` section of the crate’s `Cargo.toml` file.

For example, the `Cargo.toml` file for the `guessing_game` crate:

```toml
[package]
name = "guessing_game"
version = "1.0.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT"

[dependencies]
```

### Publishing Process

```sh
$ cargo publish
    Updating crates.io index
   Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
   Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
   Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
   Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
```

## Cargo Workspaces

A **workspace** is a set of packages that share the same `Cargo.lock` and output directory. This means that all packages in the workspace will share the same dependencies and build output.

An example of a workspace with multiple packages: [github.com/theoludwig/advent_of_code_2023](https://github.com/theoludwig/advent_of_code_2023).
