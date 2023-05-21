#![allow(dead_code)]
#![allow(unused_variables)]

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T>{
    head:Link<T>
}

struct  Node<T>{
    elem: T,
    next:Link<T>,
}

impl<T>List<T>{
    pub fn new() ->Self{
        List{head:None}
    }

    pub fn push(&mut self,elems:T){
       let new_node =Box::new(Node{
           elem:elems,
           next:self.head.take()
       });

       self.head=Some(new_node)
    }

    pub fn pop(&mut self)->Option<T>{
        self.head.take().map(|node|{
            self.head=node.next;
            node.elem
        })
    }
}


impl<T> Drop for List<T>{
    fn drop(&mut self) {
        let mut link=self.head.take();
        while let Some(mut boxed_node)=link{
            link=boxed_node.next.take()
        }
    }
}