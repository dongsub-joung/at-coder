use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let n= 30;

    let mut v: Vec<i32>= Vec::new();
    for i in 1..n  {
        v.push(i);
    }

    let mut numbers: Vec<i32>= Vec::new();
    for _ in 0..28{
        let e= buf.next().unwrap().unwrap()
            .parse::<i32>().unwrap();
        numbers.push(e);
    }
    
    let mut result_v: Vec<i32>= Vec::new();
    for i in 28..0 {
        v.sort();

        if numbers[i] == *v.last().unwrap() {
            v.pop();
        }else{
            result_v.push(v.pop().expect("REASON"));
        }
    }

    result_v.sort();
    
    for e in result_v {
        println!("{}", e);
    }
}