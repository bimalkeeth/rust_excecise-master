#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::{Error, ErrorKind};

pub fn panics() {
    let file = File::open("error.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("error.txt") {
                Ok(file_created) => file_created,
                Err(err) => panic!("cannot create file"),
            },

            _ => panic!("it was some other error"),
        },
    };

    //let vec =vec![1];
    //vec[10];
}

pub fn open_file() {
    let test = error_propagation();
    test.unwrap();
}

fn error_propagation() -> Result<File, Error> {
    let file = File::open("error.txt")?;
    Ok(file)
}
