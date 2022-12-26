use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
struct TreeNode {
    name: String,
    children_dirs: BTreeMap<String, Rc<RefCell<TreeNode>>>,
    files: Vec<usize>,
    parent_dir: Option<Weak<RefCell<TreeNode>>>,
    size: usize,
}

impl TreeNode {
    fn get_child_dir(&self, dir_name: &str) -> Option<Rc<RefCell<TreeNode>>> {
        match self.children_dirs.get(dir_name) {
            Some(dir) => Some(Rc::clone(dir)),
            None => None,
        }
    }

    fn calculate_size(&mut self) {
        let mut size = 0;
        for file in &self.files {
            size += file;
        }
        for child_dir in self.children_dirs.values() {
            child_dir.borrow_mut().calculate_size();
            size += child_dir.borrow().size;
        }
        self.size = size;
    }

    fn get_all_sizes_less_than(&self, limit: usize) -> Vec<usize> {
        let mut sizes = Vec::new();
        if self.size <= limit {
            sizes.push(self.size);
        }
        for child_dir in self.children_dirs.values() {
            sizes.append(&mut child_dir.borrow().get_all_sizes_less_than(limit));
        }
        sizes
    }

    fn get_all_sizes_greater_than(&self, limit: usize) -> Vec<usize> {
        let mut sizes = Vec::new();
        if self.size >= limit {
            sizes.push(self.size);
        }
        for child_dir in self.children_dirs.values() {
            sizes.append(&mut child_dir.borrow().get_all_sizes_greater_than(limit));
        }
        sizes
    }
}

fn build_tree() -> Rc<RefCell<TreeNode>> {
    let root = Rc::new(RefCell::new(TreeNode {
        name: "/".to_string(),
        children_dirs: BTreeMap::new(),
        files: Vec::new(),
        parent_dir: None,
        size: 0,
    }));

    let mut current_node = Rc::clone(&root);

    let input = std::fs::read_to_string("data/input.txt").unwrap();

    let lines = input.lines();
    let mut line_iter = lines.peekable();

    _ = line_iter.next().unwrap(); // skip first line

    while line_iter.peek().is_some() {
        let line = line_iter.next().unwrap();
        if line.starts_with("$ ls") {
        } else if line.starts_with("dir ") {
            let dir_name = &line[4..];
            let new_dir = Rc::new(RefCell::new(TreeNode {
                name: dir_name.to_string(),
                children_dirs: BTreeMap::new(),
                files: Vec::new(),
                parent_dir: Some(Weak::new()),
                size: 0,
            }));
            // set new_dir's parent to be current_node
            new_dir.borrow_mut().parent_dir = Some(Rc::downgrade(&current_node));
            // add new_dir to current_node's children_dirs
            current_node
                .borrow_mut()
                .children_dirs
                .insert(dir_name.to_owned(), Rc::clone(&new_dir));
        } else if line.chars().nth(0).unwrap().is_numeric() {
            let file_size: usize = line.split_whitespace().next().unwrap().parse().unwrap();
            current_node.borrow_mut().files.push(file_size);
        } else if line.starts_with("$ cd") {
            let dir_name = line.split_whitespace().last().unwrap();
            match dir_name {
                ".." => {
                    let parent_dir = current_node
                        .borrow()
                        .parent_dir
                        .as_ref()
                        .unwrap()
                        .upgrade()
                        .unwrap();
                    current_node = parent_dir;
                }
                _ => {
                    let child_dir = current_node.borrow().get_child_dir(dir_name).unwrap();
                    current_node = child_dir;
                }
            }
        } else {
            panic!("Unexpected line");
        }
    }
    root.borrow_mut().calculate_size();
    root
}

fn main() {
    let root = build_tree();
    dbg!(&root);

    // part 1
    let sizes_lt_100k = root.borrow().get_all_sizes_less_than(100000);
    dbg!(&sizes_lt_100k);
    dbg!(sizes_lt_100k.iter().sum::<usize>());

    // part 2
    let space_left = 70000000 - root.borrow().size;
    let min_amount_to_be_freed = 30000000 - space_left;
    let candidates_to_remove = root
        .borrow()
        .get_all_sizes_greater_than(min_amount_to_be_freed);
    dbg!(&candidates_to_remove);
    dbg!(candidates_to_remove.iter().min());
}
