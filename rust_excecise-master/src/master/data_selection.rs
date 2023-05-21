#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Deref;
use std::rc::Rc;
use List::Cons;

#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}

pub fn cons_list() {
    let list = List::Cons(1, Some(Box::new(Cons(2, Some(Box::new(Cons(3, None)))))));
    println!("{:?}", list)
}

pub fn box_smart() {
    let single_value = Box::new(0.625);
}

#[derive(Debug)]
struct MyBoxPtr {
    value: i32,
}

impl MyBoxPtr {
    fn new(x: i32) -> MyBoxPtr {
        MyBoxPtr { value: x }
    }
}

impl Deref for MyBoxPtr {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Drop for MyBoxPtr {
    fn drop(&mut self) {
        println!("dropping my box ptr from memory {:?}", self.value)
    }
}

pub fn process_mysmart() {
    let a = 50;
    let b = &a;

    let sptr1 = MyBoxPtr::new(a);
    let sptr2 = MyBoxPtr::new(*b);

    println!("{:?}", a == *sptr1);
}

//-----------------------singly link list --------------------------

#[derive(Debug)]
struct LinkList {
    head: Pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer,
}

type Pointer = Option<Box<Node>>;

impl LinkList {
    fn create_empty() -> LinkList {
        LinkList { head: None }
    }

    fn add(&mut self, elem: i32) {
        let previous_head = self.head.take();
        let new_head = Box::new(Node {
            element: elem,
            next: previous_head,
        });

        self.head = Some(new_head);
    }

    fn remove(&mut self) -> Option<i32> {
        let previous_head = self.head.take();
        match previous_head {
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.element)
            }
            None => None
        }
    }

    fn peek(&self) -> Option<i32> {
        match &self.head {
            Some(data) => Some(data.element),
            None => None
        }
    }

    fn print(&self) {
        let mut list_travel = &self.head;
        loop {
            match list_travel {
                Some(node) => {
                    println!("element value : {:?}", node.element);
                    list_travel = &node.next;
                }
                None => break,
            }
        }
    }
}

pub fn singly_link() {
    let list = LinkList { head: None };
    let list = LinkList {
        head: Some(Box::new(Node {
            element: 100,
            next: Some(Box::new(Node {
                element: 200,
                next: None,
            })),
        })),
    };

    println!("{:?}", list.head.unwrap().element);
}
//---------------------------Refcell/Rc combination --------------------

pub fn refcell_rc() {
    let a = Rc::new(RefCell::new(String::from("Jave")));
    let b = Rc::clone(&a);

    *b.borrow_mut() = String::from("c++");

    println!("{:?}", a)
}

//----------------------------Doubly link list ------------------------

type Pointerx<T> = Option<Rc<RefCell<Nodex<T>>>>; //pointer for rc/refcell

struct Nodex<T> {
    element: T,
    next: Pointerx<T>,
    prev: Pointerx<T>,
}

struct Listx<T> {
    head: Pointerx<T>,
    tail: Pointerx<T>,
}


impl<T: Display> Nodex<T> {
    fn new(elem: T) -> Rc<RefCell<Nodex<T>>> {
        Rc::new(RefCell::new(Nodex {
            element: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T: Display> Listx<T> {
    fn new() -> Self {
        Listx {
            tail: None,
            head: None,
        }
    }

    fn push_front(&mut self, elem: T) {
        let new_head = Nodex::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn push_back(&mut self, elem: T) {
        let new_tail= Nodex::new(elem);
        match self.tail.take() {
            Some(old_tail)=>{
                old_tail.borrow_mut().next=Some(new_tail.clone());
                new_tail.borrow_mut().prev=Some(old_tail.clone());

                self.tail=Some(new_tail);
            }
            None=> {
                self.head=Some(new_tail.clone());
                self.tail=Some(new_tail);
            }
        }

    }
}

pub fn doubly_link() {
    let list: Listx<i32> = Listx::new();
}






