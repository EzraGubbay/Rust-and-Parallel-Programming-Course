// Assignment 1.5: Resource Transfer System

struct BigData {
    data: [u8; 10_000_000],
}

fn main() {
    let x = Box::new(BigData {
        data: [0; 10_000_000],
    });

    println!("Size of x on the stack: {}", std::mem::size_of_val(&x));
}
