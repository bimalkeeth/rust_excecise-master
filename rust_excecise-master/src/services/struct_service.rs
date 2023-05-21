#![allow(dead_code)]
#![allow(unused_variables)]

struct Points {
    x: f64,
    y: f64,
}

pub fn structure() {
    let p = Points { x: 2.5, y: 6.7 };
    println!("x:{} y:{}", p.x, p.y)
}
