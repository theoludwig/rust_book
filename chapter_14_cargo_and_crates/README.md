# 14. More About Cargo and Crates.io

## Customizing Builds with Release Profiles

=> Release profiles are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code.

Cargo has 2 main profiles:

- `dev`: for development (used by default with `cargo build`)
- `release`: for release (used with `cargo build --release`)

We can customize the `release` profile by adding a section to the `Cargo.toml` file:

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## Publishing a Crate to Crates.io

Packages can be published to [crates.io](https://crates.io/), the Rust community’s package registry.
