// Assignment 3.1: Multi-Module Applications
mod math;
mod utils;

fn main() {
    let sum = math::add(10, 5);
    let greeting = utils::greet("Ezra");

    println!("{}", sum);
    println!("{}", greeting);

    // Compiler Error: function 'sub' is private
    // math::sub(10, 5);
}
