pub mod hash_maps;
pub mod strings;
pub mod vectors;

fn main() {
    /* Common Collections */
    // Rust's standard library includes a number of very useful data structures called collections.
    // Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.

    // We will learn about 3 collections in this chapter: `hash_maps.rs`, `strings.rs`, `vectors.rs`.
    hash_maps::main_hash_maps();
    strings::main_strings();
    vectors::main_vectors();
}
