use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let v: Vec<f64>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<f64>().unwrap() )
        .collect();

    println!("{}", v[0]/v[1]);
}
