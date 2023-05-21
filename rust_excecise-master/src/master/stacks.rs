#![allow(dead_code)]
#![allow(unused_variables)]

pub fn stack_operation() {}

fn new_stack(maxsize: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(maxsize);

    return vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let popped_val: Option<u32> = stack.pop();
    println!("the popped value is {:?}", popped_val);

    return popped_val;
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) {
    if stack.len()==maxsize{
        return;
    }

    stack.push(item);
}

//----------------------------------------------------------

