use std::collections::HashSet;

fn char_to_score(char_val: char) ->usize {
    // a to z are 1 to 26
    // A to Z are 27 to 52
    match char_val {
        'a'..='z' => char_val as usize - 'a' as usize + 1,
        'A'..='Z' => char_val as usize - 'A' as usize + 27,
        _ => panic!("Unknown char"),
    }
}

fn part_one() {
    let input = std::fs::read_to_string("data/input.txt").unwrap();
    let lines = input.lines();
    let mut sum_scores = 0;
    for line in lines {
        let first_half = &line[0..line.len() / 2];
        // create hashset from first_half without a loop

        let hashset: HashSet<char> = HashSet::from(first_half.chars().collect::<HashSet<char>>());
        let second_half = &line[line.len() / 2..];
        // check if any character in second_half is in the hashset
        // create an empty char
        let mut intersection = ' ';
        for c in second_half.chars() {
            if hashset.contains(&c) {
                intersection = c;
            }
        }
        let score = char_to_score(intersection);
        sum_scores += score;
        println!("{}", intersection);
    }
    println!("Part one total score: {}", sum_scores);
}

fn part_two() {
    let input = std::fs::read_to_string("data/input.txt").unwrap();
    let lines = input.lines();
    let mut sum_scores = 0;
    // iterate lines get first 3 lines
    let mut line_iter = lines.peekable();
    while line_iter.peek().is_some() {
        let first_line = line_iter.next().unwrap();
        let second_line = line_iter.next().unwrap();
        let third_line = line_iter.next().unwrap();

        let first_hashset:HashSet<char> = HashSet::from(first_line.chars().collect::<HashSet<char>>());
        let second_hashset:HashSet<char> = HashSet::from(second_line.chars().collect::<HashSet<char>>());
        let third_hashset:HashSet<char> = HashSet::from(third_line.chars().collect::<HashSet<char>>());

        // get intersection of the three hashsets
        let mut intersection = ' ';
        for c in first_hashset {
            if second_hashset.contains(&c) && third_hashset.contains(&c) {
                intersection = c;
            }
        }
        let score = char_to_score(intersection);
        sum_scores += score;
    }
    println!("Part two total score: {}", sum_scores);
}

fn main() {
    println!("Hello, world!");
    part_one();
    part_two();
}
