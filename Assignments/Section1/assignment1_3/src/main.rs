// Assignment 1.3: Shared Buffer Manager

struct Buffer {
    data: i32,
}

impl Buffer {
    fn new(data: i32) -> Buffer {
        Buffer { data }
    }
}

fn main() {
    let buf = Buffer::new(5);

    let reader1 = &buf;
    let reader2 = &buf;
    // let writer = &mut buf;

    // writer.data = 0;
}
