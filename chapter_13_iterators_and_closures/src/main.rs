use crate::shirts::ShirtColor;
use crate::shirts::ShirtsInventory;

pub mod shirts;

fn main() {
    /* Functional Language Features: Iterators and Closures */
    // - Closures, a function-like construct you can store in a variable.
    // - Iterators, a way of processing a series of elements.

    // Exercise Example:
    // - Our t-shirt company gives away an exclusive, limited-edition shirt to someone on our mailing list as a promotion.
    // - People on the mailing list can optionally add their favorite color to their profile.
    // - If the person chosen for a free shirt has their favorite color set, they get that color shirt.
    // - If the person hasn't specified a favorite color, they get whatever color the company currently has the most of.
    let closure = |num: u32| -> u32 { num };
    closure(5);
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: i32| x + 1;

    add_one_v1(5);
    add_one_v2(5);
    add_one_v3(5);

    let store = ShirtsInventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_preference_1 = Some(ShirtColor::Red);
    let giveaway_1 = store.giveaway(user_preference_1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_preference_1, giveaway_1
    );

    let user_preference_2 = None;
    let giveaway_2 = store.giveaway(user_preference_2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_preference_2, giveaway_2
    );

    // Defining a function that takes a closure
    // - `FnOnce` applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
    // - `FnMut` applies to closures that don't move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
    // - `Fn` applies to closures that don't move captured values out of their body and that don't mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.
    fn example<F: Fn()>(f: F) {
        f();
    }
    example(|| println!("This is a closure!"));

    // Iterators
    // Allows you to perform some task on a sequence of items.
    let numbers: Vec<i32> = vec![1, 2, 3];
    let mut numbers_iterator = numbers.iter();
    for (index, number) in numbers_iterator.enumerate() {
        println!("number{index}: {number}");
    }

    numbers_iterator = numbers.iter();
    assert_eq!(numbers_iterator.next(), Some(&1));
    assert_eq!(numbers_iterator.next(), Some(&2));
    assert_eq!(numbers_iterator.next(), Some(&3));
    assert_eq!(numbers_iterator.next(), None);

    // Methods that Produce Other Iterators (e.g: `.map`)
    // `.collect` is a method that takes an iterator and collects the resulting items into a collection data type.
    let vector: Vec<_> = numbers.iter().map(|number| number + 1).collect();
    assert_eq!(vector, vec![2, 3, 4]);
}
