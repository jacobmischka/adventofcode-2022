use std::str::FromStr;

use crate::{coord::Coord, direction::Direction};

/*

This will only work for this input variant:
https://preview.redd.it/jw5h94vwlo7a1.png?width=448&format=png&auto=webp&s=462d8eddb12c611e412129d709f837ced768d692

        c    d

       11112222
       11112222
     b 11112222 g
       11112222
       3333
     a 3333 h
    a  3333
       3333
   44445555
 b 44445555 g
   44445555
   44445555
   6666
 c 6666 f
   6666
   6666

     d
*/

pub fn main(input: &str) -> (u32, u32) {
    let mut chunks = input.split("\n\n");
    let map = Map::from_str(chunks.next().unwrap()).unwrap();

    let path = chunks.next().unwrap().trim();
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
                    let mut facing = self.facing;

                    loop {
                        pos = match facing {
                            Direction::Up => {
                                if pos.1 <= map.col_ends[pos.0].0 {
                                    match map_mode {
                                        MapMode::Flat => Coord(pos.0, map.col_ends[pos.0].1),
                                        MapMode::Cube => {
                                            todo!()
                                        }
                                    }
                                } else {
                                    Coord(pos.0, pos.1 - 1)
                                }
                            }
                            Direction::Down => {
                                if pos.1 >= map.col_ends[pos.0].1 {
                                    match map_mode {
                                        MapMode::Flat => Coord(pos.0, map.col_ends[pos.0].0),
                                        MapMode::Cube => {
                                            todo!()
                                        }
                                    }
                                } else {
                                    Coord(pos.0, pos.1 + 1)
                                }
                            }
                            Direction::Left => {
                                if pos.0 <= map.row_ends[pos.1].0 {
                                    match map_mode {
                                        MapMode::Flat => Coord(map.row_ends[pos.1].1, pos.1),
                                        MapMode::Cube => {
                                            todo!()
                                        }
                                    }
                                } else {
                                    Coord(pos.0 - 1, pos.1)
                                }
                            }
                            Direction::Right => {
                                if pos.0 >= map.row_ends[pos.1].1 {
                                    match map_mode {
                                        MapMode::Flat => Coord(map.row_ends[pos.1].0, pos.1),
                                        MapMode::Cube => {
                                            todo!()
                                        }
                                    }
                                } else {
                                    Coord(pos.0 + 1, pos.1)
                                }
                            }
                        };

                        match map.get(pos) {
                            Tile::Wall => {
                                return;
                            }
                            Tile::Empty => {
                                self.pos = pos;
                                self.facing = facing;
                                break;
                            }
                            Tile::Void => {
                                if map_mode == MapMode::Cube {
                                    unreachable!("no voids in a cube!")
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
    Cube,
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    row_ends: Vec<(usize, usize)>,
    col_ends: Vec<(usize, usize)>,
    cube_width: usize,
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
        let mut num_tiles = 0;
        let mut row_ends = Vec::new();
        let mut col_ends = Vec::new();
        let tiles = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                let mut start = usize::MAX;
                let mut end = usize::MIN;

                let row = line
                    .chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if col_ends.len() <= x {
                            col_ends.push((usize::MAX, usize::MIN));
                        }

                        let tile = Tile::try_from(c).unwrap();
                        if tile != Tile::Void {
                            start = start.min(x);
                            end = end.max(x);
                            col_ends[x].0 = col_ends[x].0.min(y);
                            col_ends[x].1 = col_ends[x].1.max(y);
                            num_tiles += 1;
                        }
                        tile
                    })
                    .collect();

                row_ends.push((start, end));

                row
            })
            .collect();

        Ok(Map {
            tiles,
            row_ends,
            col_ends,
            cube_width: num_tiles / 6,
        })
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
