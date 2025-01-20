use std::io::{stdin, BufRead};

fn main() {

    let std= stdin();
    let mut buf= std.lock().lines();
    let n= buf.next().unwrap().unwrap();
    let v: Vec<i32>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap() )
        .collect();
    let target= buf.next().unwrap().unwrap()
        .parse::<i32>().unwrap();


    let mut cnt=0;
    for e in v{
        if e == target{
            cnt+=1;
        }
    }

    println!("{}", cnt);
}