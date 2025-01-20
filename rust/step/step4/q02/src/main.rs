use std::{io::{stdin, BufRead}};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let inputs: Vec<i32>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap() )
        .collect();
    let v: Vec<i32>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap() )
        .collect();

    let n= inputs[0];
    let x= inputs[1];

    let mut result_v: Vec<i32>= Vec::new();
    
    for e in v {
        if x > e {
            result_v.push(e);
        }
    }

    println!("{:#?}", result_v);
}
