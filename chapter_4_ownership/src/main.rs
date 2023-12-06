// Disable warnings for learning purposes and avoid useless `println!` usage only to use a variable
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_mut)]

fn main() {
    /* Ownership */
    // Rust most unique feature that enables memory safety guarantees without needing a garbage collector or without manually allocating and freeing the memory.
    // Ownership is a set of rules (that the compiler checks) that govern how a Rust program manages memory.
    // Main purpose of ownership is to manage Heap data.

    /* The Stack and the Heap */
    // Stack => LIFO (Last In First Out), stores values in the order it gets them and removes the values in the opposite order. Data stored on the stack must have a known, fixed size at compile time. Faster, as it never has to search for a place to put new data or a place to get data from. Variables on the stack are accessed relative to the frame pointer.
    // Heap => Less organized, stores data of a unknown size at runtime or a size that might change. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.

    // Basic data types (Scalar Type) like integers, floats, booleans, and characters are stored on the Stack.
    // Data like strings (Compound Type) that we want to modify or store an unknown size are stored on the Heap.

    // When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function's local variables get pushed onto the stack. When the function is over, those values get popped off the stack (no need to free the memory in the stack, "it's done automatically").

    /* Ownership Rules
        - Each value in Rust has an owner.
        - There can only be one owner at a time.
        - When the owner goes out of scope, the value will be dropped.
    */

    /* Variable Scope */
    // Scope is the range within a program for which an item is valid.
    {
        let string = String::from("Hello, world!");
    }
    // `string` variable is not valid here, it's not in scope (and the memory is automatically returned).

    /* String types */
    // String Literal (`&str`) => Size is known at compile time, immutable, hardcoded into our programs.
    // String Type (`String`) => Size unknown at compile time (e.g: user input), mutable, stored on the Heap.

    // To create a String Type from a String Literal
    let mut string = String::from("Hello");
    // This kind of string can be mutated (given that we used `mut` in the declaration).
    string.push_str(", world!");
    string += "abc";
    string += &String::from("def");

    /* Memory and Allocation */
    /* In order to support a mutable, growable piece of text with `String`, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents.
        1. The memory must be requested from the memory allocator at runtime.
        2. We need a way of returning this memory to the allocator when we're done with our `String` (free it).
    */
    // 1. `String::from` function requests the memory.
    // 2. The memory is returned once the variable that owns it goes out of scope by calling the `drop` function (automatically called by Rust).

    /* Variables and Data Interacting with Move */
    // All values that are stored on the Stack (e.g: `i32`)
    let x = 5;
    let y = x; // make a copy of the value in `x` and bind it to `y`
    println!("x = {x}, y = {y}");

    // All values that are stored on the Heap (e.g: `String`)
    let string1 = String::from("hello");
    let string2 = string1; // `string1` is moved to `string2` (no copy is made)

    // println!("{string1}"); // `string1` is no longer valid (Rust doesn't need to free anything when `string1` goes out of scope, as it doesn't own anything)
    println!("{string2}"); // `string2` is valid

    /* String is made up of a group of data stored on the Stack:
        - Pointer to the memory that holds the contents of the string (stored on the Heap). We store the pointer on the Stack.
        - Length of the string.
        - Capacity of the string.
    */
    // When we assign `let string2 = string1;` the pointer (still points to the same location on the Heap, data is not copied, only the pointer is copied.), length, and capacity are copied.

    // When `string1` and `string2` goes out of scope, Rust calls `drop` and the memory on the Heap is freed.
    // They will both try to free the same memory (double free error) => Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
    // That's why Rust considers `string1` to no longer be valid and Rust won't let us use `string1` after we have moved its value to `string2`.

    // Rust design choice: Rust will never automatically create "deep" copies of your Heap data.

    /* Variables and Data Interacting with Clone */
    // Deep copy of the Heap data (not just the Stack data liked with move).
    let string1 = String::from("hello");
    let string2 = string1.clone(); // Might be expensive (if the data on the Heap is large).
    println!("string1 = {string1}, string2 = {string2}");

    // TL;DR: Stack data is always copied, Heap data is moved (unless we use `clone`).
    /*
        `Copy` annotation trait that we can place on types that are stored on the Stack (except if the type, or any of its parts, has implemented the `Drop` trait).

        If a type implements the `Copy` trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

        `Copy` trait are implemented by any group of simple scalar values, and nothing that requires allocation, examples:
            - All the integer types, such as `u32`.
            - The Boolean type, `bool`.
            - All the floating-point types, such as `f64`.
            - The character type, `char`.
            - Tuples, if they only contain types that also implement Copy. For example, `(i32, i32)` implements `Copy`, but `(i32, String)` does not.
    */

    /* Ownership and Functions */
    // Passing a variable to a function will move or copy, just as assignment does.

    let string = String::from("hello");
    takes_ownership(string); // `string` is moved into the function

    // println!("{string}"); // `string` is no longer valid

    let x = 5;
    makes_copy(x); // `x` is copied into the function

    println!("{x}"); // `x` is still valid

    fn takes_ownership(some_string: String) {
        println!("{some_string}");
    }

    fn makes_copy(some_integer: i32) {
        println!("{some_integer}");
    }

    // Return Values and Scope
    // Returning values can also transfer ownership.
    let string1 = gives_ownership(); // `string1` comes into scope

    let string2 = String::from("hello"); // `string2` comes into scope
    let string3 = takes_and_gives_back(string2); // `string2` is moved into the function (no longer available) and `string3` comes into scope

    fn gives_ownership() -> String {
        let some_string = String::from("hello");
        some_string // `some_string` is returned and moves out to the calling function
    }

    fn takes_and_gives_back(a_string: String) -> String {
        a_string // `a_string` is returned and moves out to the calling function
    }

    // TL;DR: The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.

    // What if we want to let a function use a value but not take ownership?
    // => Rust does let us return multiple values using a tuple.
    fn calculate_length(string: String) -> (String, usize) {
        let length = string.len(); // `len()` returns the length of a `String`
        (string, length)
    }
    let string1 = String::from("hello");
    let (string2, length) = calculate_length(string1);
    println!("The length of '{string2}' is {length}.");
    // println!("string1 = {string1}"); // `string1` is no longer valid

    // Problem: Requires lot of work for a concept that should be common.

    /* References and Borrowing (`&`) */
    /* References => allow you to refer to some value without taking ownership of it:
        - Address we can follow to access the data stored.
        - Data is owned by some other variable.
        - Guaranteed to point to a valid value of a particular type for the life of that reference (unlike pointer).
    */
    // References are immutable by default.
    // References are reprensented by `&` and `&mut` for mutable references.
    // Opposite of referencing (`&`) is dereferencing (`*`).
    fn calculate_length_references(string: &String) -> usize {
        string.len()
    }
    let string1 = String::from("hello");
    let length = calculate_length_references(&string1);
    println!("The length of '{string1}' is {length}.");
    // Creating a reference => Borrowing.
    // We can't modifiy a variable we're borrowing (immutable by default).

    // Mutable References (`&mut`).
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    let mut string = String::from("hello");
    change(&mut string);

    /* Restriction: Cannot borrow a variable as mutable more than once at a time.

    Reasons for this restriction: prevents data races at compile time, similar to a race condition and happens when these three behaviors occur:
        - 2 or more pointers access the same data at the same time.
        - At least one of the pointers is being used to write to the data.
        - There's no mechanism being used to synchronize access to the data.

    => Data races cause undefined behavior.
    */
    let result1 = &mut string;
    let result2 = &mut string; // Error: cannot borrow `string` as mutable a second time

    // println!("{result1}, {result2}");

    // Dangling References
    // A pointer that references a location in memory that may have been given to someone else. Freeing some memory while preserving a pointer to that memory.
    // Rust prevents dangling references at compile time.

    // Compiler error:
    // fn dangling_references() -> &String {
    //     let s = String::from("hello");
    //     &s
    // }

    // `dangling_references` returns a reference to a `String`, but because the `String` is deallocated when the function ends, the reference would be pointing to garbage. Rust won't compile code with dangling references.

    // Solution: Return the value directly => Ownership is moved out, and nothing is deallocated.
    fn no_dangling_references() -> String {
        let s = String::from("hello");
        s
    }

    // This is valid because `some_string` ownership is moved out of the function and returned to the calling function.
    fn returns_reference(some_string: &String) -> &String {
        some_string
    }

    /* Slice Type */
    // Reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

    // Example: A function that takes a string of words separated by spaces and returns the first word as a reference.

    // 1st solution example (without slices):
    // The problem with this solution, is that it returns a new `String` (a copy of the first word) and not a reference to the first word (might be something we want to do, but in the example statement, we wanted a reference).
    fn first_word(string: &String) -> String {
        let mut result = String::new();
        // `.chars()` and `.as_bytes()` might not do what we want, if we compare with characters that their code binary representation takes number of bits >= 8 (in Extended ASCII and UTF-8)
        // As in this case, the UTF-8 binary representation takes at least 2 bytes and necessarily is not the same as in ASCII (only 1 byte)
        // Also, in UTF-8, we can represent the same characters in many ways.
        // Example: "é" != "é" (combined with 'e' U+0065 and U+0301 the accent itself, and is a string, not a character, as it is 2 characters).
        // Iteration over grapheme clusters might be better.
        for (_, character) in string.chars().enumerate() {
            if character == ' ' {
                break;
            } else {
                result.push(character);
            }
        }
        result
    }
    let sentence = String::from("Hello world!");
    let sentence_first_word = first_word(&sentence);
    println!("Sentence: \"{sentence}\" ; First word: \"{sentence_first_word}\"");

    // 2nd example (without slices):
    // This function returns the index of the end of the word indicated by a space.
    fn first_word_2(string: &String) -> usize {
        for (index, character) in string.chars().enumerate() {
            if character == ' ' {
                return index;
            }
        }
        string.len()
    }
    // Problem the value returned is a separate value from the `String`, there's no guarantee that it will still be valid in the future.
    let mut string = String::from("hello world");
    let word = first_word(&string); // word will get the value 5
    string.clear(); // this empties the String, making it equal to ""

    // `word` still has the value `5` here, but there's no more string that we could meaningfully use the value `5` with. `word` is now totally invalid! `word` is getting out of sync with the data in `string`.

    // String Slices (also works for Arrays, and all sorts of other collections)
    // Slices ensure memory safety.

    // Range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.
    let string = String::from("hello world");
    let hello = &string[0..5]; // "hello"
    let world = &string[6..11]; // "world"

    // Range syntax (`..`)
    // By default it starts at index 0, equivalents:
    let slice = &string[0..2];
    let slice = &string[..2];

    // By default it ends at the last index, equivalents:
    let slice = &string[3..string.len()];
    let slice = &string[3..];

    // Slice of the entire string
    let slice = &string[0..string.len()];
    let slice = &string[..];

    // By default the trailing (right) number is excluded you can include it with `=`
    let hello = &string[0..=4]; // Index 0 (included) and index 4 (included) => "hello"

    // similar to:
    let hello = &string[0..5]; // Index 0 (included) and index 5 (excluded) => "hello"

    // 3rd example (with slices):
    fn first_word_3(string: &str) -> &str {
        for (index, character) in string.chars().enumerate() {
            if character == ' ' {
                return &string[0..index];
            }
        }
        string
    }
    // Now the compiler will ensure the references into the `String` remain valid.
    let mut string = String::from("hello world");
    let word = first_word_3(&string);
    // string.clear(); // Error: Rust disallows the mutable reference in `clear` and the immutable reference in `word` from existing at the same time, and compilation fails.
    println!("the first word is: {}", word);

    // String Literals as Slices
    let string = "Hello, world!";
    // `string` is a String Literal (`&str`) => it's a slice pointing to that specific point of the binary.
    // `&str` is an immutable reference.
    // => String Literals are already String Slices.

    // String Slices as Parameters
    fn first_word_4(string: &str) -> &str {
        for (index, character) in string.chars().enumerate() {
            if character == ' ' {
                return &string[0..index];
            }
        }
        string
    }
    // `&str` allows us to use the same function on both `&String` and `&str` values => deref coercions.
    // => makes our API more general and useful without losing any functionality:
    let my_string = String::from("hello world");
    let word = first_word_4(&my_string[0..6]);
    let word = first_word_4(&my_string[..]);
    let word = first_word_4(&my_string);

    let my_string_literal = "hello world";
    let word = first_word_4(&my_string_literal[0..6]);
    let word = first_word_4(my_string_literal);
    let word = first_word_4(my_string_literal);
}
