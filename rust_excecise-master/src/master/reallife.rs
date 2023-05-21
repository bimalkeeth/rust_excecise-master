#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

struct MaxStack {
    main_stack: Vec<i32>,
    maximum_stack: Vec<i32>,
}

impl MaxStack {
    fn new()->Self{
        MaxStack{
            maximum_stack:Vec::new(),
            main_stack:Vec::new(),
        }
    }

    fn push(&mut self,val:i32){
        self.main_stack.push(val);
        if !self.maximum_stack.is_empty() &&
            self.maximum_stack.last().unwrap() >&val{
            self.maximum_stack.push(*self.maximum_stack.last().unwrap())
        }else {
            self.maximum_stack.push(val)
        }
    }
}