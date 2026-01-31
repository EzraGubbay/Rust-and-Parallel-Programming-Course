// Assignment 2.5: Generic Data Structure Library

struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Self { first, second }
    }

    fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

impl Pair<i32, i32> {
    fn debug(&self) {
        println!("Integer Pair: {}, {}", self.first, self.second);
    }
}

fn main() {
    let n1: f64 = 1.0;
    let n2: f64 = 2.0;

    // Compiler throws "method not found in Pair<f64, f64>" error.
    let my_p1 = Pair::new(n1, n2);

    // Compiler throws no errors here, as we are using <i32, i32>.
    let my_p2 = Pair::new(1, 2);
    my_p2.debug();

    // Main Execution
    let p1 = Pair::new(10, 20);
    let p2 = Pair::new("Hello", 5.5);

    p1.debug();
    p1.swap();

    // Compiler error: method not found in `Pair<&str, {float}>`
    // p2.debug();
}
