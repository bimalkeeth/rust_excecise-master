#![allow(dead_code)]
#![allow(unused_variables)]
pub fn matches() {
    for (pos, y) in (30..41).enumerate() {
        println!("index :{} - value: {}", pos, y);
    }

    let country_code: i32 = 44;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        1..=1000 => "Unknown",
        _ => "Invalid",
    };

    println!("country {}", country);

    let code = String::from("1234");
}
