use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::net::IpAddr;

// Disable warnings for learning purposes and avoid useless `println!` usage only to use a variable
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(unreachable_code)]

fn main() {
    /* Error Handling */

    /*  2 categories:
            1. `Result<T, E>`: Recoverable Errors (e.g: file not found, invalid input, etc.) => Report the error to the user and retry the operation.
            2. `panic!`: Unrecoverable Errors (e.g: index out of bounds, etc.) => Symptoms of bugs, we want to immediately stop the program.
    */

    /* Unrecoverable Errors with `panic!` */
    // Sometimes, bad things happen in your code, and there's nothing you can do about it.
    // By default, these panics will print a failure message, unwind, clean up the stack, and quit.
    // We can also display the call stack of the error by setting the `RUST_BACKTRACE` environment variable to `1`.
    panic!("crash and burn");

    // `panic!` call coming from a library
    let vector = vec![1, 2, 3];
    vector[99];
    // We're attempting to access the 100th element of our vector but the vector has only 3 elements.
    // Attempting to read beyond the end of a data structure is undefined behavior.
    // You might get whatever is at the location in memory that would correspond to that element in the data structure, even though the memory doesn't belong to that structure (buffer overread). It leads to security vulnerabilities.
    // Rust will `panic!` to protect you from this sort of vulnerability.

    /* Recoverable Errors with `Result<T, E>` */
    // Most errors aren't serious enough to require the program to stop entirely.
    // Sometimes, when a function fails, it's for a reason that you can easily interpret and respond to.
    // Example: If you try to open a file and that operation fails because the file doesn't exist, you might want to create the file instead of terminating the process.
    // Variants: `Ok` and `Err`
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        }
    };

    // Matching on Different Errors (using `error.kind()`)
    // Example: We want to create the file if it doesn't exist, but `panic!` if we get any other error (e.g: we didn't have permission to open the file).
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file_created) => file_created,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // Alternatives to Using `match` with `Result<T, E>`
    // Sometimes, there are lot of `match` statements to handle `Result<T, E>` and it can be hard to read.
    // There are many other methods available on `Result<T, E>` that make it easier to handle the different variants (e.g `.unwrap`, `.expect`, `.unwrap_or_else`, etc.).

    // Shortcuts for Panic on Error: `.unwrap` and `.expect` (`Result<T, E>` helper methods)

    // `.unwrap` will `panic!` if the value of the `Result<T, E>` is an `Err` variant or otherwise return the `Ok` value.
    let greeting_file = File::open("hello.txt").unwrap();

    // `.expect` is similar to `.unwrap` but we can also provide a custom panic message.
    let greeting_file = File::open("hello.txt").expect("Failed to open `hello.txt`.");

    /* Propagating Errors */
    // When a function's implementation calls something that might fail.
    // Instead of handling the error within the function itself, we can return the error to the calling code so that it can decide what to do.

    // Example: Function that reads a username from a file. If the file doesn't exist or can't be read, this function will return those errors to the code that called the function.
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(error) => return Err(error),
        };
        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(error) => Err(error),
        }
    }

    // Shortcuts for Propagating Errors: `?` Operator
    // `?`: If the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned from this expression, and the program will continue. If the value is an `Err`, the `Err` will be returned.
    // `?` can only be used in functions that return `Result` or `Option`.
    fn read_username_from_file_shorter() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    // The `?` operator eliminates a lot of boilerplate and makes this function's implementation simpler.
    // Example: `?` chaining
    fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    // Reading a file into a string is a fairly common operation so the standard library provides `fs::read_to_string` to do this in one line.
    fn read_username_from_file_one_liner() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    // `?` also works with `Option<T>` values.
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    /* To `panic!` or Not to `panic!` */
    // Good default choice: `Result<T, E>`.
    // => It gives the calling code options to decide how to handle the error (recover or call `panic!` themselves).

    // Examples, Prototype Code, and Tests
    // => `panic!` is appropriate.

    // - Example: When you're writing an example to illustrate some concept, it's understood that a call to a method like `.unwrap` that could panic is meant as a placeholder for the way you'd want your application to handle errors.

    // - Prototype: `.unwrap` and `.expect` methods are very handy when prototyping, before you're ready to decide how to handle errors.

    // - Tests: If a method call fails in a test, you'd want the whole test to fail, `panic!` is how a test is marked as a failure, calling `.unwrap` or `.expect` is exactly what should happen.

    // Similarly, `panic!` is often appropriate if you're calling external code that is out of your control and it returns an invalid state that you have no way of fixing.

    // Cases in Which You Have More Information Than the Compiler
    // It would also be appropriate to call `.unwrap` or `.expect` when you have some other logic that ensures the `Result` will have an `Ok` value, but the logic isn't something the compiler understands.
    // Example:
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}
