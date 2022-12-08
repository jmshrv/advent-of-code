use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io,
    path::Path,
};

use walkdir::WalkDir;

fn main() {
    let input: Vec<_> = io::stdin()
        .lines()
        .map(|line_res| line_res.unwrap())
        .skip(2) // Don't care about initial cd and ls
        .collect();

    let start_dir = env::current_dir().unwrap();

    for line in input {
        if line.starts_with("dir") {
            let dir_name = line.split(' ').last().unwrap();
            fs::create_dir(dir_name).unwrap();
        } else if line.starts_with("$ cd") {
            let dest_name = line.split(' ').last().unwrap();
            env::set_current_dir(dest_name).unwrap();
        } else if line == "$ ls" {
        } else {
            let (size_str, name) = line.split_once(' ').unwrap();
            let name_fmt = format!("{}-{}", name, size_str);
            File::create(name_fmt).unwrap();
        }
    }

    let mut directories: HashMap<String, usize> = HashMap::new();

    for entry_res in WalkDir::new(start_dir) {
        let entry = entry_res.unwrap();
        println!("{}", entry.path().to_str().unwrap());

        if entry.file_type().is_file() {
            let name = entry.file_name();
            let path = entry.path().parent().unwrap().to_str().unwrap().to_string();

            let size: usize = name
                .to_str()
                .unwrap()
                .split("-")
                .last()
                .unwrap()
                .parse()
                .unwrap();

            directories
                .entry(path)
                .and_modify(|val| *val += size)
                .or_insert(size);

            // let mut parent = entry.path().parent().unwrap();
            // while let Some(new_parent) = parent.parent() {
            //     parent = new_parent;
            //     let parent_path = parent.to_str().unwrap().to_string();

            //     directories
            //         .entry(parent_path)
            //         .and_modify(|val| *val += size);
            // }
        }
    }

    for directory in directories.keys() {
        let size = directories.get(directory).unwrap();
        let path = Path::new(&directory);

        let mut parent = path.parent().unwrap();
        while let Some(new_parent) = parent.parent() {
            parent = new_parent;
            let parent_path = parent.to_str().unwrap().to_string();

            directories
                .entry(parent_path)
                .and_modify(|val| *val += size);
        }
    }

    let answer_1: usize = directories
        .iter()
        .map(|x| x.1)
        .filter(|x| x <= &&100000)
        .sum();

    println!(
        "{:?}",
        directories.iter().map(|x| x.1).filter(|x| x <= &&100000)
    );
    println!("{}", answer_1);
}
