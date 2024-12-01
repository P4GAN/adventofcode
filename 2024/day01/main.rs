use std::fs;
use std::collections::HashMap;

fn partOne() -> i32 {
    let filename = "input.txt";
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();
    
    for line in fs::read_to_string(filename).unwrap().lines() {
        let string = line.to_string();
        let (a, b) = string.split_once(char::is_whitespace).unwrap();
        left.push(a.trim().parse().unwrap());
        right.push(b.trim().parse().unwrap());
    }

    left.sort();
    right.sort();

    let sum: i32 = left.iter().zip(right.iter()).map(|(x, y)| (x-y).abs()).sum();

    sum
}

fn partTwo() -> i32 {
    let filename = "input.txt";
    let mut left = Vec::<i32>::new();
    let mut right = HashMap::<i32, i32>::new();
    
    for line in fs::read_to_string(filename).unwrap().lines() {
        let string = line.to_string();
        let (a, b) = string.split_once(char::is_whitespace).unwrap();
        left.push(a.trim().parse().unwrap());
        *right.entry(b.trim().parse().unwrap()).or_insert(0) += 1;
    }

    let sum: i32 = left.iter().map(|x| *right.entry(*x).or_default() * x).sum();

    sum
}


fn main() {
    println!("{}", partOne());
    println!("{}", partTwo());
}