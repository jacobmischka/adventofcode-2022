use std::fmt::{Debug, Display};

use crate::Coord;

pub fn main(input: &str) -> (u32, u32) {
    let mut start: Option<Coord<usize>> = None;
    let mut end: Option<Coord<usize>> = None;
    let mut lowest: Vec<Coord<usize>> = Vec::new();

    let elevations: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Some(Coord(x, y));
                    } else if c == 'E' {
                        end = Some(Coord(x, y));
                    }

                    if c == 'S' || c == 'a' {
                        lowest.push(Coord(x, y));
                    }

                    c
                })
                .collect::<Vec<char>>()
        })
        .collect();

    let start = start.unwrap();
    let end = end.unwrap();

    let dist_map: TopMap<Option<u32>> = TopMap {
        coords: vec![vec![None; elevations[0].len()]; elevations.len()],
    };
    let height_map = TopMap { coords: elevations };

    (
        {
            let mut new_dist_map = dist_map.clone();
            new_dist_map.set_pos(start, Some(0));
            spread(&height_map, &mut new_dist_map, start);
            new_dist_map.get_pos(end).unwrap()
        },
        {
            let mut min = u32::MAX;
            for c in lowest {
                let mut new_dist_map = dist_map.clone();
                new_dist_map.set_pos(c, Some(0));
                spread(&height_map, &mut new_dist_map, c);
                match new_dist_map.get_pos(end) {
                    Some(steps) => {
                        if steps < min {
                            min = steps;
                        }
                    }
                    None => {}
                }
            }
            min
        },
    )
}

fn spread(height_map: &TopMap<char>, dist_map: &mut TopMap<Option<u32>>, pos: Coord<usize>) {
    let current_height = match height_map.get_pos(pos) {
        'S' => 'a',
        'E' => 'z',
        c => c,
    };
    let current_dist = dist_map.get_pos(pos).unwrap();

    if pos.1 > 0 {
        check_coord(
            height_map,
            dist_map,
            current_height,
            current_dist,
            Coord(pos.0, pos.1 - 1),
        );
    }

    if pos.1 < height_map.coords.len() - 1 {
        check_coord(
            height_map,
            dist_map,
            current_height,
            current_dist,
            Coord(pos.0, pos.1 + 1),
        );
    }

    if pos.0 > 0 {
        check_coord(
            height_map,
            dist_map,
            current_height,
            current_dist,
            Coord(pos.0 - 1, pos.1),
        );
    }

    if pos.0 < dist_map.coords[pos.1].len() - 1 {
        check_coord(
            height_map,
            dist_map,
            current_height,
            current_dist,
            Coord(pos.0 + 1, pos.1),
        );
    }
}

fn check_coord(
    height_map: &TopMap<char>,
    dist_map: &mut TopMap<Option<u32>>,
    current_height: char,
    current_dist: u32,
    new_coord: Coord<usize>,
) {
    let height = match height_map.get_pos(new_coord) {
        'S' => 'a',
        'E' => 'z',
        c => c,
    };
    if (height as i32 - current_height as i32) <= 1 {
        let prev_dist = dist_map.get_pos(new_coord);
        if prev_dist.is_none() || prev_dist.unwrap() > current_dist + 1 {
            dist_map.set_pos(new_coord, Some(current_dist + 1));
            spread(height_map, dist_map, new_coord);
        }
    }
}

#[derive(Clone)]
struct TopMap<T: Clone> {
    coords: Vec<Vec<T>>,
}

impl<T: Copy> TopMap<T> {
    fn get_pos(&self, pos: Coord<usize>) -> T {
        self.coords[pos.1][pos.0]
    }

    fn set_pos(&mut self, pos: Coord<usize>, val: T) {
        self.coords[pos.1][pos.0] = val;
    }
}

impl<T: Display + Clone> Display for TopMap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in 0..(self.coords.len()) {
            for x in 0..(self.coords[y].len()) {
                write!(f, "{}", self.coords[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Debug + Clone> Debug for TopMap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in 0..(self.coords.len()) {
            for x in 0..(self.coords[y].len()) {
                write!(f, "{:?}\t", self.coords[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
