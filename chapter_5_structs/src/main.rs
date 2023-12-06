// Disable warnings for learning purposes and avoid useless `println!` usage only to use a variable
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_assignments)]

fn main() {
    /* Structs */
    // Structure Related Data.
    // Similar to tuples: both hold multiple related values.
    // Unlike with tuples, in a struct you'll name each piece of data.
    struct User {
        email: String,
        username: String,
        active: bool,
        sign_in_count: u64,
    }

    // Create an instance of that struct by specifying concrete values for each of the fields.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Get a specific value
    println!("{}", user1.email);

    // Set a specific value (only possible if the variable is `mut`)
    user1.username = String::from("User");

    let username = user1.username.clone();

    // Destructuring
    let User { email, .. } = user1;

    // Creating an instance of a struct is a expression
    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username, // Shorthand struct initialization
            active: true,
            sign_in_count: 1,
        }
    }

    // Creating Instances From Other Instances With Struct Update Syntax
    // => Create a new instance of a struct that includes most of the values from another instance, but changes some.

    // Without update syntax
    let user2 = User {
        active: user1.active,
        username: user1.username.clone(),
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // With update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    /* Tuple structs */
    // Using Tuple Structs without Named Fields to Create Different Types.
    // => Don't have names associated with their fields.
    // => Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Destructuring
    // let (x, y, z) = origin; // not possible with Tuple Structs
    // instead, we use:
    let Point(x, y, z) = origin;

    // Get a element:
    let x = origin.0;

    /* Example Program using structs */
    // Calculates the area of a rectangle.

    // By default `struct` does not include functionality to print out debugging information (with `{:?}`).
    // We have to explicitly opt in with `#[derive(Debug)]`.
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // We're using `&` (immutable borrow) so we borrow the struct rather than take ownership of it. This way, `main` retains its ownership and can continue using it later after the function call.
    fn calculate_rectangle_area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    let rectangle_example = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        calculate_rectangle_area(&rectangle_example)
    );

    // Adding Useful Functionality with Derived Traits like (`#[derive(Debug)]`)
    // Example: Ability to print an instance of Rectangle.
    // => The `derive` attribute generates code that will implement a trait with its own default implementation on the type you've annotated.
    println!("rectangle_example is {:?}", rectangle_example);

    /* Method syntax */
    // Similar to functions are defined within the context of a struct (or an enum or a trait object).
    // First parameter is always `self`, which represents the instance of the struct the method is being called on.
    // `impl`: Implementation block.
    impl Rectangle {
        // `&self` is short for `self: &Self` (both are valid)
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // `&mut self` to change instance fields
        fn double_width(&mut self) {
            self.width *= 2;
        }

        /**
         * Returns `true` if the second Rectangle can fit completely within `self`.
         */
        fn can_hold(&self, rectangle: &Rectangle) -> bool {
            self.area() >= rectangle.area()
        }

        // Associated Functions that don't have `self` as first parameter like "static"
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle_example.area()
    );
    let square = Rectangle::square(5);
    println!("The area of the square is {} square pixels.", square.area()); // 25

    let rectangle1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rectangle2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rectangle3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!(
        "Can rectangle1 hold rectangle2? {}",
        rectangle1.can_hold(&rectangle2)
    ); // true
    println!(
        "Can rectangle1 hold rectangle3? {}",
        rectangle1.can_hold(&rectangle3)
    ); // false
}
