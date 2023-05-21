#![allow(dead_code)]
#![allow(unused_variables)]

pub fn vector() {
    let mut a: Vec<i32> = Vec::new();
    a.push(10);
    a.push(20);

    match a.get(1) {
        Some(x) => println!("a[1]={}", x),
        None => println!("No such element"),
    }
}
