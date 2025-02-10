use std::{io::{stdin, BufRead}, str::Chars};

fn main() {
    let std= stdin();
    let mut buff= std.lock().lines();
    let input_v: Vec<i32>= buff.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap() )
        .collect();

    let a= input_v[0].to_string();
    let b= input_v[1].to_string();
    
    let mut a_v: Vec<char>= Vec::new();
    let mut b_v: Vec<char>= Vec::new();
    for e in a.chars().rev() {
        a_v.push(e)
    }

    for e in b.chars().rev() {
        b_v.push(e)
    }

    let a_str: String= a_v.iter().collect();
    let b_str: String= b_v.iter().collect();

    let a_n: i32= a_str.parse().unwrap();
    let b_n: i32= b_str.parse().unwrap();

    if a_n > b_n {
        println!("{}", a_n);       
    }else{
        println!("{}", b_n);
    }
}
