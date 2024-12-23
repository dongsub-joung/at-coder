use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let input= buf.next().unwrap().unwrap()
        .parse::<i32>().unwrap();

    if input < 60{
        println!("F");
    }else if input >= 60 && input < 70{
        println!("D")
    }else if input >=70 && input < 80{
        println!("C")
    }else if input >= 80 && input < 90{
        println!("B")
    }else{
        println!("A");
    }
}
