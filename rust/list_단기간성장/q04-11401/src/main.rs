use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buff= std.lock().lines();
    let v: Vec<i32>= buff.next().unwrap().unwrap()   
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap())
        .collect();
    
    let n= v[0];
    let k= v[1];
    let p = 1000000007;

    //factorial
}
