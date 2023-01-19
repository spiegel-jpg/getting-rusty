#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    const ONE_MIL: u32 = 1_000_000;  
    const PI: f32 = 3.141592; // constant variable doesn't change value and is always declared in captial letters
    let age: &str = "47";
    let mut age: u32 = age.trim().parse() //use parse() to convert string to integer in rust
        .expect("Age wasn't assigned a number.");
    age = age +1;
    println!("I am  {} and I want ${}", age, ONE_MIL);
}