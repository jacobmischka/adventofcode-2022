use std::{collections::HashMap, fmt::Display, str::FromStr};

use crate::coord::Coord;

const SAND_SPAWN: Coord<i32> = Coord(500, 0);

pub fn main(input: &str) -> (u32, u32) {
    let mut grid = Grid::default();
    grid.read_wall_lines(input);

    let max_y = grid.max_y().unwrap();
    let max_x = grid.max_x().unwrap();
    let min_x = grid.min_x().unwrap();
    let floor_y = max_y + 2;

    let down_unit = Coord(0, 1);
    let left_unit = Coord(-1, 0);
    let right_unit = Coord(1, 0);

    for x in 0..1000 {
        grid.coords.insert(Coord(x, floor_y), Tile::Wall);
    }

    let mut p1_sands = 0;
    let mut p2_sands = 0;
    let mut p1_over = false;

    'sand_lifecycle: loop {
        let mut sand = SAND_SPAWN;
        loop {
            if grid.coords.get(&(sand + down_unit)).is_none() {
                sand = sand + down_unit;
            } else if grid.coords.get(&(sand + down_unit + left_unit)).is_none() {
                sand = sand + down_unit + left_unit;
            } else if grid.coords.get(&(sand + down_unit + right_unit)).is_none() {
                sand = sand + down_unit + right_unit;
            } else {
                grid.coords.insert(sand, Tile::Sand);

                if sand.0 >= min_x && sand.0 <= max_x && sand.1 <= max_y && !p1_over {
                    p1_sands += 1;
                } else if !p1_over {
                    p1_over = true;
                }

                p2_sands += 1;

                if sand == SAND_SPAWN {
                    break 'sand_lifecycle;
                } else {
                    continue 'sand_lifecycle;
                }
            }
        }
    }

    (p1_sands, p2_sands)
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Sand,
}

#[derive(Debug, Clone, Default)]
struct Grid {
    coords: HashMap<Coord<i32>, Tile>,
}

impl Grid {
    fn read_wall_lines(&mut self, input: &str) {
        for line in input.lines() {
            let mut points = line
                .split(" -> ")
                .map(|s| Coord::<i32>::from_str(s).unwrap());
            let mut point = points.next().unwrap();
            self.coords.insert(point, Tile::Wall);

            for next_point in points {
                let delta = next_point.unit_difference(point);
                while point != next_point {
                    point = point + delta;
                    self.coords.insert(point, Tile::Wall);
                }
            }
        }
    }

    fn min_x(&self) -> Option<i32> {
        self.coords.keys().map(|Coord(x, _)| *x).min()
    }

    fn max_x(&self) -> Option<i32> {
        self.coords.keys().map(|Coord(x, _)| *x).max()
    }

    #[allow(unused)]
    fn min_y(&self) -> Option<i32> {
        self.coords.keys().map(|Coord(_, y)| *y).min()
    }

    fn max_y(&self) -> Option<i32> {
        self.coords.keys().map(|Coord(_, y)| *y).max()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_y = self.max_y().unwrap_or(0);
        let min_x = self.min_x().unwrap_or(0);
        let max_x = self.max_x().unwrap_or(0);

        for y in 0..=max_y {
            for x in min_x..=max_x {
                match self.coords.get(&Coord(x, y)) {
                    Some(x) => write!(f, "{}", x)?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Wall => "#",
                Tile::Sand => "o",
            }
        )
    }
}
