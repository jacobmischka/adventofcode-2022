use std::{cmp::Ordering, vec::IntoIter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range(pub i64, pub i64);

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 {
            Ordering::Less
        } else if self.0 > other.0 {
            Ordering::Greater
        } else if self.1 < other.1 {
            Ordering::Less
        } else if self.1 > other.1 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Range {
    pub fn new(l: i64, r: i64) -> Result<Self, String> {
        if l > r {
            Err(format!("{l} is greater than {r}"))
        } else {
            Ok(Range(l, r))
        }
    }

    pub fn width(&self) -> i64 {
        self.1 - self.0 + 1
    }

    pub fn intersection(&self, other: Range) -> Option<Range> {
        let l = self.0.max(other.0);
        let r = self.1.min(other.1);

        Range::new(l, r).ok()
    }

    pub fn union(self, other: Range) -> (Range, Option<Range>) {
        let (l, r) = if self <= other {
            (self, other)
        } else {
            (other, self)
        };

        if l.0 <= r.0 && l.1 >= r.1 {
            (l, None)
        } else if l.1 >= r.0 && l.0 <= r.1 {
            (Range::new(l.0, r.1).unwrap(), None)
        } else {
            (l, Some(r))
        }
    }
}

#[test]
fn union_works() {
    assert_eq!(Range(-1, 12).union(Range(4, 20)), (Range(-1, 20), None));
    assert_eq!(Range(5, 20).union(Range(7, 10)), (Range(5, 20), None));

    assert_eq!(
        Range(0, 0).union(Range(1, 12)),
        (Range(0, 0), Some(Range(1, 12)))
    );
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Coverage {
    ranges: Vec<Range>,
}

impl IntoIterator for Coverage {
    type Item = Range;
    type IntoIter = IntoIter<Range>;

    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into_iter()
    }
}

impl Coverage {
    pub fn new(ranges: Vec<Range>) -> Self {
        let mut c = Self { ranges };

        c.flatten();

        c
    }

    pub fn ranges(&self) -> &Vec<Range> {
        &self.ranges
    }

    #[allow(unused)]
    pub fn contains(&self, other: Range) -> bool {
        self.ranges
            .iter()
            .any(|range| range.intersection(other).is_some())
    }

    pub fn bounds(&self) -> Option<Range> {
        if self.ranges.is_empty() {
            None
        } else {
            Some(Range::new(self.ranges[0].0, self.ranges.last().unwrap().1).unwrap())
        }
    }

    pub fn gaps(&self) -> Coverage {
        let mut ranges = Vec::new();

        for pair in self.ranges.windows(2) {
            ranges.push(Range::new(pair[0].1 + 1, pair[1].0 - 1).unwrap());
        }

        Coverage { ranges }
    }

    pub fn area_covered(&self) -> i64 {
        self.ranges.iter().map(|range| range.width()).sum()
    }

    pub fn flatten(&mut self) {
        if self.ranges.len() <= 1 {
            return;
        }

        self.ranges.sort();
        let mut flattened = Vec::new();

        for range in &self.ranges {
            Coverage::insert_to_flattened(&mut flattened, *range);
        }

        self.ranges = flattened;
    }

    fn insert_to_flattened(flattened: &mut Vec<Range>, range: Range) {
        let search_results = flattened.binary_search_by(|r| {
            if r.intersection(range).is_some() {
                Ordering::Equal
            } else {
                r.cmp(&range)
            }
        });

        match search_results {
            Ok(i) => {
                let existing = flattened.remove(i);
                let (union, should_be_none) = existing.union(range);
                assert!(should_be_none.is_none());
                Coverage::insert_to_flattened(flattened, union);
            }
            Err(i) => flattened.insert(i, range),
        }
    }
}

#[test]
fn flatten_works() {
    let mut c = Coverage {
        ranges: vec![Range(0, 2), Range(1, 10), Range(8, 20)],
    };
    c.flatten();

    assert_eq!(
        c,
        Coverage {
            ranges: vec![Range(0, 20)]
        }
    );

    let mut c = Coverage {
        ranges: vec![Range(0, 2), Range(1, 5), Range(8, 20)],
    };
    c.flatten();

    assert_eq!(
        c,
        Coverage {
            ranges: vec![Range(0, 5), Range(8, 20)]
        }
    );
}
