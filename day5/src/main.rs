use std::collections::VecDeque;

fn solution(is_part_two: bool) {
    let mut stacks: Vec<VecDeque<char>>;
    let input = std::fs::read_to_string("data/input.txt").unwrap();

    let (left, instructions_str) = input.split_once("\n\n").unwrap();
    let (stacks_str, platforms) = left.rsplit_once('\n').unwrap();
    let num_stacks = platforms
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    // create num_stacks times a new VecDeque
    stacks = vec![VecDeque::new(); num_stacks];

    for line in stacks_str.to_string().lines() {
        let mut stack_idx = 0;
        let mut line_iter = line.chars().peekable();
        while line_iter.peek().is_some() {
            let chunk: String = line_iter.by_ref().take(3).collect();
            let chunk_char = chunk.chars().nth(1).unwrap();
            match chunk_char {
                ' ' => {}
                x => {
                    stacks[stack_idx].push_front(x);
                }
            }
            line_iter.next();
            stack_idx += 1;
        }
    }

    for line in instructions_str.lines() {
        let rest = line.strip_prefix("move ").unwrap();
        let (amount, rest) = rest.split_once(" from ").unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();
        let amount: usize = amount.parse().unwrap();
        let from: usize = from.parse::<usize>().unwrap() - 1;
        let to: usize = to.parse::<usize>().unwrap() - 1;

        if is_part_two {
            let mut popped_items: VecDeque<char> = VecDeque::new();
            for _ in 0..amount {
                let popped_item = stacks[from].pop_back().unwrap();
                popped_items.push_front(popped_item);
            }
            for item in popped_items {
                stacks[to].push_back(item);
            }
        } else {
            for _ in 0..amount {
                let popped_item = stacks[from].pop_back().unwrap();
                stacks[to].push_back(popped_item);
            }
        }
    }

    let mut result = String::new();

    for stack in &stacks {
        result.push(stack.back().unwrap().clone());
    }
    println!("{}", result);
    // dbg!(stacks);
}

fn main() {
    solution(false);
    solution(true);
}
