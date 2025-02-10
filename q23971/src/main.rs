use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buff= std.lock().lines();
    let input_v: Vec<f32>= buff.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<f32>().unwrap() )
        .collect();

    let result_h= (input_v[0] / (input_v[2] + 1.0)).ceil();
    let result_w= (input_v[1] / (input_v[3] + 1.0)).ceil();

    println!("{:}", (result_h * result_w));
}
