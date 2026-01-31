// Assignment 1.7: Configuration Manager

#[derive(Debug, Clone)]
struct Config {
    port: u16,
    retries: u8,
    name: String,
}

fn main() {
    let config_a = Config {
        port: 443,
        retries: 10,
        name: String::from("Ezra"),
    };
    let config_b = config_a.clone();

    println!("Printing Config A: {:?}", config_a);
    println!("Printing Config B: {:?}", config_b);
}
