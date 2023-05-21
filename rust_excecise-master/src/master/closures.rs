#![allow(dead_code)]
#![allow(unused_variables)]

//Fn FnMute FnOnce
#[derive(Debug)]
struct Item {
    name: String,
}

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Iterator for Range {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let result = Some(self.start);
        self.start += 1;

        result
    }
}

pub fn show_iterator() {
    let mut vec: Vec<Item> = Vec::new();

    vec.push(Item {
        name: String::from("coat"),
    });
    vec.push(Item {
        name: String::from("shirt"),
    });
    vec.push(Item {
        name: String::from("shorts"),
    });
    vec.push(Item {
        name: String::from("shoes"),
    });

    let range = Range { start: 0, end: 10 };

    for r in range {
        println!("{}", r)
    }

    let vec2: Vec<i16> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result: Vec<i16> = vec2.iter().map(|x| x * 10).collect();

    println!("{:?}", vec2);
    println!("{:?}", result);
}
