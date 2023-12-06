// Disable warnings for learning purposes and avoid useless `println!` usage only to use a variable
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_assignments)]

fn main() {
    /* Enums */
    // Define a type by enumerating its possible variants.

    // Example: Any IP address can be either a version four (IPv4) or a version six address (IPv6), but not both at the same time.
    enum IpAddressKind {
        V4,
        V6,
    }
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;
    fn route(ip_kind: IpAddressKind) {}
    route(four);
    route(six);

    // Enums with data directly (like "struct")
    enum IpAddress {
        V4(String),
        V6(String),
    }
    let home = IpAddress::V4(String::from("127.0.0.1"));
    let loopback = IpAddress::V6(String::from("::1"));

    // Advantage to use an `enum` rather than a `struct`, each variant can have different types and associated data.
    enum IpAddressCustom {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddressCustom::V4(127, 0, 0, 1);
    let loopback = IpAddressCustom::V6(String::from("::1"));

    // Storing IP addresses is only a exemple, and as it turns out doing so is common, it's included in the standard library (`std::net::IpAddr`).

    // Another example of an enum:
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // You can put any kind of data inside an enum variant: strings, numeric types, or structs, for example. You can even include another enum!

    // We can also implements methods on `enum` like for `struct` with `impl`.
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }
    let message = Message::Write(String::from("hello"));
    message.call();

    // The `Option` Enum and Its Advantages Over Null Values
    // Value could be something or it could be nothing.
    // Defined by the standard library.
    // Included in the prelude (don't need to bring it into scope explicitly).
    // Rust doesn't have the null feature that many other languages have. Null is a value that means there is no value there. In languages with null, variables can always be in one of two states: null or not-null.
    // Variants: `Some` and `None` (already in prelude, no need to use `Option::` prefix but both work).
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // `Option<T>` and `T` are different types.
    // `i8` is always a valid value, but `Option<i8>` not necessarily as it could be `None`.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // This won't compile:
    // let sum = x + y;

    // We have to convert `Option<T>` to `T` before performing `T` operations => making sure the value is valid.
    match y {
        Some(y) => {
            // `y` is `Some` and has a value
            let sum = x + y;
        }
        None => {
            // `y` is `None` and doesn't have a value
        }
    }

    /* The `match` Control Flow Construct */
    // Compare a value against a series of patterns.
    // Execute code based on which pattern matches.
    // Patterns can be made up of literal values, variable names, wildcards, and many other things.
    // `match` is an expression, it returns a value.
    // => The compiler ensures that all possible cases are handled.

    // Example: Function that takes an unknown United States coin and returns its value in cents.
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        // We must handle all the enum variants, otherwise it won't compile.
        // => Matches Are Exhaustive.
        match coin {
            // We can use `return` and `{}` or directly the value.
            Coin::Penny => {
                return 1;
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    // Patterns that Bind to Values
    // Extract values out of enum variants.

    // Example: Quarter United States Coin minted quarters with different designs for each of the 50 states on one side.
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }
    enum CoinWithState {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents_2(coin: CoinWithState) -> u8 {
        match coin {
            CoinWithState::Penny => 1,
            CoinWithState::Nickel => 5,
            CoinWithState::Dime => 10,
            // We can use `state` to get the value of the variant.
            CoinWithState::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    // Matching with `Option<T>`
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(x) => Some(x + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five); // `Some(6)`
    let none = plus_one(None); // `None`

    // Catch-all Patterns and the `_` Placeholder
    // Using enums, we can also take special actions for a few particular values, but for all other values take one default action.

    // Example: Game where, if you roll a 3 on a dice roll, your player doesn't move, but instead gets a new fancy hat. If you roll a 7, your player loses a fancy hat. For all other values, your player moves that number of spaces on the game board.
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),

        // It matches all possible values, we can call the variable however we want, here `other` and if we don't need it we can use `_`.
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(spaces: u8) {}

    // Concise Control Flow with `if let`
    // Lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest.
    // `if let` as "syntax sugar" for a `match` that runs code when the value matches one pattern and then ignores all other values.

    // Example: We want to only execute code if the value is the `Some` variant.

    // With `match`:
    let value = Some(5);
    match value {
        Some(value) => println!("The value is {}", value),
        _ => (),
    }

    // With `if let` (less boilerplate code to add):
    let value = Some(5);
    if let Some(value) = value {
        println!("The value is {}", value);
    }
}
