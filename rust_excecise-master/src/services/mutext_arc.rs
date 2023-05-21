#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::{Arc, Mutex};
use std::thread;

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>,
}

impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
        Person { name, state }
    }

    fn greet(&self) {
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("exited");

        println!("Hi my name is {} and I am {}", self.name, state.as_str());
    }
}

pub fn mutext_demo() {
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("board".to_string()));
    let person = Person::new(name.clone(), state.clone());

    let t = thread::spawn(move || person.greet());

    println!(
        "name = {}, state = {}",
        name,
        state.lock().unwrap().as_str()
    );
}
