use std::fmt::{Debug, Display};

// Disable warnings for learning purposes and avoid useless `println!` usage only to use a variable
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(unreachable_code)]

fn main() {
    /* Generic Types, Traits, and Lifetimes */

    /* Generic Data Types */
    // Function signatures or structs, that can use many different concrete data types.

    // Generic type parameters in `fn` definitions
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            // Error: `>` cannot be applied to type `T`
            // Solution: `PartialOrd` trait (only use types whose values can be ordered)
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // Generic type parameters in `struct` definitions
    struct Point<T> {
        x: T,
        y: T,
    }
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // We can use different concrete types for each generic type parameter
    struct Point2<T, U> {
        x: T,
        y: U,
    }
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    // Generic type parameters in `enum` definitions
    enum Option<T> {
        Some(T),
        None,
    }

    // Generic type parameters in `impl` blocks
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let point = Point { x: 5, y: 10 };
    println!("p.x = {}", point.x());

    // We can also specify constraints on generic types when defining methods on the type. We could, for example, implement methods only on `Point<f32>` instances rather than on `Point<T>` instances.
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    /* Traits: Defining Shared Behavior */
    // Defines functionality a particular type has and can share with other types.
    // We can use trait bounds to specify that a generic type can be any type that has certain behavior.
    // Similar to interfaces in other languages.

    // Defining a Trait (using `trait`)
    /* Example:
        We have multiple structs that hold various kinds and amounts of text: a `NewsArticle` struct that holds a news story filed in a particular location and a `Tweet` that can have at most 140 characters along with metadata like whether it was a retweet or a reply to another tweet.

        We want to make a media aggregator library crate named `aggregator` that can display summaries of data that might be stored in a `NewsArticle` or `Tweet` instance.
    */
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // Implementing a Trait on a Type (using 'for`)
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    /* Restriction: We can implement a trait on a type only if at least one of the trait or the type is local to our crate (coherence, and more specifically the orphan rule).

    Examples:
        - We can implement standard library traits like `Display` on a custom type like `Tweet`.
        - We can also implement `Summary` on `Vec<T>`.
        - We can't implement external traits on external types. For example, we can't implement the `Display` trait on `Vec<T>` (both defined in the standard library).
    */

    // Default Implementations
    pub trait SummaryWithDefault {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // Traits as Parameters (using `impl Trait`)
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // Trait Bound Syntax
    // The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound.
    pub fn notify_trait_bound<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    // Trait bounds can express more complexity in others cases than `impl Trait` syntax.
    // Example:
    pub fn notify_verbose(item1: &impl Summary, item2: &impl Summary) {}
    // vs
    pub fn notify_trait_bound_verbose<T: Summary>(item1: &T, item2: &T) {}

    // Specifying Multiple Trait Bounds with the `+` Syntax
    pub fn notify_multiple(item: &(impl Summary + Display)) {}

    // Also valid with trait bounds
    pub fn notify_trait_bound_multiple<T: Summary + Display>(item: &T) {}

    // Clearer Trait Bounds with `where` Clauses
    fn some_function_verbose<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        0
    }
    // vs
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        0
    }

    // Returning Types that Implement Traits
    // Restriction: We can only return a single type. Example: we can't return a `NewsArticle` and a `Tweet` in the same function (even if both implements `Summary`). There is a way to do that covered in Chapter 17.
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    // Using Trait Bounds to Conditionally Implement Methods
    // Example:
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn compare_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}.", self.x);
            } else {
                println!("The largest member is y = {}.", self.y);
            }
        }
    }

    /* Validating References with Lifetimes */
    // Lifetimes are another kind of generic.
    // Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.
    // Every reference in Rust has a lifetime, which is the scope for which that reference is valid.

    // Preventing Dangling References with Lifetimes
    let result;
    {
        let x = 5;
        result = &x;
    }
    // println!("result: {}", result); // error: `x` does not live long enough (once `x` goes out of scope, it will be deallocated and the memory will be invalid)

    // Generic Lifetimes in Functions
    // Lifetime Annotation Syntax
    // Lifetime annotations don't change how long any of the references live.
    // Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

    //     &i32      // a reference
    // &'a i32      // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    // Example: Function that returns the longest of two string slices.
    // Signature express the following constraint:
    // The returned reference will be valid as long as both the parameters are valid.
    // => string slices that live at least as long as lifetime `'a`.
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(&string1, string2);
    println!("The longest string is \"{}\".", result);
}
