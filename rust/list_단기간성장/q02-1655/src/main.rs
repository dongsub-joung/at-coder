use std::{cmp::Reverse, collections::BinaryHeap, io::{stdin, BufRead}};

fn main() {
    let std= stdin();
    let mut buff= std.lock().lines();
    let n= buff.next().unwrap().unwrap()
        .parse::<i32>().unwrap();
    
    let mut max_heap = BinaryHeap::new();
    let mut min_heap = BinaryHeap::new();
    for i in 0..n {
        let number= buff.next().unwrap().unwrap()
            .parse::<i32>().unwrap();
        if max_heap.is_empty() || number <= *max_heap.peek().unwrap() {
            max_heap.push(number);
        }else {
            min_heap.push(Reverse(number));
        }
        
        if max_heap.len() > min_heap.len() +1  {
            min_heap.push(HeapPoll::poll(max_heap));  
        }else if min_heap.len()>max_heap.len() {
            max_heap.push(HeapPoll::poll(min_heap));  
        }
    }

    println!("{}", max_heap.peek().unwrap());
}

struct HeapPoll<T> {
    heap: T,
}
impl<T> HeapPoll<T> {   
    fn poll(heap: T) -> i32{
        let mut result_heap = BinaryHeap::new();
        result_heap.

        0
    }
}
