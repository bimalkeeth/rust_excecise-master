#![allow(dead_code)]
#![allow(unused_variables)]

pub fn owner_ship() {
    let x = vec!["tyler".to_string()];
    let y = x.clone();

    println!("{:?}", x);

    let s = String::from("takes"); //giving ownership
    take_ownership(s);

    let result = take_give("data".to_string());

    println!("{}", result)
}

fn take_ownership(s: String) {
    let strln = s;
    println!("{}", strln);
}

fn take_give(str: String) -> String {
    str
}
