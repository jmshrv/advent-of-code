use std::io;

use itertools::Itertools;

#[derive(Clone, Copy)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scisors = 3,
}

impl TryFrom<char> for Play {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scisors),
            _ => Err(()),
        }
    }
}

impl Play {
    fn score(self, play: Play) -> usize {
        let outcome = Outcome::from((self, play));

        outcome as usize + play as usize
    }
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl From<(Play, Play)> for Outcome {
    fn from(play: (Play, Play)) -> Self {
        match play {
            (Play::Rock, Play::Paper) => Self::Win,
            (Play::Rock, Play::Scisors) => Self::Loss,
            (Play::Paper, Play::Rock) => Self::Loss,
            (Play::Paper, Play::Scisors) => Self::Win,
            (Play::Scisors, Play::Rock) => Self::Win,
            (Play::Scisors, Play::Paper) => Self::Loss,
            _ => Self::Draw,
        }
    }
}

fn main() {
    let input: Vec<(Play, Play)> = io::stdin()
        .lines()
        .map(|x| {
            x.unwrap()
                .split(' ')
                .take(2)
                .map(|y| Play::try_from(y.chars().next().unwrap()).unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let score: usize = input.iter().map(|x| x.0.score(x.1)).sum();

    println!("{}", score);
}
