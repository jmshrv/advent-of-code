use std::{collections::HashMap, io, str::FromStr};

use itertools::Itertools;

type WorryLevel = u64;
type WorryOp = dyn Fn(WorryLevel) -> WorryLevel;

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => Err(()),
        }
    }
}

impl Operator {
    fn eval(&self, a: WorryLevel, b: WorryLevel) -> WorryLevel {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
        }
    }
}

#[derive(Clone, Copy)]
struct WorryTest {
    divisible: WorryLevel,
    if_true_dest: usize,
    if_false_dest: usize,
}

impl FromStr for WorryTest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let divisible = lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let if_true = lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let if_false = lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Ok(Self {
            divisible,
            if_true_dest: if_true,
            if_false_dest: if_false,
        })
    }
}

impl WorryTest {
    fn eval(&self, worry_level: WorryLevel) -> usize {
        if worry_level % self.divisible == 0 {
            self.if_true_dest
        } else {
            self.if_false_dest
        }
    }
}

fn parse_operation(operation: &str) -> Box<WorryOp> {
    let mut segments = operation.split(' ').skip(2);

    let left_str = segments.next().unwrap();
    let operator_str = segments.next().unwrap();
    let right_str = segments.next().unwrap();

    let left: Option<WorryLevel> = if left_str == "old" {
        None
    } else {
        Some(left_str.parse().unwrap())
    };

    let operator: Operator = operator_str.parse().unwrap();

    let right: Option<WorryLevel> = if right_str == "old" {
        None
    } else {
        Some(right_str.parse().unwrap())
    };

    Box::new(move |old| {
        let old_relief = (old as f64 / 3.0).floor() as WorryLevel;
        operator.eval(left.unwrap_or(old_relief), right.unwrap_or(old_relief))
    })
}

struct Monkey {
    items: Vec<WorryLevel>,
    operation: Box<WorryOp>,
    test: WorryTest,
    inspect_count: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().skip(1); // Don't care about monkey number
        let items: Vec<WorryLevel> = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|worry_level| worry_level.parse().unwrap())
            .collect();

        let operation =
            parse_operation(lines.next().unwrap().strip_prefix("  Operation: ").unwrap());

        let test = lines.join("\n").parse().unwrap();

        Ok(Monkey {
            items,
            operation,
            test,
            inspect_count: 0,
        })
    }
}

impl Monkey {
    fn play(&self) -> HashMap<usize, Vec<WorryLevel>> {
        let mut destinations: HashMap<usize, Vec<WorryLevel>> = HashMap::new();
        for item in &self.items {
            let new_worry = (self.operation)(*item);
            let destination = self.test.eval(new_worry);

            destinations
                .entry(destination)
                .and_modify(|items| items.push(new_worry))
                .or_insert(vec![new_worry]);
        }

        destinations
    }
}

fn main() {
    let mut input: Vec<Monkey> = include_str!("input.txt")
        .split("\n\n")
        .map(|monkey_text| monkey_text.parse().unwrap())
        .collect();

    for _ in 0..20 {
        for monkey in &input {
            let new_dests = monkey.play();

            for mut dest in new_dests {
                input[dest.0 - 1].items.append(&mut dest.1);
            }
        }
    }
}
