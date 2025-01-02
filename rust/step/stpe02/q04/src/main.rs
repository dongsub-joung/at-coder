use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let x= buf.next().unwrap().unwrap()
        .parse::<i32>().unwrap();
    let y= buf.next().unwrap().unwrap()
        .parse::<i32>().unwrap();


    let x_condition: bool= if x > 0{
        true
    }else{
        false
    };

    let y_condition: bool= 
        if y > 0{
            true
        }else{
            false
        };

    if x_condition && y_condition{
        println!("1");
    }else if !x_condition && !y_condition{
        println!("3");
    }else if x_condition && !y_condition {
        println!("4");
    }else{
        println!("2");
    }
}
