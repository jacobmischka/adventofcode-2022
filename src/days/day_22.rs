use std::str::FromStr;

use crate::{coord::Coord, direction::Direction};

pub fn main(input: &str) -> (u32, u32) {
    let mut chunks = input.split("\n\n");
    let map = Map::from_str(chunks.next().unwrap()).unwrap();

    let path = chunks.next().unwrap();
    let movements: Vec<Movement> = path
        .split_inclusive(&['L', 'R'])
        .flat_map(|pair| {
            if pair.contains(&['L', 'R']) {
                let len = pair.len();
                vec![
                    Movement::Move(pair[..(len - 1)].parse().unwrap()),
                    Movement::Turn(Turn::from_str(&pair[(len - 1)..]).unwrap()),
                ]
            } else {
                vec![Movement::Move(pair.parse().unwrap())]
            }
        })
        .collect();

    let mut flat_you = Position {
        pos: Coord(map.row_ends[0].0, 0),
        facing: Direction::Right,
    };
    // let mut folded_you = flat_you.clone();

    for movement in &movements {
        flat_you.do_movement(*movement, &map, MapMode::Flat);
        // folded_you.do_movement(*movement, &map, MapMode::Folded);
    }

    (flat_you.get_password(), 0)
}

#[derive(Debug, Clone)]
struct Position {
    pos: Coord<usize>,
    facing: Direction,
}

impl Position {
    fn get_password(&self) -> u32 {
        ((1000 * (self.pos.1 + 1))
            + 4 * (self.pos.0 + 1)
            + match self.facing {
                Direction::Right => 0,
                Direction::Down => 1,
                Direction::Left => 2,
                Direction::Up => 3,
            }) as u32
    }

    fn do_movement(&mut self, movement: Movement, map: &Map, map_mode: MapMode) {
        match movement {
            Movement::Move(mut distance) => {
                while distance > 0 {
                    let mut pos = self.pos;
                    loop {
                        pos = match self.facing {
                            Direction::Up => {
                                if pos.1 == 0 {
                                    Coord(pos.0, map.tiles.len() - 1)
                                } else {
                                    Coord(pos.0, pos.1 - 1)
                                }
                            }
                            Direction::Down => Coord(pos.0, (pos.1 + 1) % map.tiles.len()),
                            Direction::Left => {
                                if pos.0 == 0 {
                                    Coord(map.tiles[pos.1].len(), pos.1)
                                } else {
                                    Coord(pos.0 - 1, pos.1)
                                }
                            }
                            Direction::Right => Coord((pos.0 + 1) % map.tiles[pos.1].len(), pos.1),
                        };

                        match map.get(pos) {
                            Tile::Wall => {
                                return;
                            }
                            Tile::Empty => {
                                self.pos = pos;
                                break;
                            }
                            Tile::Void => {
                                if map_mode == MapMode::Folded {
                                    panic!("shouldn't get a void when folding the map")
                                }
                                // if flat, continue loop and wrap around
                            }
                        }
                    }

                    distance -= 1;
                }
            }
            Movement::Turn(turn) => {
                self.facing = match self.facing {
                    Direction::Up => match turn {
                        Turn::Right => Direction::Right,
                        Turn::Left => Direction::Left,
                    },
                    Direction::Down => match turn {
                        Turn::Right => Direction::Left,
                        Turn::Left => Direction::Right,
                    },
                    Direction::Left => match turn {
                        Turn::Right => Direction::Up,
                        Turn::Left => Direction::Down,
                    },
                    Direction::Right => match turn {
                        Turn::Right => Direction::Down,
                        Turn::Left => Direction::Up,
                    },
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Movement {
    Move(usize),
    Turn(Turn),
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Turn::Left),
            "R" => Ok(Turn::Right),
            _ => Err(format!("invalid turn {s}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapMode {
    Flat,
    Folded,
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    row_ends: Vec<(usize, usize)>,
}

impl Map {
    fn get(&self, coord: Coord<usize>) -> Tile {
        self.tiles
            .get(coord.1)
            .and_then(|row| row.get(coord.0))
            .copied()
            .unwrap_or(Tile::Void)
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row_ends = Vec::new();
        let tiles = s
            .lines()
            .map(|line| {
                let mut start = usize::MAX;
                let mut end = usize::MIN;

                let row = line
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        let tile = Tile::try_from(c).unwrap();
                        if tile != Tile::Void {
                            start = start.min(i);
                            end = end.max(i);
                        }
                        tile
                    })
                    .collect();

                row_ends.push((start, end));

                row
            })
            .collect();

        Ok(Map { tiles, row_ends })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Void,
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            ' ' => Ok(Tile::Void),
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Wall),
            _ => Err(format!("invalid tile: {c}")),
        }
    }
}
