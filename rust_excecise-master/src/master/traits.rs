#![allow(dead_code)]
#![allow(unused_variables)]

use crate::master;
use std::ops::Add;

struct Points<T, U> {
    x: T,
    y: U,
}

impl Points<i32, f32> {
    fn get_some(&self) -> f32 {
        self.y as f32 + self.x as f32
    }
}

impl Points<&str, &str> {
    pub fn return_words(&self) -> String {
        format!("{} or {}", self.x, self.y)
    }
}

pub fn generics() {
    let one = Points { y: 1.23, x: 1 };
    let result1 = one.get_some();

    println!("{}", result1);

    let two = Points { y: "One", x: "Two" };
    let result2 = two.return_words();

    println!("{}", result2);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub fn traits() {
    let code1 = Point { x: 5.0, y: 5.0 };
    let code2 = Point { x: 1.0, y: 2.0 };

    let sum = code1 + code2;

    master::debug_print::print(sum);
}

//-----------------------traits----------------------------

trait Person{
    fn name(&self)->&str;
}

trait Student:Person{
 fn complete_info(&self)->(&str,u8,&str);
}

struct UniStudent {
    name_std:String,
    age :u8,
    university:String
}

impl Person for UniStudent {
    fn name(&self) -> &str {
        &self.name_std
    }
}

impl Student for UniStudent {
    fn complete_info(&self) -> (&str, u8, &str) {
        (&self.name(),self.age,&self.university)
    }
}