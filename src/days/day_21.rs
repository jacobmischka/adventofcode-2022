use std::{collections::HashMap, str::FromStr};

use crate::operation::Operation;

pub fn main<'input>(input: &'input str) -> (u64, u64) {
    let (mut numbers, mut maths): (
        HashMap<&'input str, Monkey<'input>>,
        HashMap<&'input str, Monkey<'input>>,
    ) = input
        .lines()
        .map(|line| {
            let mut chunks = line.split(": ");
            let name = chunks.next().unwrap();
            let monkey = Monkey::from_str(chunks.next().unwrap()).unwrap();

            (name, monkey)
        })
        .partition(|(_, monkey)| match monkey {
            Monkey::Number(_) => true,
            Monkey::Math(_, _, _) => false,
        });

    (
        {
            let mut numbers = numbers.clone();
            let mut maths = maths.clone();
            while maths.contains_key(&"root") {
                let mut to_remove: Vec<&str> = Vec::new();
                for name in maths.keys().copied() {
                    let monkey = maths.get(&name).unwrap();
                    let val = match monkey {
                        Monkey::Math(op, Value::Variable(lhs), Value::Variable(rhs)) => {
                            if let (Some(Monkey::Number(lhs)), Some(Monkey::Number(rhs))) =
                                (numbers.get(lhs), numbers.get(rhs))
                            {
                                Some(op.perform(*lhs, *rhs))
                            } else {
                                None
                            }
                        }
                        _ => panic!("bad monkey {:?}", monkey),
                    };

                    if let Some(val) = val {
                        numbers.insert(name, Monkey::Number(val));
                        to_remove.push(name);
                    }
                }

                for name in to_remove {
                    maths.remove(&name);
                }
            }

            if let Some(Monkey::Number(p1)) = numbers.get(&"root") {
                *p1
            } else {
                panic!("no root");
            }
        },
        {
            numbers.remove(&"humn");
            while maths.len() > 0 {
                let mut to_remove: Vec<&str> = Vec::new();
                let names: Vec<&str> = maths.keys().copied().collect();
                for name in names {
                    let monkey = maths.get_mut(&name).unwrap();
                    let val = match monkey {
                        Monkey::Math(op, Value::Literal(lhs), Value::Literal(rhs)) => {
                            Some(op.perform(*lhs, *rhs))
                        }
                        Monkey::Math(op, Value::Literal(lhs), Value::Variable(rhs)) => {
                            if let Some(Monkey::Number(rhs)) = numbers.get(rhs) {
                                Some(op.perform(*lhs, *rhs))
                            } else if let Some(Monkey::Number(result)) = numbers.get(&name) {
                                let rhs_val = op.solve_rhs(*result, *lhs);
                                numbers.insert(rhs, Monkey::Number(rhs_val));
                                None
                            } else {
                                None
                            }
                        }
                        Monkey::Math(op, Value::Variable(lhs), Value::Literal(rhs)) => {
                            if let Some(Monkey::Number(lhs)) = numbers.get(lhs) {
                                Some(op.perform(*lhs, *rhs))
                            } else if let Some(Monkey::Number(result)) = numbers.get(&name) {
                                let lhs_val = op.solve_lhs(*result, *rhs);
                                numbers.insert(lhs, Monkey::Number(lhs_val));
                                None
                            } else {
                                None
                            }
                        }
                        Monkey::Math(op, ref mut lhs, ref mut rhs) => {
                            if let (Value::Variable(lhs_var), Value::Variable(rhs_var)) =
                                (*lhs, *rhs)
                            {
                                match (numbers.get(lhs_var), numbers.get(rhs_var)) {
                                    (Some(Monkey::Number(lhs)), Some(Monkey::Number(rhs))) => {
                                        Some(op.perform(*lhs, *rhs))
                                    }
                                    (Some(Monkey::Number(lhs_val)), None) => {
                                        *lhs = Value::Literal(*lhs_val);
                                        if name == "root" {
                                            *rhs = Value::Literal(*lhs_val);
                                            numbers.insert(rhs_var, Monkey::Number(*lhs_val));
                                            to_remove.push("root");
                                        }
                                        None
                                    }
                                    (None, Some(Monkey::Number(rhs_val))) => {
                                        *rhs = Value::Literal(*rhs_val);
                                        if name == "root" {
                                            *lhs = Value::Literal(*rhs_val);
                                            numbers.insert(lhs_var, Monkey::Number(*rhs_val));
                                            to_remove.push("root");
                                        }
                                        None
                                    }
                                    _ => None,
                                }
                            } else {
                                panic!("impossible")
                            }
                        }
                        _ => panic!("bad monkey: {:?}", monkey),
                    };

                    if let Some(val) = val {
                        numbers.insert(name, Monkey::Number(val));
                        to_remove.push(name);
                    }
                }

                for name in to_remove {
                    maths.remove(&name);
                }
            }

            if let Some(Monkey::Number(humn)) = numbers.get(&"humn") {
                *humn
            } else {
                panic!("no humn?");
            }
        },
    )
}

#[derive(Debug, Clone, Copy)]
enum Monkey<'a> {
    Number(u64),
    Math(Operation, Value<'a>, Value<'a>),
}

#[derive(Debug, Clone, Copy)]
enum Value<'a> {
    Literal(u64),
    Variable(&'a str),
}

impl<'a> Monkey<'a> {
    fn from_str(s: &'a str) -> Result<Self, String> {
        let mut words = s.split_whitespace();
        let lhs = words.next().ok_or(format!("missing lhs or number: {s}"))?;
        if let Ok(num) = lhs.parse::<u64>() {
            Ok(Monkey::Number(num))
        } else {
            let op = Operation::from_str(words.next().ok_or(format!("missing operation: {s}"))?)?;
            let rhs = words.next().ok_or(format!("missing rhs"))?;
            Ok(Monkey::Math(op, Value::Variable(lhs), Value::Variable(rhs)))
        }
    }
}
