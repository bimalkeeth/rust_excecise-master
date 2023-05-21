#![allow(dead_code)]
#![allow(unused_variables)]

use async_std::{fs::File, io, prelude::*, task};

pub fn async_data() {
    let task =task::spawn(async{
        let result =read_file("error.txt").await;
        match result {
            Ok(k) => println!("{}",k),
            Err(e) => println!("{}",e)
        }
    });

    println!("task has started");

    task::block_on(task);

    println!("task stopped");


}

async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents =String::new();
    file.read_to_string(&mut contents).await?;

    Ok(contents)

}