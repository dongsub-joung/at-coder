use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let input_v: Vec<i32>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap())
        .collect();

    if input_v[0] == input_v[1]{
        println!("==");
    }else{

        let which_max= input_v[0].max(input_v[1]);
        if which_max == input_v[0]{
            println!(">");
        }else{
            println!("<");
        }
    }
}
