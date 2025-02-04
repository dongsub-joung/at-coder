// DP - https://github.com/TheAlgorithms/Rust/blob/master/src/dynamic_programming/coin_change.rs

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

    let mut item_values: Vec<Vec<Option<i32>>>= Vec::new();

    let n= range_input[0];
    let k= range_input[1];
    for i in 0..n {
        let mut base: Vec<Option<i32>> = vec![None; k as usize];
        item_values.push(base);
    }

    for k_idx in 1..k {
        for i in 1..n {
            item_values[i as usize][k as usize]= item_values[(i as usize)-1][k as usize];
            if k_idx - items[i as usize][0] >= 0 {
                let value1= items[i as usize][1];

                let value2= item_values[(i as usize) - 1][(k as usize) - (items[i as usize][0] as usize)];
                let unwrap_option= match value2{
                    Some(e) => e,
                    None => 0,
                };

                let result= value1 + unwrap_option;
                item_values[i as usize][k as usize]= item_values[(i as usize)-1][k as usize]
                    .max(Some(result));
            }
        }
    }

    println!("{:#?}", item_values[n as usize][k as usize]);
}
