use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::coord::Coord;

const SPAWN_X: u64 = 2;
const TOTAL_ROCKS: usize = 1000000000000;

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

            // if rock.pos.1 > chamber.top {
            //     continue;
            // }

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
        let existing = chamber.top_deltas_to_rock_num.get(&top_deltas).copied();
        chamber.top_deltas_to_rock_num.insert(top_deltas, i);
        chamber.rock_num_to_tops.push(chamber.tops.clone());

        if let Some(j) = existing {
            dbg!(existing, i);
            let i_max = chamber.rock_num_to_tops[i].iter().copied().max().unwrap();
            let j_min = chamber.rock_num_to_tops[j].iter().copied().min().unwrap();
            let delta = i_min - j_min;
            if (i_min..=i_max).all(|y| {
                (0..chamber.width).all(|x| {
                    if chamber.filled.contains(&Coord(x, y)) {
                        chamber.filled.contains(&Coord(x, y - delta))
                    } else {
                        true
                    }
                })
            }) {
                chamber.cycle = Some((j - 5, i - 5));
                break;
            }
        }
    }

    println!("{}", &chamber);
    dbg!(&chamber.cycle);
    if let Some((start, end)) = &chamber.cycle {
        dbg!(
            &chamber.rock_num_to_tops[*start],
            &chamber.rock_num_to_tops[*end],
        );
    }

    (
        chamber.get_top(2021).unwrap() + 1,
        chamber.get_top(TOTAL_ROCKS - 1).unwrap() + 1,
    )
}

struct Chamber {
    width: u64,
    filled: HashSet<Coord<u64>>,
    top: u64,
    tops: Vec<u64>,
    top_deltas_to_rock_num: HashMap<Vec<u64>, usize>,
    rock_num_to_tops: Vec<Vec<u64>>,
    cycle: Option<(usize, usize)>,
}

impl Chamber {
    fn new(width: u64) -> Self {
        Chamber {
            width,
            filled: HashSet::new(),
            top: 0,
            tops: vec![0; width as usize],
            rock_num_to_tops: Vec::new(),
            top_deltas_to_rock_num: HashMap::new(),
            cycle: None,
        }
    }

    fn get_top(&self, rock_num: usize) -> Option<u64> {
        if let Some(tops) = self.rock_num_to_tops.get(rock_num) {
            tops.iter().copied().max().map(|y| y)
        } else if let Some((start, end)) = self.cycle {
            let rock_num_after_start = rock_num - start;
            let cycle_size = end - start;
            let num_cycles = rock_num_after_start / cycle_size;
            let rem_after_start = rock_num_after_start % cycle_size;

            let cycle_start_top = self.get_top(start)?;
            let cycle_end_top = self.get_top(end)?;
            let cycle_delta_height = cycle_end_top - cycle_start_top;

            let rem_delta_height = self.get_top(start + rem_after_start)?;

            dbg!(
                rock_num,
                rock_num_after_start,
                cycle_size,
                num_cycles,
                rem_after_start,
                start,
                end,
                cycle_start_top,
                cycle_end_top,
                cycle_delta_height,
                rem_delta_height,
            );

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
