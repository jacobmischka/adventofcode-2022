use std::collections::HashSet;
use std::str::FromStr;

use crate::direction::Direction;

pub fn main(input: &str) -> (u32, u32) {
    let mut rope1 = Rope::new(2);
    let mut visited1: HashSet<Position> = HashSet::new();
    visited1.insert(Position(0, 0));
    let mut rope2 = Rope::new(10);
    let mut visited2: HashSet<Position> = HashSet::new();
    visited2.insert(Position(0, 0));

    for line in input.lines() {
        let mut pieces = line.split_whitespace();
        let direction = Direction::from_str(pieces.next().unwrap()).unwrap();
        let num: usize = pieces.next().unwrap().parse().unwrap();

        for _ in 0..num {
            rope1.move_dir(direction);
            visited1.insert(*rope1.tail().unwrap());

            rope2.move_dir(direction);
            visited2.insert(*rope2.tail().unwrap());

            if cfg!(feature = "debug") {
                for y in (0..50).rev() {
                    for x in 0..50 {
                        if rope2.head().unwrap() == &Position(x, y) {
                            print!("H");
                        } else if rope2.tail().unwrap() == &Position(x, y) {
                            print!("T");
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
                print!("\t");

                for y in (0..50).rev() {
                    for x in 0..50 {
                        if rope2.head().unwrap() == &Position(x, y) {
                            print!("H");
                        } else if rope2.tail().unwrap() == &Position(x, y) {
                            print!("T");
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
                println!();
            }
        }
    }

    (visited1.len() as _, visited2.len() as _)
}

struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    fn new(len: usize) -> Self {
        Rope {
            knots: vec![Position(0, 0); len],
        }
    }

    fn head(&self) -> Option<&Position> {
        self.knots.first()
    }

    fn tail(&self) -> Option<&Position> {
        self.knots.last()
    }

    fn move_dir(&mut self, direction: Direction) {
        if self.knots.len() == 0 {
            return;
        }

        self.knots[0].move_dir(direction);

        if self.knots.len() == 1 {
            return;
        }

        for i in 1..self.knots.len() {
            if !self.knots[i].is_touching(&self.knots[i - 1]) {
                let distance = self.knots[i - 1].manhattan_distance(&self.knots[i]);
                self.knots[i].0 += distance.0.signum();
                self.knots[i].1 += distance.1.signum();
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position(i32, i32);

impl Position {
    fn move_dir(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.1 += 1,
            Direction::Down => self.1 -= 1,
            Direction::Left => self.0 -= 1,
            Direction::Right => self.0 += 1,
        }
    }

    fn is_touching(&self, other: &Position) -> bool {
        self.0.abs_diff(other.0) <= 1 && self.1.abs_diff(other.1) <= 1
    }

    fn manhattan_distance(&self, other: &Position) -> (i32, i32) {
        (self.0 - other.0, self.1 - other.1)
    }
}
