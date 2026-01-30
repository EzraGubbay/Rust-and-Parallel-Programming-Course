// Assignment 1.2: Resource Transfer System

struct TracedAllocation {
    id: i32,
}

impl TracedAllocation {
    fn new(id: i32) -> Self {
        println!("Allocating ID: {}", id);
        TracedAllocation { id }
    }
}

// This function is called automatically when the variable goes out of scope
impl Drop for TracedAllocation {
    fn drop(&mut self) {
        println!("Freeing ID: {}", self.id);
    }
}

fn process_resource(res: TracedAllocation) {
    println!("Processed ID: {}", res.id);
}

fn main() {
    let ta = TracedAllocation::new(1);

    println!("Before transfer");
    process_resource(ta);
    println!("After transfer");
    // println!(
    //     "Attempting to print TracedAllocation ID (this shouldn't work): {}",
    //     ta.id
    // );
}
