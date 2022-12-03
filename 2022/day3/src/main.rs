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
    let priority_sum = io::stdin()
        .lines()
        .map(|x| {
            let line = x.unwrap();
            let halves = line.split_at(line.chars().count() / 2);
            duplicates(halves)
                .iter()
                .map(|y| y.priority() as u32)
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("{}", priority_sum);
}
