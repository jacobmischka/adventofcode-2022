use std::str::FromStr;

use crate::operation::Operation;

pub fn main(input: &str) -> (u64, u64) {
    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|s| Monkey::from_str(s).unwrap())
        .collect();

    let part1 = throw_around(monkeys, 20, false);

    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|s| Monkey::from_str(s).unwrap())
        .collect();

    let part2 = throw_around(monkeys, 10000, true);

    (part1, part2)
}

fn throw_around(mut monkeys: Vec<Monkey>, rounds: usize, worried: bool) -> u64 {
    let common_multiple = monkeys
        .iter()
        .map(|m| m.divisible_test)
        .fold(1, |acc, x| acc * x);

    let mut trues: Vec<u64> = Vec::new();
    let mut falses: Vec<u64> = Vec::new();
    for round in 0..rounds {
        for i in 0..monkeys.len() {
            let true_monkey: usize;
            let false_monkey: usize;
            {
                let monkey = &mut monkeys[i];
                true_monkey = monkey.if_divisible_true;
                false_monkey = monkey.if_divisible_false;
                for mut item_worry_level in monkey.items.drain(..) {
                    if worried {
                        item_worry_level =
                            monkey.operation.as_mut()(item_worry_level) % common_multiple;
                    } else {
                        item_worry_level = monkey.operation.as_mut()(item_worry_level) / 3;
                    }

                    if item_worry_level % monkey.divisible_test == 0 {
                        trues.push(item_worry_level);
                    } else {
                        falses.push(item_worry_level);
                    }

                    monkey.total_inspections += 1;
                }
            }
            monkeys[true_monkey].items.append(&mut trues);
            monkeys[false_monkey].items.append(&mut falses);
        }

        if cfg!(feature = "debug") {
            if worried {
                if (round + 1) % 1000 == 0 || round == 0 || round + 1 == 20 {
                    eprintln!("== After round {} ==", round + 1);
                    for (i, monkey) in monkeys.iter().enumerate() {
                        eprintln!(
                            "Monkey {i} inspected items {:?} times.",
                            &monkey.total_inspections
                        );
                    }
                    eprintln!();
                }
            } else {
                for (i, monkey) in monkeys.iter().enumerate() {
                    eprintln!("Monkey {i}: {:?}", &monkey.items);
                }
                eprintln!();
            }
        }
    }

    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.total_inspections).collect();
    inspections.sort();

    (inspections.pop().unwrap() * inspections.pop().unwrap()) as u64
}

struct Monkey {
    items: Vec<u64>,
    total_inspections: usize,
    operation: Box<dyn Fn(u64) -> u64>,
    divisible_test: u64,
    if_divisible_true: usize,
    if_divisible_false: usize,
}

impl Default for Monkey {
    fn default() -> Self {
        Monkey {
            items: Vec::new(),
            total_inspections: 0,
            operation: Box::new(|x| x),
            divisible_test: 0,
            if_divisible_true: 0,
            if_divisible_false: 0,
        }
    }
}

enum Value {
    Variable(String),
    Literal(u64),
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u64>() {
            Ok(val) => Ok(Value::Literal(val)),
            _ => Ok(Value::Variable(s.to_string())),
        }
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkey = Monkey::default();

        for line in s.lines() {
            if line.starts_with("Monkey ") {
                continue;
            }

            let mut chunks = line.trim().split(": ");
            match chunks.next().unwrap() {
                "Starting items" => {
                    monkey.items = chunks
                        .next()
                        .ok_or("missing chunk for starting items".to_string())?
                        .trim()
                        .split(", ")
                        .map(|s| s.parse().unwrap())
                        .collect();
                }
                "Operation" => {
                    let mut pieces = chunks
                        .next()
                        .ok_or("missing chunk for starting items".to_string())?
                        .split_whitespace()
                        .skip(2);

                    let lhs =
                        Value::from_str(pieces.next().ok_or("missing operaton lhs".to_string())?)?;
                    let operation = Operation::from_str(
                        pieces
                            .next()
                            .ok_or("missing operaton operation".to_string())?,
                    )?;
                    let rhs =
                        Value::from_str(pieces.next().ok_or("missing operaton rhs".to_string())?)?;

                    monkey.operation = Box::new(move |old| {
                        operation.perform(
                            match lhs {
                                Value::Literal(x) => x,
                                Value::Variable(_) => old,
                            },
                            match rhs {
                                Value::Literal(x) => x,
                                Value::Variable(_) => old,
                            },
                        )
                    });
                }
                "Test" => {
                    monkey.divisible_test = chunks
                        .next()
                        .ok_or("missing chunk for test".to_string())?
                        .split_whitespace()
                        .last()
                        .ok_or("test missing value".to_string())?
                        .parse()
                        .map_err(|e| format!("invalid test value {:?}", e))?;
                }
                "If true" => {
                    monkey.if_divisible_true = chunks
                        .next()
                        .ok_or("missing chunk for true".to_string())?
                        .split_whitespace()
                        .last()
                        .ok_or("test missing value".to_string())?
                        .parse()
                        .map_err(|e| format!("invalid true value {:?}", e))?;
                }
                "If false" => {
                    monkey.if_divisible_false = chunks
                        .next()
                        .ok_or("missing chunk for false".to_string())?
                        .split_whitespace()
                        .last()
                        .ok_or("test missing value".to_string())?
                        .parse()
                        .map_err(|e| format!("invalid false value {:?}", e))?;
                }
                s => return Err(format!("unrecognized chunk: {s}")),
            }
        }

        Ok(monkey)
    }
}
