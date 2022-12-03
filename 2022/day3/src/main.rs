use std::{
    collections::{HashMap, HashSet},
    io,
    str::Chars,
};

use itertools::Itertools;

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

trait SingleOccurance {
    fn single_occurance(self) -> Vec<char>;
}

impl SingleOccurance for Chars<'_> {
    fn single_occurance(self) -> Vec<char> {
        let mut occurances: HashMap<char, usize> = HashMap::new();

        for char in self {
            if occurances.contains_key(&char) {
                let value = occurances.get(&char).unwrap();
                occurances.insert(char, value + 1);
            } else {
                occurances.insert(char, 1);
            }
        }

        occurances
            .iter()
            .filter(|character| *character.1 == 1)
            .map(|tuple| *tuple.0)
            .collect()
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

    let priority_sum_part_2: u32 = input
        .chunks_exact(3)
        .map(|chunk| chunk.join(""))
        .map(|combined_chunk| {
            combined_chunk
                .chars()
                .single_occurance()
                .iter()
                .map(|char| char.priority() as u32)
                .sum::<u32>()
        })
        .sum();

    println!("{}", priority_sum_part_1);
    println!("{}", priority_sum_part_2);
}
