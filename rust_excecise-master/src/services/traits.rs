#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Add;

//  trait definition.
trait Summable<T> {
    fn sum(&self) -> T;
}

//  implementation of trait above.
impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result: i32 = 0;
        for x in self {
            result += *x;
        }
        result
    }
}

pub fn vector_summ() {
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("sum of vec {}", a.sum());

    let john = Person::new("John");

    println!("name of the person {}", john.name);

    let x = Complex::new(1, 2);
    println!("{:?}", x);

    let y = Complex::new(3, 8);

    println!("{:?}", x + y)
}

struct Person {
    name: String,
}

impl Person {
    // fn new(name:&str)->Person{
    //     Person{name:name.to_string()}
    // }

    fn new<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("{} is dead", self.name)
    }
}

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { im, re }
    }
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
