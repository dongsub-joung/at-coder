use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let n= buf.next().unwrap().unwrap()
        .parse::<i32>().unwrap();

    for _ in  {
        
    }
}
