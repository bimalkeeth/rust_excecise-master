#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;

pub fn reference_counter() {
    let s1 = Rc::new(String::from("Pointer"));
    let s2 = s1.clone();
    let s3 = s2.clone();

    println!("{} {} {}", s1, s2, s3);

    let flg = Flagger {
        is_true: Rc::new(RefCell::new(true)),
    };

    let reference = Rc::new(flg.is_true.clone());
    println!("{:?}", reference);

    let mut reference2 = reference.borrow_mut();

    *reference2 = false;

    println!("{}", reference2);
}

struct Flagger {
    is_true: Rc<RefCell<bool>>,
}
