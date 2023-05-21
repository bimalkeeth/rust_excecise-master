#![allow(dead_code)]
#![allow(unused_variables)]

pub fn print<T: std::fmt::Debug>(val: T) {
    println!("{:?}", val)
}
