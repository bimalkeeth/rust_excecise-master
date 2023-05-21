#![allow(dead_code)]
#![allow(unused_variables)]

fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    move |y| y > limit
}

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

pub fn sum_of_square() {
    let limit = 500;

    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x: &u32| is_even(*x))
        .fold(0, |sum, x| sum + x);

    println!("sum of the square {}", sum2)
}
