#![allow(dead_code)]

use std::{sync::Arc, sync::Mutex, thread};

pub mod exercises;
pub mod factorial;
pub mod prime_factorial;
pub mod quick_find;
pub mod quick_sort;
pub mod quick_union;
pub mod weighted_quick_union;

fn main() {
    let count = Arc::new(Mutex::new(0));
    let mut threads = vec![];
    for _ in 0..10 {
        let count = Arc::clone(&count);
        threads.push(thread::spawn(move || {
            let mut count = count.lock().unwrap();
            *count += 1;
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    println!("{}", count.lock().unwrap());
}
