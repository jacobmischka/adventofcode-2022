use std::{
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    pub fn perform<V>(&self, lhs: V, rhs: V) -> V
    where
        V: Add<V, Output = V> + Sub<V, Output = V> + Div<V, Output = V> + Mul<V, Output = V>,
    {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Subtract => lhs - rhs,
            Operation::Multiply => lhs * rhs,
            Operation::Divide => lhs / rhs,
        }
    }

    pub fn solve_lhs<V>(&self, result: V, rhs: V) -> V
    where
        V: Add<V, Output = V> + Sub<V, Output = V> + Div<V, Output = V> + Mul<V, Output = V>,
    {
        match self {
            Operation::Add => result - rhs,
            Operation::Subtract => result + rhs,
            Operation::Multiply => result / rhs,
            Operation::Divide => result * rhs,
        }
    }

    pub fn solve_rhs<V>(&self, result: V, lhs: V) -> V
    where
        V: Add<V, Output = V> + Sub<V, Output = V> + Div<V, Output = V> + Mul<V, Output = V>,
    {
        match self {
            Operation::Add => result - lhs,
            Operation::Subtract => lhs - result,
            Operation::Multiply => result / lhs,
            Operation::Divide => lhs / result,
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "*" => Ok(Operation::Multiply),
            "/" => Ok(Operation::Divide),
            s => Err(format!("invalid operation {s}")),
        }
    }
}
