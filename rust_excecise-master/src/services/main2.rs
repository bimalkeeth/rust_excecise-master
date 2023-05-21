#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

pub fn operators() {
    for n in 1..10 {
        println!("Hello, world! {}", n);
    }

    let c = 241342124;
    println!("c = {}  takes up {} bytes", c, mem::size_of_val(&c));

    let d: char = 'x';
    println!("x size of the {}", mem::size_of_val(&d));

    let m = Box::new(10);
    println!("boxed value {}", *m);

    std::mem::drop(m);

    let mut c = 2 + 4 * 3;
    println!("before {}", c);

    c = c + 1;

    println!(" after {}", c);
}
