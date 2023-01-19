#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you.";
    let stdin = io::stdin();
    stdin.lock().read_line(&mut name).expect("Didn't receive input");
    //io::stdin().read_line(buf: &mut name)
    //    .expect(msg: "Didn't receive input");

    println!("Hello {}! {}", name.trim_end(), greeting);
}
