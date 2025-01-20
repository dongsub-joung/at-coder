use std::{io::{stdin, BufRead}, iter::Rev};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();

    let n = buf.next().unwrap().unwrap()
        .parse::<i32>().unwrap();

    let v: Vec<i32>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap() )
        .collect();

    let min= v.iter().min().unwrap();
    let max= v.iter().max().unwrap();

    println!("{} {}", min, max);
}
