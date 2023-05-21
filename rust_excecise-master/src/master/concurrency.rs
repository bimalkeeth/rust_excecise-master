#![allow(dead_code)]
#![allow(unused_variables)]

use num::{BigUint, One};
use std::sync::{Arc, Mutex};

pub fn threads() {
    let vec: Vec<i32> = vec![1, 2, 3];
    let handle = std::thread::spawn(move || println!("{:?}", vec));

    handle.join().unwrap();

    let vec2: Vec<i32> = vec![1, 2, 3];
    let mut thread_handlers = Vec::new();
    for e in vec2 {
        thread_handlers.push(std::thread::spawn(move || println!("Thread {}", e)))
    }

    println!("Main thread");

    for handle in thread_handlers {
        handle.join().unwrap();
    }
}

pub fn mutext_locks() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", counter.lock().unwrap());
}

pub fn factorial(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        return BigUint::one();
    } else {
        (1..=num)
            .map(BigUint::from)
            .reduce(|acc, x| acc * x)
            .unwrap()
    }
}
