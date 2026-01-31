// Assignment 1.6: The File Handle Pool

use std::{
    fs::File,
    sync::atomic::{AtomicUsize, Ordering},
};

static OPEN_FILES: AtomicUsize = AtomicUsize::new(0);
static MAX_FILES: usize = 3;

struct FileHandle;

impl FileHandle {
    fn new() -> Result<FileHandle, String> {
        if OPEN_FILES.load(Ordering::SeqCst) < MAX_FILES {
            OPEN_FILES.fetch_add(1, Ordering::SeqCst);
            return Ok(FileHandle);
        } else {
            return Err("Limit Reached".to_string());
        }
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        OPEN_FILES.fetch_sub(1, Ordering::SeqCst);
        println!(
            "File closed. Open count: {}",
            OPEN_FILES.load(Ordering::SeqCst)
        )
    }
}

fn main() {
    let mut v = Vec::new();

    for _i in 0..5 {
        match FileHandle::new() {
            Ok(handle) => v.push(handle),
            Err(e) => println!("{}", e),
        }
    }

    drop(v);

    FileHandle::new();
}
