#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_macros)]


use std::mem;
macro_rules! our_macro {
  ()=>{

  }
}


pub fn partial_move() {
    let some_option = Some("Alice".to_string());
    match &some_option {
        Some(inner_value) => println!("name is {}", inner_value),
        None => println!("no name provided")
    }

    let some_option2 = Some("Alice".to_string());

    let some_1 = &some_option2;
    let some_2 = some_option2.as_ref();


    println!("{:?}", some_option)
}

//------------------------------------------------
#[derive(Debug)]
enum Customer {
    New { name: String },
    Loyal { name: String },
    Rich { name: String },
}

pub fn avoid_allocation() {
    let mut customer_1 = Customer::New { name: "michel".to_string() };
    promote(&mut customer_1);

    println!("customer_1 {:?}", customer_1);

    promote(&mut customer_1);
    println!("customer_1 {:?}", customer_1);
}

fn promote(user: &mut Customer) {
    *user = match user {
        Customer::New { name } => Customer::Loyal { name: mem::take(name) },
        Customer::Loyal { name } => Customer::Rich { name: mem::take(name) },
        Customer::Rich { name } => return,
    }
}