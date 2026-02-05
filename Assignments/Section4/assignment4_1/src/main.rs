// Assignment 4.1: String Interning System

struct Interner {
    content: String,
}

impl Interner {
    fn new(content: &str) -> Self {
        Self {
            content: String::from(content),
        }
    }

    fn get_or_update(&mut self, new_val: &str) -> &str {
        if new_val.len() > self.content.len() {
            self.content = String::from(new_val);
        }

        &self.content
    }
}

fn pick_longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    y
}

fn main() {
    let mut interner = Interner::new("Hellooooo");
    println!("{}", interner.get_or_update("Bruhh"));
    println!("{}", pick_longer("short", "looooong"));
}
