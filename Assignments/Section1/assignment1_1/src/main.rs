// Assignment 1.1: Demonstrate Memory Leak Detector in Rust.

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
        println!("Freeing ID: {}", self.id)
    }
}

fn main() {
    let ta1 = TracedAllocation::new(1);

    {
        let ta2 = TracedAllocation::new(2);
    }

    let ta3 = TracedAllocation::new(3);
}
