use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    str::FromStr,
};

use crate::{coord::Coord, direction::Direction};

const SEARCH_LIMIT: u32 = 500;

pub fn main(input: &str) -> (u32, u32) {
    let valley = Valley::from_str(input).unwrap();

    let (first, valley) = find_min((valley.start, Direction::Down), valley.end, valley).unwrap();
    let (second, valley) = find_min((valley.end, Direction::Up), valley.start, valley).unwrap();
    let (third, _) = find_min((valley.start, Direction::Down), valley.end, valley).unwrap();

    (first, first + second + third)
}

fn find_min(
    (start, start_dir): (Coord<usize>, Direction),
    dest: Coord<usize>,
    valley: Valley,
) -> Option<(u32, Valley)> {
    let mut cache: HashMap<String, u32> = HashMap::new();
    let mut stack: VecDeque<(Valley, u32)> = VecDeque::new();
    stack.push_front((valley, 0));

    let mut min = SEARCH_LIMIT;
    let mut min_valley_end: Option<Valley> = None;

    while !stack.is_empty() {
        let (mut valley, steps_so_far) = stack.pop_front().unwrap();

        valley.advance_blizzards();

        if valley.expedition.manhattan_distance(dest) == 1 {
            if steps_so_far + 1 < min {
                min = steps_so_far + 1;
                min_valley_end = Some(valley);
            }
            continue;
        }

        if steps_so_far >= min - valley.expedition.manhattan_distance(dest) as u32 {
            continue;
        }

        if valley.expedition == start {
            let next_pos = valley.expedition.move_direction_udlr(start_dir, 1);
            if !valley.blizzards.contains_key(&next_pos) {
                let new_valley = Valley {
                    expedition: next_pos,
                    ..valley.clone()
                };
                let serialized = new_valley.to_string();
                if let Some(prev_steps) = cache.get_mut(&serialized) {
                    if steps_so_far + 1 < *prev_steps {
                        *prev_steps = steps_so_far + 1;
                        stack.push_front((new_valley, steps_so_far + 1));
                    }
                } else {
                    cache.insert(serialized, steps_so_far + 1);
                    stack.push_front((new_valley, steps_so_far + 1));
                }
            }
        } else {
            for dir in Direction::all() {
                let next_pos = valley.expedition.move_direction_udlr(*dir, 1);
                if next_pos.0 > 0
                    && next_pos.0 <= valley.bound.0
                    && next_pos.1 > 0
                    && next_pos.1 <= valley.bound.1
                    && !valley.blizzards.contains_key(&next_pos)
                {
                    let new_valley = Valley {
                        expedition: next_pos,
                        ..valley.clone()
                    };
                    let serialized = new_valley.to_string();
                    if let Some(prev_steps) = cache.get_mut(&serialized) {
                        if steps_so_far + 1 < *prev_steps {
                            *prev_steps = steps_so_far + 1;
                            stack.push_front((new_valley, steps_so_far + 1));
                        }
                    } else {
                        cache.insert(serialized, steps_so_far + 1);
                        stack.push_front((new_valley, steps_so_far + 1));
                    }
                }
            }
        }

        if !valley.blizzards.contains_key(&valley.expedition) {
            let serialized = valley.to_string();
            if let Some(prev_steps) = cache.get_mut(&serialized) {
                if steps_so_far + 1 < *prev_steps {
                    *prev_steps = steps_so_far + 1;
                    stack.push_front((valley, steps_so_far + 1));
                }
            } else {
                cache.insert(serialized, steps_so_far + 1);
                stack.push_front((valley, steps_so_far + 1));
            }
        }
    }

    min_valley_end.map(|valley| (min, valley))
}

type Blizzards = HashMap<Coord<usize>, Vec<Direction>>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valley {
    bound: Coord<usize>,
    blizzards: Blizzards,
    expedition: Coord<usize>,
    start: Coord<usize>,
    end: Coord<usize>,
}

impl Valley {
    fn advance_blizzards(&mut self) {
        let mut blizzards: Blizzards = HashMap::new();
        for (pos, dirs) in &self.blizzards {
            for dir in dirs {
                let mut new_pos = pos.move_direction_udlr(*dir, 1);
                if new_pos.0 == 0 {
                    new_pos.0 = self.bound.0;
                } else if new_pos.0 > self.bound.0 {
                    new_pos.0 = 1;
                }

                if new_pos.1 == 0 {
                    new_pos.1 = self.bound.1;
                } else if new_pos.1 > self.bound.1 {
                    new_pos.1 = 1;
                }

                blizzards.entry(new_pos).or_default().push(*dir);
            }
        }

        self.blizzards = blizzards;
    }
}

impl Display for Valley {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=(self.bound.1 + 1) {
            for x in 0..=(self.bound.0 + 1) {
                if self.expedition == Coord(x, y) {
                    write!(f, "E")?;
                } else if y == 0 {
                    if self.start == Coord(x, y) {
                        write!(f, ".")?;
                    } else {
                        write!(f, "#")?;
                    }
                } else if y > self.bound.1 {
                    if self.end == Coord(x, y) {
                        write!(f, ".")?;
                    } else {
                        write!(f, "#")?;
                    }
                } else if x == 0 {
                    write!(f, "#")?;
                } else if x > self.bound.0 {
                    write!(f, "#")?;
                } else if let Some(dirs) = self.blizzards.get(&Coord(x, y)) {
                    let len = dirs.len();
                    if dirs.is_empty() {
                        write!(f, ".")?;
                    } else if len == 1 {
                        write!(f, "{}", dirs[0])?;
                    } else {
                        write!(f, "{}", len)?;
                    }
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromStr for Valley {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Option<Coord<usize>> = None;
        let mut end: Option<Coord<usize>> = None;
        let mut bound: Option<Coord<usize>> = None;
        let mut blizzards: HashMap<Coord<usize>, Vec<Direction>> = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            let mut is_bound_line = false;
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        if x > 0 {
                            is_bound_line = true;
                        }
                    }
                    '.' => {
                        if y == 0 {
                            start = Some(Coord(x, y));
                        } else if is_bound_line {
                            end = Some(Coord(x, y));
                        } else {
                            bound = Some(Coord(x, y));
                        }
                    }
                    d => {
                        let dir = Direction::try_from(d)?;
                        bound = Some(Coord(x, y));
                        blizzards.insert(Coord(x, y), vec![dir]);
                    }
                }
            }
        }

        let bound = bound.ok_or_else(|| "bound not found?".to_string())?;
        let start = start.ok_or_else(|| "no start found".to_string())?;
        let end = end.ok_or_else(|| "no end found".to_string())?;

        Ok(Valley {
            bound,
            blizzards,
            expedition: start,
            start,
            end,
        })
    }
}
