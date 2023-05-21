#![allow(dead_code)]
#![allow(unused_variables)]

pub fn process_value() {
    let x = 3.0;
    let y = 2.0;

    let result = if y != 0.0 { Some(x / y) } else { None };
    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("cannot divide by zero"),
    }
}
