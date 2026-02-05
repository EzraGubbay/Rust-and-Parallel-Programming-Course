// Assignment 3.8: Text Processing Utility

fn process_text(input: &str) -> String {
    input
        .chars()
        .take(5)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
}

fn main() {
    let test1 = "Hello World";
    let test2 = "🦀🚀🔥⛔🟢";
    let test3 = "abc";

    println!("{}", process_text(test1));
    println!("{}", process_text(test2));
    println!("{}", process_text(test3));
}
