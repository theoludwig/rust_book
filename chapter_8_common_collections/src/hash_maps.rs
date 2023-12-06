use std::collections::HashMap;

// Disable warnings for learning purposes and avoid useless `println!` usage only to use a variable
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_mut)]

pub fn main_hash_maps() {
    /* Hash Maps (`HashMap<K, V>`) */
    // Stores a mapping of keys of type `K` to values of type `V` using a hash function (like a dictionary).

    // Creating a New Hash Map
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing Values in a Hash Map
    let team_name = String::from("Blue");

    // `get` returns an `Option<&V>` (a reference to the value of type `V`).
    // We can use `.copied` after `.get` to get a copy of the value instead of a reference so we get `Option<V>` instead of `Option<&V>`.
    // match scores.get(&team_name).copied() {
    match scores.get(&team_name) {
        Some(score) => println!("{team_name}: {score}."),
        None => println!("The team {team_name} does not exist."),
    }

    // Another way to handle `Option` returned by `get`:
    // `unwrap_or` returns the value inside the `Some` variant if it exists, otherwise it returns the value passed as an argument.
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Iterating Over Each Key-Value Pair in a Hash Map (arbitrary order)
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hash Maps and Ownership
    // For types that implement the `Copy` trait, like `i32`, the values are copied into the Hash Map.
    // For owned values like `String`, the values will be moved and the Hash Map will be the owner of those values.

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point.
    // println!("{field_name}: {field_value}"); // error: use of moved values: `field_name`, `field_value`

    // Overwriting a Value
    // If we insert a key and a value into a Mash Map and then insert that same key with a different value, the value associated with that key will be replaced.
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 25);
    scores.insert(String::from("Blue"), 50);
    println!("{:?}", scores); // {"Blue": 50}

    // Only Inserting a Value If the Key Has No Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores); // {"Blue": 10, "Yellow": 50}

    // Updating a Value Based on the Old Value
    // Example: Counting how many times each word appears in some text.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map); // {"hello": 1, "world": 2, "wonderful": 1}
}
