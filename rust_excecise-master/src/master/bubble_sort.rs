#![allow(dead_code)]
#![allow(unused_variables)]

pub fn bubble() {
    let mut vec = vec![5, 2, 1, 6, 7, 8, 9];
    let data=bubble_sort(&mut vec );

    println!("{:?}",data)
}


fn bubble_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    for _ in 1..=arr.len() - 1 {
        for j in 0..=arr.len()-2{
            if arr[j]>arr[j+1]{
                arr.swap(j,j+1)
            }
        }
    }

    arr.to_vec()
}