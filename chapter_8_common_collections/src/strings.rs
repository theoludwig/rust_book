use std::ops::Add;

// Disable warnings for learning purposes and avoid useless `println!` usage only to use a variable
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_mut)]

pub fn main_strings() {
    /* Strings (`String`) */
    // Collection of bytes.
    // Wrapper around `Vec<u8>` (vector of `u8` values).
    // UTF-8 encoded.
    // String Literal (`&str`) => Size is known at compile time, immutable, hardcoded into our programs.
    // String Type (`String`) => Size unknown at compile time (e.g: user input), mutable, stored on the Heap.

    // Creating a New String
    let mut string = String::new(); // empty string
    let mut string = "initial contents".to_string();
    let mut string = String::from("initial contents");

    // Updating a String
    let mut string = String::from("foo");
    string.push_str("bar");
    string.push('a'); // push a single character

    // Concatenation with the `+` Operator
    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");
    let string3 = string1 + &string2; // note `string1` has been moved here and can no longer be used

    // The `+` operator uses the `add` method:
    let string4 = string3.add("concatenated string");

    // `format!` Macro (works like `println!` but returns a `String` instead of printing to the console)
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let result = format!("{tic}-{tac}-{toe}");

    // Indexing into Strings
    // In many other programming languages, accessing individual characters in a string by referencing them by index is a valid and common operation.
    // However, if you try to access parts of a `String` using indexing syntax in Rust, you'll get an error.

    // Rust strings don't support indexing.
    // The reason is that a string is a wrapper over a `Vec<u8>`, that means, we would access a byte, not necessarily a character as UTF-8 is a variable width encoding (from 1 to 4 bytes).
    // Therefore, an index into the string's bytes will not always correlate to a valid Unicode scalar value.

    // Bytes and Scalar Values and Grapheme Clusters
    // 3 ways to look at strings: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).

    // Example: "é" != "é" (it is combined with 'e' U+0065 and U+0301 the accent itself, and is a string, not a character, as it is 2 characters).
    // Looking at "é" (the string "e" followed by the accent)
    let string = String::from("é");

    // - As bytes (`u8`): [101, 204, 129]
    let bytes = string.as_bytes();

    // - As scalar values (`char`): ['e', '́']
    let chars = string.chars();

    // - As grapheme clusters: ['é']
    // Not implemented by the Rust Standard Library (as it is complex) but is implemented with crates on crates.io (e.g: `unicode-segmentation`).

    // Usually we want to look at strings as grapheme clusters, which means we want to take into account the way people perceive the characters.

    // Slicing Strings
    // We can use `[]` with a range to create a string slice containing particular bytes:
    let string = String::from("Hello, world!");
    let hello = &string[0..5]; // Takes the first 5 bytes

    // let h = &string[0..1]; // panic at runtime

    // Methods for Iterating Over Strings
    // => Be explicit about what you want to do with the characters (bytes? chars? grapheme clusters?).

    for character in "hello".chars() {
        println!("{character}"); // => "h" "e" "l" "l" "o"
    }

    for byte in "hello".bytes() {
        println!("{} => {}", byte, byte as char); // "104 => h" "101 => e" "108 => l" "108 => l" "111 => o
    }

    // Useful methods:
    println!("Length in bytes: {}", "hello".len()); // => 5
    println!("Length in chars: {}", "hello".chars().count()); // => 5

    // Length in grapheme clusters: not implemented by the Rust Standard Library.

    println!("Is empty: {}", "".is_empty()); // => true
    println!("Is empty: {}", "hello".is_empty()); // => false

    println!("Contains \"ello\": {}", "hello".contains("ello")); // => true
    println!("Contains \"abc\": {}", "hello".contains("abc")); // => false

    println!("Starts with \"hello\": {}", "hello".starts_with("hello")); // => true
    println!("Starts with \"abc\": {}", "hello".starts_with("abc")); // => false

    // We can also add methods to strings (thanks to `trait`, ref: Chapter 10)
    // Example: Capitalize a string ("capitalize a string" => "Capitalize a string")
    fn capitalize(string: &str) -> String {
        let mut characters = string.chars();
        match characters.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + characters.as_str(),
        }
    }

    trait Capitalize {
        fn capitalize(&self) -> String;
    }

    impl Capitalize for str {
        fn capitalize(&self) -> String {
            capitalize(self)
        }
    }

    let string = String::from("capitalize 😎");
    println!("capitalize: {}", capitalize(&string));
    println!("capitalize: {}", string.capitalize());
    println!("capitalize: {}", string);
}
