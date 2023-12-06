// Scope of a program => prelude
// If a type you want to use isn't in the prelude, you have to bring that type into scope explicitly with a `use` statement.
use std::io::{self, Write};

/**
 * Declare a function named `main`.
 * The `main` function => first code that runs.
 * The `main` function doesn't need to return a value.
 */
fn main() {
    /* General */
    // `println!` is a macro to print to stdout (`!` => means it is a macro instead of a normal function)
    println!("Hello world!");
    print!("Hello, world!\n");

    // Get the user's input
    let mut some_variable = String::new();
    // If we hadn't imported the `io` library with `use std::io;` we could use `std::io::stdin`
    io::stdin()
        .read_line(&mut some_variable)
        .expect("Failed to read `stdin` line.");

    // Printing Values with Placeholders
    print!("Your input: {some_variable}"); // only working with variable name between {}
    print!("Your input: {}", some_variable); // result of evaluating an expression

    // Get user's input and convert it to a number
    print!("Please input a number: ");

    // `flush` => make sure the output is printed to the screen immediately before we start waiting for input (causes the buffer to be written without having completed the line with '\n')
    io::stdout().flush().expect("Failed to flush `stdout`.");

    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read `stdin` line.");

    // `trim` => remove the newline character ('\n'), if we don't do it, the `parse` will fail as it can't convert `"5\n"` to a number (`5` as an example)
    // `parse` => convert the string to a number (`u32`)
    let number: u32 = number.trim().parse().expect("Please type a number!");
    println!("Your number: {}", number);
}
