// Assignment 1.4: Explicit Relationship Tracking - Lifetime Annotations

struct TextCache<'a> {
    text: &'a str,
}

impl<'a> TextCache<'a> {
    fn new(text: &str) -> TextCache {
        TextCache { text: text }
    }

    fn read(&self) {
        println!("{}", self.text);
    }
}

fn main() {
    let cache;

    {
        let data = String::from("Hello");
        cache = TextCache::new(&data);
    }

    cache.read();
}
