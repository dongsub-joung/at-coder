use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let v: Vec<i64>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<i64>().unwrap() )
        .collect();

    println!("{}", v[0]+v[1]);
    println!("{}", v[0]-v[1]);
    println!("{}", v[0]*v[1]);
    println!("{}", v[0]/v[1]);
    println!("{}", v[0]%v[1]);
}
