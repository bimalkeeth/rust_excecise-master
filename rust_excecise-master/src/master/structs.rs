#![allow(dead_code)]
#![allow(unused_variables)]

struct User {
    active: bool,
    name: String,
    sign_in_count: u32,
}

struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

//  function to implement struct lifetime .
pub fn struct_life_time() {
    let user1 = User {
        active: true,
        name: String::from("Tyler"),
        sign_in_count: 0,
    };

    let square = Square {
        width: 10,
        height: 20,
    };
    let area = square.area();

    println!("area of square {}", area)
}


pub fn struct_process() {
    let person= Person::new();

    println!("tax for person{} is {}", person.name, person.compute_tax())
}

struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32,
}

impl Person {

    fn new()->Self{
        return Person{
            age:23,
            citizenship:String::from("Sri Lankan"),
            salary:2000,
            name:String::from("Bimal Kaluarachchi"),
            gender:'M'
        }
    }

    fn compute_tax(&self) -> f32 {
        (self.salary as f32 / 3.) * 0.5
    }
}