use std::{fmt::Display, str::FromStr};

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
     a 3333 e
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
    let mut folded_you = flat_you.clone();

    for movement in &movements {
        flat_you.do_movement(*movement, &map, MapMode::Flat);
        folded_you.do_movement(*movement, &map, MapMode::Cube);
    }

    (flat_you.get_password(), folded_you.get_password())
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
                                        MapMode::Cube => match pos.0 / map.cube_width {
                                            0 => {
                                                facing = Direction::Right;
                                                let y = map.cube_width + pos.0;
                                                Coord(map.row_ends[y].0, y)
                                            }
                                            1 => {
                                                facing = Direction::Right;
                                                let y = 3 * map.cube_width + pos.0 % map.cube_width;
                                                Coord(map.row_ends[y].0, y)
                                            }
                                            2 => {
                                                facing = Direction::Up;
                                                let x = pos.0 % map.cube_width;
                                                Coord(x, map.col_ends[x].1)
                                            }
                                            _ => unreachable!("{:?} {:?}", pos, facing),
                                        },
                                    }
                                } else {
                                    Coord(pos.0, pos.1 - 1)
                                }
                            }
                            Direction::Down => {
                                if pos.1 >= map.col_ends[pos.0].1 {
                                    match map_mode {
                                        MapMode::Flat => Coord(pos.0, map.col_ends[pos.0].0),
                                        MapMode::Cube => match pos.0 / map.cube_width {
                                            0 => {
                                                facing = Direction::Down;
                                                let x = 2 * map.cube_width + pos.0;
                                                Coord(x, map.col_ends[x].0)
                                            }
                                            1 => {
                                                facing = Direction::Left;
                                                let y = 3 * map.cube_width + pos.0 % map.cube_width;
                                                Coord(map.row_ends[y].1, y)
                                            }
                                            2 => {
                                                facing = Direction::Left;
                                                let y = map.cube_width + pos.0 % map.cube_width;
                                                Coord(map.row_ends[y].1, y)
                                            }
                                            _ => unreachable!("{:?} {:?}", pos, facing),
                                        },
                                    }
                                } else {
                                    Coord(pos.0, pos.1 + 1)
                                }
                            }
                            Direction::Left => {
                                if pos.0 <= map.row_ends[pos.1].0 {
                                    match map_mode {
                                        MapMode::Flat => Coord(map.row_ends[pos.1].1, pos.1),
                                        MapMode::Cube => match pos.1 / map.cube_width {
                                            0 => {
                                                facing = Direction::Right;
                                                let y = 3 * map.cube_width - pos.1 - 1;
                                                Coord(map.row_ends[y].0, y)
                                            }
                                            1 => {
                                                facing = Direction::Down;
                                                let x = pos.1 % map.cube_width;
                                                Coord(x, map.col_ends[x].0)
                                            }
                                            2 => {
                                                facing = Direction::Right;
                                                let y = map.cube_width - pos.1 % map.cube_width - 1;
                                                Coord(map.row_ends[y].0, y)
                                            }
                                            3 => {
                                                facing = Direction::Down;
                                                let x = map.cube_width + pos.1 % map.cube_width;
                                                Coord(x, map.col_ends[x].0)
                                            }
                                            _ => unreachable!("{:?} {:?}", pos, facing),
                                        },
                                    }
                                } else {
                                    Coord(pos.0 - 1, pos.1)
                                }
                            }
                            Direction::Right => {
                                if pos.0 >= map.row_ends[pos.1].1 {
                                    match map_mode {
                                        MapMode::Flat => Coord(map.row_ends[pos.1].0, pos.1),
                                        MapMode::Cube => match pos.1 / map.cube_width {
                                            0 => {
                                                facing = Direction::Left;
                                                let y = 3 * map.cube_width - pos.1 - 1;
                                                Coord(map.row_ends[y].1, y)
                                            }
                                            1 => {
                                                facing = Direction::Up;
                                                let x = 2 * map.cube_width + pos.1 % map.cube_width;
                                                Coord(x, map.col_ends[x].1)
                                            }
                                            2 => {
                                                facing = Direction::Left;
                                                let y = map.cube_width - pos.1 % map.cube_width - 1;
                                                Coord(map.row_ends[y].1, y)
                                            }
                                            3 => {
                                                facing = Direction::Up;
                                                let x = map.cube_width + pos.1 % map.cube_width;
                                                Coord(x, map.col_ends[x].1)
                                            }
                                            _ => unreachable!("{:?} {:?}", pos, facing),
                                        },
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

    #[allow(unused)]
    fn dump(&self, pos: &Position) {
        print!("\t");
        for x in 0..self.tiles[0].len() {
            print!("{}", x / 100);
        }
        println!();
        print!("\t");
        for x in 0..self.tiles[0].len() {
            print!("{}", (x % 100) / 10);
        }
        println!();
        print!("\t");
        for x in 0..self.tiles[0].len() {
            print!("{}", x % 10);
        }
        println!("\n");

        for (y, row) in self.tiles.iter().enumerate() {
            print!("{}", y / 100);
            print!("{}", (y % 100) / 10);
            print!("{}\t", (y % 10));

            for (x, tile) in row.iter().enumerate() {
                if pos.pos == Coord(x, y) {
                    print!("{}", pos.facing);
                } else {
                    print!("{}", tile);
                }
            }
            println!();
        }
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
            cube_width: ((num_tiles / 6) as f64).sqrt() as usize,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Void,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Wall => write!(f, "#"),
            Tile::Void => write!(f, " "),
        }
    }
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

#[test]
fn wrapping_works() {
    let map = Map::from_str(
        "    ........
    ........
    ........
    ........
    ....
    ....
    ....
    ....
........
........
........
........
....
....
....
....",
    )
    .unwrap();

    // a
    let mut pos = Position {
        pos: Coord(4, 6),
        facing: Direction::Left,
    };
    let start = pos.clone();
    pos.do_movement(Movement::Move(1), &map, MapMode::Cube);

    assert_eq!(
        pos,
        Position {
            pos: Coord(2, 8),
            facing: Direction::Down,
        },
    );

    pos.do_movement(Movement::Turn(Turn::Left), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Left), &map, MapMode::Cube);
    pos.do_movement(Movement::Move(1), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Right), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Right), &map, MapMode::Cube);

    assert_eq!(pos, start);

    // b
    let mut pos = Position {
        pos: Coord(4, 2),
        facing: Direction::Left,
    };
    let start = pos.clone();
    pos.do_movement(Movement::Move(1), &map, MapMode::Cube);

    assert_eq!(
        pos,
        Position {
            pos: Coord(0, 9),
            facing: Direction::Right,
        },
    );

    pos.do_movement(Movement::Turn(Turn::Left), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Left), &map, MapMode::Cube);
    pos.do_movement(Movement::Move(1), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Right), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Right), &map, MapMode::Cube);

    assert_eq!(pos, start);

    // c
    let mut pos = Position {
        pos: Coord(5, 0),
        facing: Direction::Up,
    };
    let start = pos.clone();
    pos.do_movement(Movement::Move(1), &map, MapMode::Cube);

    assert_eq!(
        pos,
        Position {
            pos: Coord(0, 13),
            facing: Direction::Right,
        },
    );

    pos.do_movement(Movement::Turn(Turn::Left), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Left), &map, MapMode::Cube);
    pos.do_movement(Movement::Move(1), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Right), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Right), &map, MapMode::Cube);

    assert_eq!(pos, start);

    // d
    let mut pos = Position {
        pos: Coord(10, 0),
        facing: Direction::Up,
    };
    let start = pos.clone();
    pos.do_movement(Movement::Move(1), &map, MapMode::Cube);

    assert_eq!(
        pos,
        Position {
            pos: Coord(2, 15),
            facing: Direction::Up,
        },
    );

    pos.do_movement(Movement::Turn(Turn::Left), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Left), &map, MapMode::Cube);
    pos.do_movement(Movement::Move(1), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Right), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Right), &map, MapMode::Cube);

    assert_eq!(pos, start);

    // e
    let mut pos = Position {
        pos: Coord(7, 5),
        facing: Direction::Right,
    };
    let start = pos.clone();
    pos.do_movement(Movement::Move(1), &map, MapMode::Cube);

    assert_eq!(
        pos,
        Position {
            pos: Coord(9, 3),
            facing: Direction::Up,
        },
    );

    pos.do_movement(Movement::Turn(Turn::Left), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Left), &map, MapMode::Cube);
    pos.do_movement(Movement::Move(1), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Right), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Right), &map, MapMode::Cube);

    assert_eq!(pos, start);

    // f
    let mut pos = Position {
        pos: Coord(3, 13),
        facing: Direction::Right,
    };
    let start = pos.clone();
    pos.do_movement(Movement::Move(1), &map, MapMode::Cube);

    assert_eq!(
        pos,
        Position {
            pos: Coord(5, 11),
            facing: Direction::Up,
        },
    );

    pos.do_movement(Movement::Turn(Turn::Left), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Left), &map, MapMode::Cube);
    pos.do_movement(Movement::Move(1), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Right), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Right), &map, MapMode::Cube);

    assert_eq!(pos, start);

    // g
    let mut pos = Position {
        pos: Coord(7, 9),
        facing: Direction::Right,
    };
    let start = pos.clone();
    pos.do_movement(Movement::Move(1), &map, MapMode::Cube);

    assert_eq!(
        pos,
        Position {
            pos: Coord(11, 2),
            facing: Direction::Left,
        },
    );

    pos.do_movement(Movement::Turn(Turn::Left), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Left), &map, MapMode::Cube);
    pos.do_movement(Movement::Move(1), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Right), &map, MapMode::Cube);
    pos.do_movement(Movement::Turn(Turn::Right), &map, MapMode::Cube);

    assert_eq!(pos, start);
}
