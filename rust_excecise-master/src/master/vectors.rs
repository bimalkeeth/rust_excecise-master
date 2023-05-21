#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::BinaryHeap;

pub fn vectors_example() {
    let mut nums: Vec<i32> = vec![];

    nums.push(10);
    nums.push(20);
    nums.push(30);

    let pop = nums.pop(); // return Option<T>,return None or Some(T)
}

pub fn binary_heap() {
    let mut heap = BinaryHeap::new();
    heap.push(1)
}
