#![allow(dead_code)]
#![allow(unused_variables)]



pub fn string_compound(){
    let some_string = "fixed length string";

    let mut numbers :[u8;4]= [1,2,3,4];
    {
        let all:&[u8]=&numbers[..];
        println!("All of them : {:?}",all)
    }
    {
        let first_two:&mut [u8]=&mut numbers[0..2];
        first_two[0]=100;
        first_two[1]=90;
    }

    println!("look ma ! i can modify through the slice: {:?}",numbers)
}