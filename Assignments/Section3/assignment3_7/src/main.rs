// Assignment 3.7: In-Memory Key-Value Store
use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Database {
    store: HashMap<String, String>,
}

impl Database {
    fn new() -> Database {
        Database {
            store: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    fn get_or_default(&self, key: String) -> String {
        match self.store.get(&key) {
            Some(s) => s.clone(),
            None => return String::from("Not Found"),
        }
    }

    fn update_strict(&mut self, key: String, new_value: String) -> bool {
        match self.store.entry(key) {
            Entry::Occupied(mut e) => {
                e.insert(new_value);
                true
            }
            Entry::Vacant(_) => false,
        }
    }
}

/**
 * 1. Insert User1 -> Admin
 * 2. Update User1 -> SuperAdmin
 * 3. Update User2 -> SuperAdmin (should return false)
 * 4. Print final value of User1 in Database
 */
fn main() {
    let mut db = Database::new();
    let key1 = String::from("User1");
    let value1 = String::from("Admin");

    db.insert(key1.clone(), value1);
    let mut res = db.update_strict(key1.clone(), String::from("SuperAdmin"));
    assert!(res);

    res = db.update_strict(String::from("User2"), String::from("SuperAdmin"));
    assert!(!res);

    println!("{}", db.get_or_default(key1));
}
