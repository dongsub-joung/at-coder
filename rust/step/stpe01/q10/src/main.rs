use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let a= buf.next().unwrap().unwrap()
        .parse::<i32>().unwrap();
    let b= buf.next().unwrap().unwrap()
        .parse::<i32>().unwrap();

    println!("{}", a * (b%10));
    println!("{}", a * ((b%100)/10));
    println!("{}", a * (b/100));
    println!("{}", a * b);
}
