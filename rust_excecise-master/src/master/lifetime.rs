#![allow(dead_code)]
#![allow(unused_variables)]

pub fn life_time() {
    let s = "Hello, world!";
    let sx = "!";

    let result = example(s, sx);

    println!("{}", result)
}

fn example<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
    let ds = format!("{}{}", x, y);
    let slice: &str = ds.as_str();

    return y;
} //'a for one parameter 'b for next parameter and so on

enum Pet {
    Dog,
    Cat,
    Fish,
}

impl Pet {
    fn what_am_i(self) -> &'static str {
        match self {
            Pet::Dog => "I am a dog",
            Pet::Cat => "I am a cat",
            Pet::Fish => "I am a fish",
        }
    }
}

pub fn option_enum() {
    let pet = Pet::Dog;
    println!("{}", pet.what_am_i());

    let five = Some(5);
    let six = plus_one(five);

    println!("{}", six.unwrap())
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
