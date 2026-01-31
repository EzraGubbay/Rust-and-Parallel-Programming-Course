// Assignment 2.8: Heterogeneous Serialization Framework

trait Serialize {
    fn to_json(&self) -> String;
}

impl Serialize for i32 {
    fn to_json(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl Serialize for String {
    fn to_json(&self) -> String {
        format!("\"{}\"", self)
    }
}

struct User {
    name: String,
    age: u8,
}

impl Serialize for User {
    fn to_json(&self) -> String {
        format!("{{ {}, {} }}", self.name.to_json(), self.age)
    }
}

fn main() {
    let mut list: Vec<Box<dyn Serialize>> = Vec::new();
    list.push(Box::new(10));
    list.push(Box::new(String::from("Hello")));
    list.push(Box::new(User {
        name: String::from("Ezra"),
        age: 26,
    }));

    for item in list {
        println!("{}", item.to_json());
    }
}
