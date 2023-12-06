// Disable warnings for learning purposes and avoid useless `println!` usage only to use a variable
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_assignments)]

fn main() {
    /* Variables */
    // Variables are declared with the `let` keyword.
    // By default variables are immutable (once a value is bound to a name, you can't change that value).
    let x = 10;

    // Not allowed (immutable):
    // let x: i32 = 5;
    // x = 6;

    // Allowed (`mut` make variable mutable):
    let mut x: i32 = 5;
    x = 6;

    // Get the size in bytes of a variable
    println!("{}", std::mem::size_of_val(&x));

    // Get the size in bytes of a type
    println!("{}", std::mem::size_of::<i32>());

    // Constants
    /* Like immutable variables, but with differences:
        - You can't use `mut` (always immutable).
        - Can only be set to a constant expression, not the result of a value that could only be computed at runtime (compiler is able to evaluate a limited set of operations at compile time).
    */
    const HOUR_IN_SECONDS: u32 = 60 * 60;

    // Shadowing
    /* Declare a new variable with the same name, different from mutable variables:
        - By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
        - We can also change the type of the variable but reuse the same name.
    */
    // Example: We have a string that we want to convert to integer, with a variable with the same name
    let number = "58";
    let number: i32 = number.parse().expect("Parsing failed");
    let number = number + 1;
    println!("{number}"); // 59

    /* Data Types */
    // Rust is a statically typed language => know the types of all variables at compile time.
    // 2 data type subsets: Scalar and Compound.
    // Can usually infer the type except in cases when many types are possible (e.g: convert a String to a numeric type)
    let guess: u32 = "42".parse().expect("Not a number!");

    // Scalar Types
    // Represents a single value. Rust has 4 primary scalar types: integers, floating-point numbers, booleans, and characters.

    // Integer Types
    // Number without a fractional component.
    // Signed integer types start with `i` ; Unsigned integer types start with `u`
    // Default: `i32`

    // 8 bits (1 byte)
    let number: i8 = -5; // -128 to 127 (from -2^7 to 2^7 - 1)
    let number: u8 = 5; // 0 to 255 (2^8 = 256)

    // 16 bits (2 bytes)
    let number: i16 = -5; // -32 768 to 32 767
    let number: u16 = 5; // 0 to 65 535 (2^16 = 65 536)

    // 32 bits (4 bytes)
    let number: i32 = -5; // -2 147 483 648 to 2 147 483 647
    let number: u32 = 5; // 0 to 4 294 967 295 (2^32 = 4 294 967 296)

    // 64 bits (8 bytes)
    let number: i64 = -5; // (from -2^63 to 2^63 - 1)
    let number: u64 = 5;

    // 128 bits (16 bytes)
    let number: i128 = -5;
    let number: u128 = 5;

    // Arch (Depend on the architecture of the computer your program is running on: 64 bits if you're on a 64-bit architecture and 32 bits if you're on a 32-bit architecture).
    // Often used for indexing some sort of collection.
    let number: isize = -5;
    let number: usize = 5;

    // Integer Literals
    let decimal = 98_222; // (`_` as a visual separator to make the number easier to read)
    let hexadecimal = 0xff; // 255
    let octal = 0o77; // 63
    let binary = 0b1111_0000; // 240
    let byte = b'A'; // 65

    // Integer Overflow
    // Rust uses the term panicking when a program exits with an error (e.g: Integer Overflow).
    // Type `u8` that can hold values between 0 and 255, but if we try to store 256 in a `u8`, we'll get an error:
    // let number: u8 = 255;
    // let number = number + 1;
    /* To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:
        - Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`.
        - Return the `None` value if there is overflow with the `checked_*` methods.
        - Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
        - Saturate at the value's minimum or maximum values with the `saturating_*` methods.
    */
    let guess = "256";
    let guess: u8 = guess.parse().expect("Not a number!"); // In this case, `parse` returns the `Err` variant and the program will crash and display the message.

    // Floating-Point Types
    // Numbers with decimal points.
    // All floating-point types are signed.
    // Floating-point numbers are represented according to the IEEE-754 standard.
    // Default: `f64`
    let x: f64 = 2.0; // f64 (double precision)
    let y: f32 = 3.0; // f32 (single precision)

    // Numeric Operations
    // Rust supports the basic mathematical operations
    let addition = 5 + 10;
    let subtraction = 95.5 - 4.3;
    let multiplication = 4 * 30;
    let division = 56.7 / 32.2;
    let remainder = 43 % 5;

    // Boolean Type (1 byte)
    let true_value: bool = true;
    let false_value: bool = false;

    // Character Type (4 bytes)
    // Most primitive alphabetic type: Represents a Unicode Scalar Value (not UTF-8, it is always 4 bytes) => can represent a lot more than just ASCII.
    // Encoding-independent.
    // Declared with single quotes.
    let heart_eyed_cat: char = '😻';
    /* Side note:
    'U+xxxx' notation is used to refer to Unicode Codepoints. (e.g: 'U+0041' is the Unicode Codepoint for the 'A' character)

    Unicode idea is to always provide the base characters and the combining characters (e.g: 'é', is combined with 'e' U+0065 and U+0301 the accent itself) separated.
    But Unicode important goal was to have round-trip compatibility for existing, widely-used encodings (like ASCII), so some pre-composed characters were included (e.g: 'é' already included with ASCII).

    That means "é" != "é", even if correctly working Unicode-capable should render the same way and be treated the same way, so the user doesn't notice the difference.
    However the difference is noticeable in the code:
        - 'é' is stored as 1 character (UTF-8 stored with 2 bytes, ASCII stored with 1 byte)
        - "é" is stored as 2 characters, therefore not possible to store in a `char` but must be in a `String` (UTF-8 stored with 3 bytes)

    Also, even if pre-composed characters in ASCII and UTF-8 can have the same decimal value (e.g: 'é', 233), it does not take the same amount of bytes to represent it in memory as the way to represent it in binary differs between ASCII and UTF-8.
    */

    // Compound Types
    // Compound types can group multiple values into one type. Rust has 2 primitive compound types: tuples and arrays.

    // Tuple Type
    // General way of grouping together values.
    let mut tuple: (i32, f64, u8) = (500, 6.4, 1);

    // Accessing Tuple Values
    let x = tuple.0; // acessing value at index 0 => 500
    let y = tuple.1; // acessing value at index 1 => 6.4
    tuple.1 = 23.4; // changing value at index 1 (possible thanks to `mut`) => (500, 23.4, 1)

    // Destructuring
    let (x, y, z) = tuple;

    // Swapping Values
    let a = 5;
    let b = 10;
    let (a, b) = (b, a);

    // Unit value => Tuple without any value.
    // Written as `()`.
    // Represent an empty value or an empty return type (default return type of functions).

    // Array Type
    // Collection of multiple values.
    // Unlike a tuple, every element of an array must have the same type
    // Arrays in Rust have a fixed length.
    // Useful when you want your data allocated on the stack rather than the heap.
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // The first month is January
    println!("The first month is {}", months[0]);

    // Pretty print
    println!("{:?}", months);

    // Debug print
    dbg!(months);

    // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon.
    let array: [i32; 5] = [3; 5]; // same as writing: let array = [3, 3, 3, 3, 3];

    // Invalid Array Element Access
    // array[6]; // => panic! => program will crash and display a message

    /* Functions */
    // Functions are declared using the `fn` keyword.
    fn a_function() {
        println!("Another function.");
    }
    a_function();

    // Function Parameters
    // `x` and `y` are called parameters
    fn another_function(x: i32, y: i32) {
        println!("The value of x is: {x}");
    }

    // `5` and `6` are called arguments
    another_function(5, 6);

    // Statements and Expressions
    // Function bodies are made up of a series of statements optionally ending in an expression.
    // Rust is an expression-based language.
    /* Differences:
        - Statements are instructions that perform some action and do not return a value. (e.g: `let y = 6;`, `function definitions`)
        - Expressions evaluate to a resulting value. (e.g: `5 + 6` or even the value `6`, calling a function, calling a macro, scope block etc.)

    Expressions can be part of statements.
    Expressions do not include ending semicolons.
    => If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
    */

    // Blocks are expressions too => y = 4
    let y = {
        let x = 3;
        x + 1
    };

    // This is not possible in Rust, because statements do not return values so there isn't anything for `x` to bind to.
    // let x = (let y = 6);

    // Functions with Return Values
    // The return value of the function is the value of the final expression in the block of the body of a function (implicitly).
    fn five() -> i32 {
        5
    }
    let x = five(); // x = 5

    // You can return early from a function by using the `return` keyword.
    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    /* Control Flow */

    // `if` Expressions
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("number is divisible by 3.");
    } else if number % 2 == 0 {
        println!("number is divisible by 2.");
    } else {
        println!("number is not divisible by 4, 3, or 2.");
    }

    // `if` in a `let` Statement (like ternary operator)
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // `loop` Expression
    // Loop forever until you explicitly tell it to stop with `break`.
    loop {
        println!("again!");
        if condition {
            break;
        }
    }

    // Returning Values from Loops
    let mut counter = 0;
    // result = 20
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // Loop Labels to Disambiguate Between Multiple Loops
    // `break` and `continue` apply to the innermost loop at that point.
    // You can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // `while` Loops
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!");

    // `for` Loops

    // Looping Through a Collection
    let array = [10, 20, 30, 40, 50];
    for element in array {
        println!("the value is: {element}");
    }

    // `for` Loops with Range
    // `rev()` => reverse the range
    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!");

    // Loop from 0 to 4 (5 excluded)
    for number in 0..5 {
        println!("{number}"); // 0, 1, 2, 3, 4
    }

    // Loop from 0 to 5 (5 included)
    for number in 0..=5 {
        println!("{number}"); // 0, 1, 2, 3, 4, 5
    }
}
