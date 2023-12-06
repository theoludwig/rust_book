use crate::garden::vegetables::Asparagus;

// Tells the compiler to include the code it finds in `src/garden/mod.rs`.
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
