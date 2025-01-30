use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    let range_input: Vec<i32>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap() )
        .collect();
    
    let mut items: Vec<Vec<i32>>= Vec::new();

    for _ in 0..range_input[0] {
        let each_itme: Vec<i32>= buf.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap() )
        .collect();

        items.push(each_itme);
    }

    let mut max_number= 0;
    let mut amount_kg=0;
    for i in 0..items.len()-1{
        for j in 0..items[0].len(){
            let value= items[i][1] + items[i+1][1];
            let kg= items[i][0]+ items[i+1][0];
            if max_number > value {
                max_number= value;
            }
        }
    }

    println!("{}", max_number);
}
