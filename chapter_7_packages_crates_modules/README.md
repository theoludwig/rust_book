# 7. Managing Growing Projects with Packages, Crates, and Modules

=> Organizing the code.
=> Packages can contain multiple binary crates and optionally one library crate.
=> Very large projects can use workspaces.

These features, sometimes collectively referred to as the module system, include:

- **Packages:** A Cargo feature that lets you build, test, and share crates.
- **Crates:** A tree of modules that produces a library (`--lib`) or executable (`--bin`).
- **Modules and use:** Let you control the organization, scope, and privacy of paths.
- **Paths:** A way of naming an item, such as a struct, function, or module.

When we execute `cargo new`, Cargo creates a `Cargo.toml` file giving us a package, there's also a `src` directory, with the crate root file, following the convention (by default):

- Binary crates have a `src/main.rs` root file.
- Library crates have a `src/lib.rs` root file.

Crate root files create a module named `crate` at the root of the crate's module structure, known as the module tree.

If we have both files, it means that our package contains both a library and a binary crate.

A package can have multiple binary crates by placing files in the `src/bin` directory (additionally to `src/main.rs`), but only one library crate `src/lib.rs`.

## Modules

In the crate root file, we can declare modules with the `mod` keyword (e.g: `mod garden;`). The compiler will look for the module's code in these places:

- Inline, within curly brackets that replace the semicolon following `mod garden { // code }`.
- In the file `src/garden.rs`.
- In the file `src/garden/mod.rs`.

=> Code within a module is private by default, we must use `pub` for each code we're exporting.

## Submodules

In any file other than the crate root, you can declare submodules. For example, you might declare `mod vegetables;` in `src/garden.rs`. The compiler will look for the submodule's code within the directory named for the parent module in these places:

- Inline, within curly brackets that replace the semicolon following `mod vegetables { // code }`.
- In the file `src/garden/vegetables.rs`.
- In the file `src/garden/vegetables/mod.rs`.

## Paths to code in modules

Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.

Example:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

=> Our preference in general is to **specify absolute paths** because it's more likely we'll want to move code definitions and item calls independently of each other.

## Private vs Public

Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.

## The `use` keyword

Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and from then on you only need to write `Asparagus` to make use of that type in the scope.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Conventions:

- For functions, we use until the last part of the path (without the the function name), so he have still to call the module name (e.g: `hosting::add_to_waitlist();` and not `add_to_waitlist();` directly).
- For structs, enums, and other items with `use`, we use the full path (except, if we're bringing 2 items with the same name):

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

We can also reexport items with `pub use`:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

We can also use nested path if we're using multiple items defined in the module:

```rust
// Without nesting
use std::cmp::Ordering;
use std::io;

// With nesting
use std::{cmp::Ordering, io};
```

Another example of nesting:

```rust
// Without nesting
use std::io;
use std::io::Write;

// With nesting
use std::io::{self, Write};
```

## Declaration of modules and submodules in the same file

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
