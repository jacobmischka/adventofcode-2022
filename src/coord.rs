use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, Sub},
    str::FromStr,
};

use crate::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord<T>(pub T, pub T);

impl Coord<i32> {
    pub fn unit_difference(&self, other: Coord<i32>) -> Coord<i32> {
        Coord(
            match self.0.cmp(&other.0) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            },
            match self.1.cmp(&other.1) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            },
        )
    }
}

impl Coord<i64> {
    #[allow(unused)]
    pub fn distance(&self, other: Coord<i64>) -> f64 {
        let difference = *self - other;
        let f = ((difference.0 * difference.0) + (difference.1 * difference.1)) as f64;

        f.sqrt()
    }

    /// Works when directions increment up -> down, left -> right
    pub fn move_direction_udlr(self, direction: Direction, distance: i64) -> Coord<i64> {
        match direction {
            Direction::Up => Coord(self.0, self.1 - distance),
            Direction::Down => Coord(self.0, self.1 + distance),
            Direction::Left => Coord(self.0 - distance, self.1),
            Direction::Right => Coord(self.0 + distance, self.1),
        }
    }

    pub fn adjacent_coords_include_diag(self) -> [Self; 8] {
        [
            Coord(self.0, self.1 - 1),
            Coord(self.0 + 1, self.1 - 1),
            Coord(self.0 + 1, self.1),
            Coord(self.0 + 1, self.1 + 1),
            Coord(self.0, self.1 + 1),
            Coord(self.0 - 1, self.1 + 1),
            Coord(self.0 - 1, self.1),
            Coord(self.0 - 1, self.1 - 1),
        ]
    }
}

impl<T> Coord<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + PartialOrd
        + Ord
        + Debug
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + Default,
{
    pub fn manhattan_distance(&self, other: Coord<T>) -> T {
        let x_diff = if self.0 < other.0 {
            other.0 - self.0
        } else {
            self.0 - other.0
        };
        let y_diff = if self.1 < other.1 {
            other.1 - self.1
        } else {
            self.1 - other.1
        };
        x_diff + y_diff
    }
}

#[test]
fn manhattan_distance_works() {
    assert_eq!(Coord(0, 0).manhattan_distance(Coord(12, 0)), 12);
    assert_eq!(Coord(0, 0).manhattan_distance(Coord(0, 17)), 17);
    assert_eq!(Coord(0, 0).manhattan_distance(Coord(5, 7)), 12);
    assert_eq!(Coord(0, 0).manhattan_distance(Coord(-5, 7)), 12);
}

impl<T> From<(T, T)> for Coord<T>
where
    T: Debug + Clone + Copy + PartialEq + Eq + Hash,
{
    fn from((x, y): (T, T)) -> Self {
        Coord(x, y)
    }
}

impl<T> Sub for Coord<T>
where
    T: Sub<Output = T> + Debug + Clone + Copy + PartialEq + Eq + Hash,
{
    type Output = Coord<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T> Add for Coord<T>
where
    T: Add<Output = T> + Debug + Clone + Copy + PartialEq + Eq + Hash,
{
    type Output = Coord<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> FromStr for Coord<T>
where
    T: FromStr + Debug + Clone + Copy + PartialEq + Eq + Hash,
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(',');

        Ok(Coord(
            T::from_str(
                pieces
                    .next()
                    .ok_or(format!("missing first piece of coordinate: {s}"))?
                    .trim(),
            )
            .map_err(|_| format!("invalid first piece of coordinate: {s}"))?,
            T::from_str(
                pieces
                    .next()
                    .ok_or(format!("missing second piece of coordinate: {s}"))?
                    .trim(),
            )
            .map_err(|_| format!("invalid second piece of coordinate: {s}"))?,
        ))
    }
}
