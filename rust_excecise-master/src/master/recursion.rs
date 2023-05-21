#![allow(dead_code)]
#![allow(unused_variables)]

pub fn selection_sort(){
    println!("{}",silly_sub(10,12))
}

fn silly_sub(a:i32,b:i32)->i32{
    let mut result = 0;
    'increment: loop {
        if result ==a{
            let mut dec = b;
            loop{
                if dec ==0{
                    break 'increment;
                }else{
                    result -=1;
                    dec -=1
                }
            }
        }else{
            result+=1
        }

    }
    result
}