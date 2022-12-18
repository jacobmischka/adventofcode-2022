use std::{
    collections::HashSet,
    ops::{Add, Sub},
    str::FromStr,
};

const SEARCH_SPACE: i32 = 22;

pub fn main(input: &str) -> (u32, u32) {
    let cubes: HashSet<Coords> = input
        .lines()
        .map(|line| {
            let cube = Coords::from_str(line).unwrap();

            cube
        })
        .collect();

    let mut exterior = HashSet::new();
    spread_outside(&cubes, &mut exterior, Coords { x: 0, y: 0, z: 0 });

    let (surface_area, external_surface_area) =
        cubes
            .iter()
            .fold((0, 0), |(mut surface, mut external), cube| {
                for c in cube.adjacent_coords() {
                    if !cubes.contains(&c) {
                        surface += 1;
                    }
                    if exterior.contains(&c) {
                        external += 1;
                    }
                }

                (surface, external)
            });

    (surface_area, external_surface_area)
}

fn spread_outside(cubes: &HashSet<Coords>, exterior: &mut HashSet<Coords>, pos: Coords) {
    for c in pos.adjacent_coords() {
        if !exterior.contains(&c)
            && !cubes.contains(&c)
            && c.x >= -1
            && c.x <= SEARCH_SPACE
            && c.y >= -1
            && c.y <= SEARCH_SPACE
            && c.z >= -1
            && c.z <= SEARCH_SPACE
        {
            exterior.insert(c);
            spread_outside(cubes, exterior, c);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    x: i32,
    y: i32,
    z: i32,
}

impl Coords {
    fn unit_coords() -> &'static [Coords; 6] {
        &[
            Coords { x: 1, y: 0, z: 0 },
            Coords { x: -1, y: 0, z: 0 },
            Coords { x: 0, y: 1, z: 0 },
            Coords { x: 0, y: -1, z: 0 },
            Coords { x: 0, y: 0, z: 1 },
            Coords { x: 0, y: 0, z: -1 },
        ]
    }

    fn adjacent_coords(self) -> [Coords; 6] {
        Coords::unit_coords().map(|c| self + c)
    }
}

impl Add for Coords {
    type Output = Coords;

    fn add(self, rhs: Self) -> Self::Output {
        Coords {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Coords {
    type Output = Coords;

    fn sub(self, rhs: Self) -> Self::Output {
        Coords {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl FromStr for Coords {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.split(',');
        Ok(Coords {
            x: chunks
                .next()
                .ok_or(format!("missing x coordinate: {s}"))?
                .parse()
                .map_err(|_| format!("invalid x coordinate: {s}"))?,
            y: chunks
                .next()
                .ok_or(format!("missing x coordinate: {s}"))?
                .parse()
                .map_err(|_| format!("invalid x coordinate: {s}"))?,
            z: chunks
                .next()
                .ok_or(format!("missing x coordinate: {s}"))?
                .parse()
                .map_err(|_| format!("invalid x coordinate: {s}"))?,
        })
    }
}
