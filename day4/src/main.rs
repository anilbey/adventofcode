struct Elf {
    min: i32,
    max: i32,
}

impl Elf {
    fn new(init_str: &str) -> Elf {
        let mut split_str = init_str.split("-");
        let min = split_str.next().unwrap().parse::<i32>().unwrap();
        let max = split_str.next().unwrap().parse::<i32>().unwrap();
        Elf { min, max }
    }

    fn contains(&self, other_elf: &Elf) -> bool {
        self.min <= other_elf.min && self.max >= other_elf.max
    }
    fn contains_point(&self, point: i32) -> bool {
        self.min <= point && self.max >= point
    }

    fn overlaps(&self, other_elf: &Elf) -> bool {
        self.contains_point(other_elf.min) || self.contains_point(other_elf.max)
    }
}

fn both_parts() {
    let input = std::fs::read_to_string("data/input.txt").unwrap();
    let lines = input.lines();
    let mut n_reconsiderations = 0;
    let mut n_overlaps = 0;
    for line in lines {
        let mut split_line = line.split(",");
        let first_elf = split_line.next().unwrap();
        let second_elf = split_line.next().unwrap();
        let first_elf = Elf::new(first_elf);
        let second_elf = Elf::new(second_elf);
        if first_elf.contains(&second_elf) || second_elf.contains(&first_elf) {
            n_reconsiderations += 1;
        }
        if first_elf.overlaps(&second_elf) || second_elf.overlaps(&first_elf) {
            n_overlaps += 1;
        }
    }
    println!("Part one: {}", n_reconsiderations);
    println!("Part two: {}", n_overlaps);
}

fn main() {
    println!("Hello, world!");
    both_parts();
}
