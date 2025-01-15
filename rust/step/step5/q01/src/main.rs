use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let str= buf.next().unwrap().unwrap();
    let n= buf.next().unwrap().unwrap()
        .parse::<i32>().unwrap();

    let chars: Vec<char>= str.chars().collect();
    
    println!("{}", chars[(n-1) as usize]);
}
