#![allow(dead_code)]
#![allow(unused_variables)]

struct Person {
    name: String,
}

struct Company<'l> {
    name: String,
    ceo: &'l Person,
}

pub fn memory_worker() {
    let boss = Person {
        name: String::from("Elon Mask"),
    };
    let tesla = Company {
        name: String::from("Tesla"),
        ceo: &boss,
    };
}
