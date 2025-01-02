use std::io::{stdin, BufRead};

// 87967284	ehdtjqwjd	2753	맞았습니다!!	13204	0	Rust 2021 / 수정	334	35초 전
fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let year= buf.next().unwrap().unwrap()
        .parse::<i32>().unwrap();

    if year%400 == 0 {
        println!("1");
    }else if year%4==0 && year%100 != 0 {
        println!("1");
    }else{
        println!("0");
    }
}
