use std::{collections::HashSet, io};

fn duplicates(halves: (&str, &str)) -> HashSet<char> {
    let mut duplicates = HashSet::new();

    for left_char in halves.0.chars() {
        for right_char in halves.1.chars() {
            if left_char == right_char {
                duplicates.insert(left_char);
            }
        }
    }

    duplicates
}

trait Priority {
    fn priority(self) -> u8;
}

impl Priority for char {
    fn priority(self) -> u8 {
        // In a proper piece of software you'd have an assert here to ensure
        // that the character is ascii, but that would slow us down ;)
        if self.is_ascii_lowercase() {
            return self as u8 - 96;
        }

        self as u8 - 38
    }
}

fn main() {
    let input: Vec<_> = io::stdin().lines().map(|line| line.unwrap()).collect();

    let priority_sum_part_1: u32 = input
        .iter()
        .map(|line| {
            let halves = line.split_at(line.chars().count() / 2);
            duplicates(halves)
                .iter()
                .map(|y| y.priority() as u32)
                .sum::<u32>()
        })
        .sum();

    let priority_sum_part_2: u32 = input.chunks_exact(3).map(|chunk| {});

    println!("{}", priority_sum_part_1);
}
