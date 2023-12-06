use std::io::{self, Write};

use chapter_11_testing::add;

fn main() {
    println!("Adding `numbers_count` integers together!");

    let mut integers_count = String::from("");
    print!("`numbers_count`: ");
    io::stdout().flush().expect("Failed to flush `stdout`.");
    io::stdin()
        .read_line(&mut integers_count)
        .expect("Failed to read `stdin` line.");
    let numbers_count: usize = integers_count
        .trim()
        .parse()
        .expect("Failed to convert `numbers_count` as an `usize`.");

    if numbers_count == 0 {
        panic!("`numbers_count` should not be 0.");
    }

    let mut numbers = Vec::new();
    for index in 0..numbers_count {
        let mut number = String::from("");
        print!("`number{index}`: ");
        io::stdout().flush().expect("Failed to flush `stdout`.");
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read `stdin` line.");
        let number: i32 = number
            .trim()
            .parse()
            .unwrap_or_else(|_| panic!("Failed to convert `number{index}` as an `i32`."));
        numbers.push(number);
    }

    let result = add(&numbers);
    let numbers = numbers
        .iter()
        .map(|number| number.to_string())
        .collect::<Vec<String>>()
        .join(" + ");
    println!("{numbers} = {result}");
}
