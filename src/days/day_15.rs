use std::{collections::HashSet, str::FromStr};

use crate::{
    coord::Coord,
    range::{Coverage, Range},
};

pub fn main(input: &str) -> (u64, u64) {
    let sensors: Vec<Sensor> = input
        .lines()
        .map(|line| Sensor::from_str(line).unwrap())
        .collect();

    let (coverage, beacons) = get_coverage(&sensors, 2000000);
    let cannot_contain = coverage.area_covered() as u64 - beacons.len() as u64;

    let can_contain = find_possible_position(&sensors, Range(0, 4000000)).unwrap();
    let tuning_frequency = (can_contain.0 * 4000000 + can_contain.1) as u64;

    (cannot_contain, tuning_frequency)
}

fn find_possible_position(sensors: &Vec<Sensor>, search_range: Range) -> Option<Coord<i64>> {
    for y in search_range.0..=search_range.1 {
        let (coverage, _) = get_coverage(&sensors, y);

        if let Some(bounds) = coverage.bounds() {
            if search_range.0 < bounds.0 {
                return Some(Coord(search_range.0, y));
            } else if search_range.1 > bounds.1 {
                return Some(Coord(search_range.1, y));
            }
        } else {
            return Some(Coord(search_range.0, y));
        }

        let gaps = coverage.gaps();
        if let Some(range) = gaps.ranges().first() {
            return Some(Coord(range.0, y));
        }
    }

    None
}

#[test]
fn day_15_examples_work() {
    let sensors: Vec<Sensor> = "
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"
    .trim()
    .lines()
    .map(|line| Sensor::from_str(line).unwrap())
    .collect();

    let (coverage, beacons) = get_coverage(&sensors, 10);
    let cannot_contain = coverage.area_covered() as u64 - beacons.len() as u64;
    assert_eq!(cannot_contain, 26);

    let can_contain = find_possible_position(&sensors, Range(0, 20)).unwrap();
    dbg!(can_contain);

    let tuning_frequency = (can_contain.0 * 4000000 + can_contain.1) as u64;

    assert_eq!(tuning_frequency, 56000011);
}

fn get_coverage(sensors: &Vec<Sensor>, y: i64) -> (Coverage, HashSet<Coord<i64>>) {
    let mut beacons: HashSet<Coord<i64>> = HashSet::new();
    let coverage = Coverage::new(
        sensors
            .iter()
            .filter_map(|sensor| {
                let md = sensor.beacon_distance();
                let width_delta = md - y.abs_diff(sensor.pos.1) as i64;
                if width_delta <= 0 {
                    None
                } else {
                    if sensor.closest_beacon.1 == y {
                        beacons.insert(sensor.closest_beacon);
                    }
                    Some(
                        Range::new(sensor.pos.0 - width_delta, sensor.pos.0 + width_delta).unwrap(),
                    )
                }
            })
            .collect(),
    );

    (coverage, beacons)
}

#[derive(Debug)]
struct Sensor {
    pos: Coord<i64>,
    closest_beacon: Coord<i64>,
}

impl Sensor {
    fn beacon_distance(&self) -> i64 {
        self.pos.manhattan_distance(self.closest_beacon)
    }

    #[allow(unused)]
    fn within_beacon_range(&self, target: Coord<i64>) -> bool {
        self.pos.manhattan_distance(target) <= self.beacon_distance()
    }
}

impl FromStr for Sensor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.trim().split(": ");

        Ok(Sensor {
            pos: {
                let mut bits = chunks
                    .next()
                    .ok_or(format!("input missing `:` separator: {s}"))?
                    .split_whitespace()
                    .skip(2);
                Coord(
                    bits.next()
                        .ok_or(format!("sensor missing x coordinate: {s}"))?
                        .replace(',', "")
                        .split('=')
                        .last()
                        .ok_or(format!("sensor missing x value: {s}"))?
                        .parse::<i64>()
                        .map_err(|e| format!("sensor invalid x value: {s} ({:?})", e))?,
                    bits.next()
                        .ok_or(format!("sensor missing y coordinate: {s}"))?
                        .split('=')
                        .last()
                        .ok_or(format!("sensor missing y value: {s}"))?
                        .parse::<i64>()
                        .map_err(|_| format!("sensor invalid y value: {s}"))?,
                )
            },
            closest_beacon: {
                let mut bits = chunks
                    .next()
                    .ok_or(format!("input missing `:` separator: {s}"))?
                    .split_whitespace()
                    .skip(4);
                Coord(
                    bits.next()
                        .ok_or(format!("beacon missing x coordinate: {s}"))?
                        .replace(',', "")
                        .split('=')
                        .last()
                        .ok_or(format!("beacon missing x value: {s}"))?
                        .parse::<i64>()
                        .map_err(|e| format!("beacon invalid x value: {s} ({:?})", e))?,
                    bits.next()
                        .ok_or(format!("beacon missing y coordinate: {s}"))?
                        .split('=')
                        .last()
                        .ok_or(format!("beacon missing y value: {s}"))?
                        .parse::<i64>()
                        .map_err(|_| format!("beacon invalid y value: {s}"))?,
                )
            },
        })
    }
}
