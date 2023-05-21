#![allow(dead_code)]
#![allow(unused_variables)]

pub fn formatting() {
    let name = "Dimithri";
    let greetings = format!("hi , I am {} nice to mee you!", name);

    println!("{}", greetings)
}

pub fn ret_params() -> (i32, i32) {
    return (10, 11);
}
