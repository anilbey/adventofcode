
use std::collections::{HashSet, VecDeque};

fn all_different(vector: &VecDeque<char>) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    for i in vector {
        set.insert(*i);
    }
    set.len() == vector.len()
}

fn solution(distinct_char_size: i32) -> i32 {
    let input = std::fs::read_to_string("data/input.txt").unwrap();
    let mut input_iter = input.chars().into_iter().peekable();

    let mut chunk_of_four: VecDeque<char> = VecDeque::new();
    
    for _ in 0..distinct_char_size {
        chunk_of_four.push_back(input_iter.next().unwrap());
    }
    let mut count = distinct_char_size;
    if all_different(&chunk_of_four) {
        return count;
    }
    else {
        while input_iter.peek().is_some() {
            let _ = chunk_of_four.pop_front();
            chunk_of_four.push_back(input_iter.next().unwrap());
            count += 1;
            if all_different(&chunk_of_four) {
                return count;
            }
        }
        return -1;
    }

}

fn main() {
    println!("{}", solution(4));
    println!("{}", solution(14));
    println!("Hello, world!");
}
