#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

pub fn life_time_specifier() {
    let s_1: &str = "Hello";
    let v: &str;
    {
        let s_2: String = String::from("World");
        v = some_func(s_1, s_2.as_str())
    }
}


fn some_func<'a, 'b>(first_str: &'a str, second_str: &'b str) -> &'a str {
    first_str
}


pub fn closures() {
    let mut vec_1: Vec<i32> = vec![1, 2, 3];
    vec_1[0] = 10;

    let mut some_closure = || {
        let vec_2: &mut Vec<i32> = &mut vec_1;
        vec_2[0] = 11;
    };

    some_closure();

    println!("Vec_1 = {:?}", vec_1);
    //println!("Vec_2 = {:?}",vec_2);
}


pub fn iterators(){
    let a :Vec<i32>=vec![0,1,2,3,4,5,6,7,8,9];

    let filtered_values=a.iter().filter(|&x|{ return *x >= 5}).collect::<Vec<&i32>>();

    println!("collection value {:?}",filtered_values);

    println!("{:?}",a);

    let b =a.clone();

    let filtered_val =b.into_iter().filter(|&x|{
        x>7
    }).collect::<Vec<i32>>();

    println!("collection value {:?}",filtered_val);
}