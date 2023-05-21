#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;

struct Person {
    name: Rc<String>,
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        Person { name }
    }

    fn greetings(&self) {
        println!("Hi my name is {}", self.name)
    }
}

pub fn rc_show_name() {
    let name = Rc::new("john".to_string());
    let person = Person::new(name.clone()); // clone increment reference count

    person.greetings()
}
