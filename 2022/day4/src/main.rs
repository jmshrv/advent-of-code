#![warn(clippy::pedantic)]

use std::{
    cmp::{max, min},
    io,
    ops::Range,
};

use range_ext::intersect::Intersect;

fn range_from_string(input: &str) -> Range<u8> {
    let (start_str, end_str) = input.split_once('-').unwrap();

    let start = start_str.parse().unwrap();
    let end = end_str.parse().unwrap();

    start..end
}

// Stolen from:
// https://www.reddit.com/r/rust/comments/aynxgl/is_there_a_rangetools_crate/
fn intersect(a: &Range<u8>, b: &Range<u8>) -> Range<u8> {
    max(a.start, b.start)..min(a.end, b.end)
}

fn main() {
    let misc = intersect(&(1..3), &(5..7));

    let input: Vec<_> = io::stdin()
        .lines()
        .map(|line_res| {
            let line = line_res.unwrap();
            let line_split = line.split_once(',').unwrap();
            (
                range_from_string(line_split.0),
                range_from_string(line_split.1),
            )
        })
        .collect();

    let answer_1 = input
        .iter()
        .filter(|pair| {
            let intersect = intersect(&pair.0, &pair.1);
            intersect == pair.0 || intersect == pair.1
        })
        .count();

    let answer_2 = input.iter().filter(|pair| min(pair.0.start)).count();

    println!("{}", answer_1);
}
