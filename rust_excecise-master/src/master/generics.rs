#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::collections::HashMap;

fn square<T>(x: T) -> T where T: std::ops::Mul<Output=T> + Copy + std::ops::Add {
    x * x
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> where T: std::fmt::Debug, U: std::fmt::Debug {
    fn printing(&self) {
        println!("the value of point coordination {:?} {:?}", self.y, self.x)
    }
}

pub fn generics_process() {
    println!("the square of the number {}", square(3.6))
}

//-----------------------------Option Enum--------------------------------------

pub fn option_enum(){
    let mut disease :Option<String>=None;
    disease =Some(String::from("diabetes"));

    match disease {
        Some(disease_name)=>println!("you have the disease of {}",disease_name),
        None=>println!("you do not have diabetes"),
    }
}


pub fn option_result(){
    println!("{:?}",division(9.0,3.0));
    println!("{:?}",division(4.0,0.0));
    println!("{:?}",division(0.0,2.0))
}

fn division(dividend:f64,divisor:f64) ->Result<f64,String>{
    if divisor==0.0{
        Err(String::from("error divide by zero"))
    }else{
        Ok(dividend/divisor)
    }
}

pub fn hash_mapper(){
    let some_vec=vec![5,5,8,8,1,0,1,5,5,5,5];
    let mut freq_vec:HashMap<i32,u32>=HashMap::new();

    for i in &some_vec{
        let freq: &mut u32 =freq_vec.entry(*i).or_insert(0);
        *freq+=1;
    }
}

