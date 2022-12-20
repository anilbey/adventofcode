use std::collections::HashMap;

struct Node {
    name: String,
    size: i32,
    children: HashMap<&Node>,
    parent: Option<&Node>,
}

// implement hashable for Node



fn main() {

    let root = Node {
        name: "/".to_string(),
        size: 0,
        children: HashMap::new(),
    };

    let input = std::fs::read_to_string("data/input.txt").unwrap();

    println!("Hello, world!");
}
