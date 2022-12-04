use std::str::FromStr;

pub fn main(input: &str) -> (u32, u32) {
    let mut contained = 0;
    let mut overlaps = 0;
    for line in input.lines() {
        let mut assignments = line.split(',').map(RoomAssignment::from_str);
        let ass1 = assignments.next().unwrap().unwrap();
        let ass2 = assignments.next().unwrap().unwrap();
        if ass1.contains(&ass2) || ass2.contains(&ass1) {
            contained += 1;
        }
        if ass1.overlaps(&ass2) {
            overlaps += 1;
        }
    }

    (contained, overlaps)
}

struct RoomAssignment(u32, u32);

impl RoomAssignment {
    fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.0 <= other.1 && self.1 >= other.0
    }
}

impl FromStr for RoomAssignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split('-').map(u32::from_str);
        Ok(Self(
            pieces.next().unwrap().unwrap(),
            pieces.next().unwrap().unwrap(),
        ))
    }
}
