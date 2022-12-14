use std::{
    fmt::Debug,
    hash::Hash,
    io::{self, Read},
    ops::{Add, Sub},
    str::FromStr,
};

pub mod days;

pub fn get_input() -> io::Result<String> {
    let mut s = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut s)?;
    Ok(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord<T>(T, T);

impl Coord<i32> {
    fn unit_difference(&self, other: Coord<i32>) -> Coord<i32> {
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
