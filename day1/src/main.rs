use std::collections::BinaryHeap;


fn part_one(){
    let input = std::fs::read_to_string("data/input.txt").unwrap();
    let lines = input.lines();
    let mut max = 0;
    let mut elf_sum = 0;
    for line in lines {
        if line == "" {
            if elf_sum > max {
                max = elf_sum;
            }
            elf_sum = 0;
        }
        else {
            let line_value = line.parse::<i32>().unwrap();
            elf_sum += line_value;
        }
    }
    println!("Elf carrying the max is carrying: {}.", max);
}

fn part_two(){
    let input = std::fs::read_to_string("data/input.txt").unwrap();
    let lines = input.lines();
    let mut elf_sum = 0;

    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    for line in lines{
        if line == "" {
            heap.push(elf_sum);
            elf_sum = 0;
        }
        else {
            let line_value = line.parse::<i32>().unwrap();
            elf_sum += line_value;
        }
    }
    let top_three = heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap();
    println!("Sum of top three elves' carriage is {top_three}.");
}


fn main() {
    part_one();
    part_two();
}
