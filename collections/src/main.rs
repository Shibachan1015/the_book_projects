// use std::fmt::Display;
use std::io::{self, BufReader};
//use std::io::prelude::*;

fn main() {
    let mut v = Vec::new();
    
    let mut input = BufReader::new;

    io::stdin()
        .read_line(&mut input)
        .expect_err("miss");

    let vnumber = vec![v];

    for v in vnumber.iter() {
        println!("{:?}", v);
    }

    println!("vnumber: {:?}",vnumber);
}
