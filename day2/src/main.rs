
enum OpponentMove {
    A,
    B,
    C,
}

enum YourMove {
    X,
    Y,
    Z,
}

fn calculate_score_by_comparison(opponent: &OpponentMove, you: &YourMove) -> usize {
    let win = 6;
    let draw = 3;
    let loss = 0;
    match (opponent, you) {
        (OpponentMove::A, YourMove::X) => draw,
        (OpponentMove::A, YourMove::Y) => win,
        (OpponentMove::A, YourMove::Z) => loss,
        (OpponentMove::B, YourMove::X) => loss,
        (OpponentMove::B, YourMove::Y) => draw,
        (OpponentMove::B, YourMove::Z) => win,
        (OpponentMove::C, YourMove::X) => win,
        (OpponentMove::C, YourMove::Y) => loss,
        (OpponentMove::C, YourMove::Z) => draw,
    }
}

fn calculate_score_by_your_move(you: &YourMove) -> usize {
    match you {
        YourMove::X => 1,
        YourMove::Y => 2,
        YourMove::Z => 3,
    }
}

fn part_one() {
    let input = std::fs::read_to_string("data/input.txt").unwrap();
    let lines = input.lines();
    let mut total_score = 0;
    for line in lines {
        let mut parts = line.split_whitespace();

        let opponent = parts.next().unwrap();
        let you = parts.next().unwrap();

        let opponent = match opponent {
            "A" => OpponentMove::A,
            "B" => OpponentMove::B,
            "C" => OpponentMove::C,
            _ => panic!("Unknown opponent move"),
        };
        let you = match you {
            "X" => YourMove::X,
            "Y" => YourMove::Y,
            "Z" => YourMove::Z,
            _ => panic!("Unknown your move"),
        };
        // calculate score
        total_score += calculate_score_by_comparison(&opponent, &you);
        total_score += calculate_score_by_your_move(&you);
    }
    println!("Part one total score: {}", total_score);
}

fn determine_your_move(opponent: &OpponentMove, hint: &YourMove) -> YourMove {
    match (opponent, hint) {
        (OpponentMove::A, YourMove::X) => YourMove::Z,
        (OpponentMove::A, YourMove::Y) => YourMove::X,
        (OpponentMove::A, YourMove::Z) => YourMove::Y,
        (OpponentMove::B, YourMove::X) => YourMove::X,
        (OpponentMove::B, YourMove::Y) => YourMove::Y,
        (OpponentMove::B, YourMove::Z) => YourMove::Z,
        (OpponentMove::C, YourMove::X) => YourMove::Y,
        (OpponentMove::C, YourMove::Y) => YourMove::Z,
        (OpponentMove::C, YourMove::Z) => YourMove::X,
    }
}

fn part_two() {
    let input = std::fs::read_to_string("data/input.txt").unwrap();
    let lines = input.lines();
    let mut total_score = 0;
    for line in lines {
        let mut parts = line.split_whitespace();

        let opponent = parts.next().unwrap();
        let you = parts.next().unwrap();

        let opponent = match opponent {
            "A" => OpponentMove::A,
            "B" => OpponentMove::B,
            "C" => OpponentMove::C,
            _ => panic!("Unknown opponent move"),
        };

        let hint = match you {
            "X" => YourMove::X,
            "Y" => YourMove::Y,
            "Z" => YourMove::Z,
            _ => panic!("Unknown your move"),
        };
        let your_move = determine_your_move(&opponent, &hint);
        // calculate score
        total_score += calculate_score_by_comparison(&opponent, &your_move);
        total_score += calculate_score_by_your_move(&your_move);
    }
    println!("Part two total score: {}", total_score);
}


fn main() {

    part_one();
    part_two();

}
