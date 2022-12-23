use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

use crate::{coord::Coord, direction::Direction, range::Range};

pub fn main(input: &str) -> (u64, u64) {
    let mut map = Map::from_str(input).unwrap();

    let mut round = 0;
    let mut p1: Option<u64> = None;
    let mut p2: Option<u64> = None;
    while p1.is_none() || p2.is_none() {
        let moved = map.do_round(round);

        round += 1;

        if moved == 0 && p2.is_none() {
            p2 = Some(round as u64);
        }

        if round == 10 {
            p1 = Some(map.count_empty_inside());
        }
    }

    (p1.unwrap(), p2.unwrap())
}

#[derive(Debug, Clone)]
struct Map {
    elves: HashSet<Coord<i64>>,
    x_range: Range,
    y_range: Range,
}

impl Map {
    fn count_empty_inside(&self) -> u64 {
        (self.x_range.width() as u64 * self.y_range.width() as u64) - self.elves.len() as u64
    }

    fn do_round(&mut self, round_index: usize) -> usize {
        let mut proposed: HashMap<Coord<i64>, Vec<Coord<i64>>> = HashMap::new();

        'elves: for elf in &self.elves {
            let adjacent = elf.adjacent_coords_include_diag();
            if adjacent.iter().all(|c| !self.elves.contains(c)) {
                proposed.entry(*elf).or_default().push(*elf);
                continue;
            }

            for i in 0..4 {
                let proposed_direction = Direction::all()[(round_index + i) % 4];
                let proposed_coord = elf.move_direction_udlr(proposed_direction, 1);
                let mut coords_to_consider = adjacent
                    .iter()
                    .filter(|c| c.manhattan_distance(proposed_coord) <= 1);
                if coords_to_consider.all(|c| !self.elves.contains(c)) {
                    proposed.entry(proposed_coord).or_default().push(*elf);
                    continue 'elves;
                }
            }

            proposed.entry(*elf).or_default().push(*elf);
        }

        let mut moved = 0;
        let mut final_x_range = Range(i64::MAX, i64::MIN);
        let mut final_y_range = Range(i64::MAX, i64::MIN);
        let mut final_elves = HashSet::new();
        for (proposed_end, elves_start) in proposed {
            if elves_start.len() == 1 {
                if proposed_end != elves_start[0] {
                    moved += 1;
                }

                final_elves.insert(proposed_end);
                final_x_range.add_point(proposed_end.0);
                final_y_range.add_point(proposed_end.1);
            } else {
                for start in elves_start {
                    final_elves.insert(start);
                    final_x_range.add_point(start.0);
                    final_y_range.add_point(start.1);
                }
            }
        }

        self.elves = final_elves;
        self.x_range = final_x_range;
        self.y_range = final_y_range;

        moved
    }
}

impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x_range = Range(i64::MAX, i64::MIN);
        let mut y_range = Range(i64::MAX, i64::MIN);
        let elves: HashSet<Coord<i64>> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        let x = x as i64;
                        let y = y as i64;
                        Some(Coord(x, y))
                    } else {
                        None
                    }
                })
            })
            .inspect(|coord| {
                x_range.add_point(coord.0);
                y_range.add_point(coord.1);
            })
            .collect();

        Ok(Map {
            elves,
            x_range,
            y_range,
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.y_range {
            for x in self.x_range {
                if self.elves.contains(&Coord(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
