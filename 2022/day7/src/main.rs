use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::{Hash, Hasher},
    io,
};

struct File {
    name: String,
    size: usize,
}

impl Hash for File {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for File {}

struct Directory {
    name: String,
    children: HashSet<Element>,
}

impl Hash for Directory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Directory {}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name: name,
            children: HashSet::new(),
        }
    }

    fn get_child(self, name: String) -> Option<&'static Element> {
        self.children.get(&Element::Directory(Directory::new(name)))
    }
}

#[derive(Eq, Hash, PartialEq)]
enum Element {
    File(File),
    Directory(Directory),
}

fn main() {
    let input: Vec<_> = io::stdin()
        .lines()
        .map(|line_res| line_res.unwrap())
        .skip(2) // Don't care about initial cd and ls
        .collect();

    let mut fs_tree = Directory::new("/".to_string());
    let mut cur_dir_stack = vec![&fs_tree];

    for line in input {
        let cur_dir = cur_dir_stack.last_mut().unwrap();

        // Directory
        if line.starts_with("dir") {
            let dir_name = line.split(' ').last().unwrap();
            cur_dir
                .children
                .insert(Element::Directory(Directory::new(dir_name.to_string())));
        }
        // Change directory
        else if line.starts_with("$ cd") {
            let dest_name = line.split(' ').last().unwrap();
            if let Element::Directory(dest) = cur_dir.get_child(dest_name.to_string()).unwrap() {
                cur_dir_stack.push(dest);
            }
        }
        // We don't care about ls
        else if line == "$ ls" {
        }
        // File
        else {
            let (size_str, name) = line.split_once(' ').unwrap();
            cur_dir.children.insert(Element::File(File {
                name: name.to_string(),
                size: size_str.parse().unwrap(),
            }));
        }
    }

    println!("hi");
}
