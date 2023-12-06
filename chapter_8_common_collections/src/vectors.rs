// Disable warnings for learning purposes and avoid useless `println!` usage only to use a variable
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_mut)]

pub fn main_vectors() {
    /* Vectors (`Vet<T>`) */
    // Storing Lists of Values.

    // Creating a New Vector
    let _vector: Vec<i32> = Vec::new(); // empty vector
    let mut vector = vec![1, 2, 3]; // vector with values
    vector.push(4); // add value to vector (given that vector is mutable `mut`)

    // Reading Elements of Vectors
    let third: &i32 = &vector[2]; // will panic if index is out of bounds
    println!("The third element is {third}.");

    // Safer Access with `get` (handle index out of bounds explicitly)
    let third: Option<&i32> = vector.get(2);
    match third {
        Some(third) => {
            println!("The third element is {third}.");
        }
        None => println!("There is no third element."),
    }

    // Borrow checker rule
    let mut vector = vec![1, 2, 3, 4, 5];
    let first = &vector[0];
    // vector.push(6); // will cause error: cannot borrow `vector` as mutable because it is also borrowed as immutable
    println!("The first element is: {first}");
    // => We can't have mutable and immutable references in the same scope.
    // That rule applies here, we hold an immutable reference to the first element in a vector and try to add an element to the end.
    // This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn't enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory.
    // => Instead we can use the `get` method to get a reference to an element without holding a reference to the whole vector.

    // Iterating over the Values in a Vector
    let vector = vec![100, 32, 57];
    for item in &vector {
        println!("{item}");
    }

    // We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.
    let mut vector = vec![100, 32, 57];
    for item in &mut vector {
        *item += 50;
    }

    // Iterating over a vector, whether immutably or mutably, is safe.
    // If we try `vector.push(4);` in the loop, we'll get a compile-time error (borrow checker rule).

    // Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    match row.get(0) {
        Some(value) => match value {
            SpreadsheetCell::Int(value) => {}
            SpreadsheetCell::Float(value) => {}
            SpreadsheetCell::Text(value) => {}
        },
        None => (),
    }
}
