use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let input= buf.next().unwrap().unwrap()
        .parse::<i32>().unwrap();

    let gap= 543;
    println!("{}", input - gap);
}