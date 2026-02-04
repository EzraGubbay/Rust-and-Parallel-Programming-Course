// Assignment 3.5: Event Handler System

fn run_simulation<F>(max_events: i32, mut handler: F)
where
    F: FnMut(i32),
{
    for i in 1..max_events {
        handler(i);
    }
}

fn main() {
    let mut event_count = 0;
    let count_ptr = &mut event_count;
    let prefix = String::from("Event");

    let handler = move |id| {
        *count_ptr += 1;
        println!("{prefix} - ID: {id}");
    };

    run_simulation(5, handler);
    println!("Final count: {event_count}");
    // println!("{prefix}");
}
