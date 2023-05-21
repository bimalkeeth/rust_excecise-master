#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    return Point { y: 0.0, x: 0.0 };
}

pub fn stack_and_heap() {
    let p1 = origin(); //stack allocated
    let p2 = Box::new(origin()); //heap allocated

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));
}
