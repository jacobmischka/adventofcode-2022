use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::coord::Coord;

const SPAWN_X: u64 = 2;
const TOTAL_ROCKS: usize = 1000000000000;
const CYCLE_CHECK_HEIGHT_HEURISTIC: usize = 10;

pub fn main(input: &str) -> (u64, u64) {
    let jet_pattern: Vec<JetDirection> = input
        .chars()
        .map(|c| JetDirection::try_from(c).unwrap())
        .collect();
    let mut jets = jet_pattern.iter().copied().cycle();

    let mut chamber = Chamber::new(7);

    for (i, shape) in RockShape::all()
        .into_iter()
        .cycle()
        .take(TOTAL_ROCKS)
        .enumerate()
    {
        let mut rock = Rock {
            shape,
            pos: Coord(SPAWN_X, chamber.top + 3),
        };

        loop {
            if let Some(x) = match jets.next().unwrap() {
                JetDirection::Left => rock.pos.0.checked_sub(1),
                JetDirection::Right => rock.pos.0.checked_add(1),
            } {
                let prev = rock.pos.0;
                rock.pos.0 = x;
                if rock.pos.0 + rock.shape.width() > chamber.width
                    || rock.coords().any(|c| chamber.filled.contains(&c))
                {
                    rock.pos.0 = prev;
                }
            }

            if rock.pos.1 == 0 {
                break;
            }

            rock.pos.1 -= 1;

            if rock.pos.1 > chamber.top {
                continue;
            }

            if rock.coords().any(|c| chamber.filled.contains(&c)) {
                rock.pos.1 += 1;
                break;
            }
        }

        for c in rock.coords() {
            chamber.tops[c.0 as usize] = chamber.tops[c.0 as usize].max(c.1);
            let new = chamber.filled.insert(c);
            if !new {
                panic!("overlap detected! {:?}, {:?}", c, chamber.filled);
            }
        }
        chamber.top = chamber.top.max(rock.pos.1 + rock.shape.height());
        let i_min = chamber.tops.iter().copied().min().unwrap();
        let top_deltas: Vec<u64> = chamber.tops.iter().copied().map(|y| y - i_min).collect();
        let existing = chamber.top_deltas_to_rock_index.get(&top_deltas).copied();
        chamber.top_deltas_to_rock_index.insert(top_deltas, i);
        chamber.rock_index_to_tops.push(chamber.tops.clone());

        if let Some(j) = existing {
            let cycle_size = i - j;
            if cycle_size % RockShape::all().len() == 0 {
                if let Some((start, end)) = chamber.cycle {
                    let offset = i - end;
                    if start > j || j - start != offset {
                        chamber.cycle = None;
                    } else if offset >= CYCLE_CHECK_HEIGHT_HEURISTIC {
                        let i_max = chamber.tops.iter().copied().max().unwrap();
                        let start_tops = &chamber.rock_index_to_tops[start];
                        let end_tops = &chamber.rock_index_to_tops[end];
                        let end_bot = end_tops.iter().copied().min().unwrap();
                        let start_bot = start_tops.iter().copied().min().unwrap();
                        let cycle_diff = end_bot - start_bot;

                        if (end_bot..=i_max).rev().all(|y| {
                            (0..chamber.width).all(|x| {
                                !chamber.filled.contains(&Coord(x, y))
                                    || chamber.filled.contains(&Coord(x, y - cycle_diff))
                            })
                        }) {
                            break;
                        } else {
                            chamber.cycle = None;
                        }
                    }
                } else {
                    chamber.cycle = Some((j, i));
                }
            }
        }
    }

    // println!("{}", &chamber);
    (
        chamber.get_height(2022).unwrap(),
        chamber.get_height(TOTAL_ROCKS).unwrap(),
    )
}

struct Chamber {
    width: u64,
    filled: HashSet<Coord<u64>>,
    top: u64,
    tops: Vec<u64>,
    top_deltas_to_rock_index: HashMap<Vec<u64>, usize>,
    rock_index_to_tops: Vec<Vec<u64>>,
    cycle: Option<(usize, usize)>,
}

impl Chamber {
    fn new(width: u64) -> Self {
        Chamber {
            width,
            filled: HashSet::new(),
            top: 0,
            tops: vec![0; width as usize],
            rock_index_to_tops: Vec::new(),
            top_deltas_to_rock_index: HashMap::new(),
            cycle: None,
        }
    }

    fn get_height(&self, rock_num: usize) -> Option<u64> {
        self.get_top(rock_num - 1).map(|top| top + 1)
    }

    fn get_top(&self, rock_index: usize) -> Option<u64> {
        if let Some(tops) = self.rock_index_to_tops.get(rock_index) {
            tops.iter().copied().max().map(|y| y)
        } else if let Some((start, end)) = self.cycle {
            let rock_index_after_start = rock_index - start;
            let cycle_size = end - start;
            let num_cycles = rock_index_after_start / cycle_size;
            let rem_after_start = rock_index_after_start % cycle_size;

            let cycle_start_top = self.get_top(start)?;
            let cycle_end_top = self.get_top(end)?;
            let cycle_delta_height = cycle_end_top - cycle_start_top;

            let rem_delta_height = self.get_top(start + rem_after_start)?;

            Some((cycle_delta_height * num_cycles as u64) + rem_delta_height)
        } else {
            None
        }
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let top = self.top + 2;

        write!(f, "\t")?;
        for x in 0..=(self.width + 1) {
            if x == 0 || x == self.width + 1 {
                write!(f, "+")?;
            } else {
                write!(f, "-")?;
            }
        }
        writeln!(f)?;

        for y in (0..=top).rev() {
            write!(f, "{}\t", y)?;
            write!(f, "|")?;
            for x in 0..self.width {
                if self.filled.contains(&Coord(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "|")?;
        }

        write!(f, "\t")?;
        for x in 0..=(self.width + 1) {
            if x == 0 || x == self.width + 1 {
                write!(f, "+")?;
            } else {
                write!(f, "-")?;
            }
        }
        writeln!(f)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Rock {
    shape: RockShape,
    pos: Coord<u64>,
}

impl Rock {
    fn coords(self) -> impl Iterator<Item = Coord<u64>> {
        self.shape.coords().into_iter().map(move |c| *c + self.pos)
    }
}

#[derive(Debug, Clone, Copy)]
enum RockShape {
    Horizontal,
    Plus,
    BackwardsL,
    Vertical,
    Square,
}

impl RockShape {
    fn all() -> [RockShape; 5] {
        [
            RockShape::Horizontal,
            RockShape::Plus,
            RockShape::BackwardsL,
            RockShape::Vertical,
            RockShape::Square,
        ]
    }

    fn coords(self) -> &'static [Coord<u64>] {
        match self {
            RockShape::Horizontal => &[Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(3, 0)],
            RockShape::Plus => &[
                Coord(1, 0),
                Coord(0, 1),
                Coord(1, 1),
                Coord(2, 1),
                Coord(1, 2),
            ],
            RockShape::BackwardsL => &[
                Coord(0, 0),
                Coord(1, 0),
                Coord(2, 0),
                Coord(2, 1),
                Coord(2, 2),
            ],
            RockShape::Vertical => &[Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(0, 3)],
            RockShape::Square => &[Coord(0, 0), Coord(0, 1), Coord(1, 0), Coord(1, 1)],
        }
    }

    fn width(self) -> u64 {
        match self {
            RockShape::Horizontal => 4,
            RockShape::Plus => 3,
            RockShape::BackwardsL => 3,
            RockShape::Vertical => 1,
            RockShape::Square => 2,
        }
    }

    fn height(self) -> u64 {
        match self {
            RockShape::Horizontal => 1,
            RockShape::Plus => 3,
            RockShape::BackwardsL => 3,
            RockShape::Vertical => 4,
            RockShape::Square => 2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum JetDirection {
    Left,
    Right,
}

impl TryFrom<char> for JetDirection {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(JetDirection::Left),
            '>' => Ok(JetDirection::Right),
            c => Err(c),
        }
    }
}
