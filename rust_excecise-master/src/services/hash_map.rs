#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

pub fn hashmap_operation() {
    let mut shapes = HashMap::new();

    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("square has {} sides", shapes["square".into()]);

    for (key, val) in &shapes {
        println!("shapes of {} are {}", key, val)
    }

    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]; //brow with &
    for x in &vec {
        println!("vec {}", *x)
    }
}
