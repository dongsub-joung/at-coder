use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let input_v: Vec<i32>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap())
        .collect();

    let a= input_v[0];
    let b= input_v[1];
    let c= input_v[2];

    println!("{}", (a+b)%c );
    println!("{}", ((a%c)+(b%c))%c);
    println!("{}", (a*b)%c );
    println!("{}", ((a%c)*(b%c))%c);
}