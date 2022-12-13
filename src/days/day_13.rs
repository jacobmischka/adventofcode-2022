use std::{cmp::Ordering, iter::Peekable, str::FromStr};

pub fn main(input: &str) -> (u32, u32) {
    let mut sum = 0;
    let dividers = vec![
        PacketPiece::List(vec![PacketPiece::Number(2)]),
        PacketPiece::List(vec![PacketPiece::Number(6)]),
    ];
    let mut packets: Vec<PacketPiece> = dividers.clone();
    for (i, pair) in input.split("\n\n").enumerate() {
        let index = i + 1;
        let mut lines = pair.lines();
        let left = PacketPiece::from_str(lines.next().unwrap()).unwrap();
        let right = PacketPiece::from_str(lines.next().unwrap()).unwrap();

        if left <= right {
            sum += index;
        }
        packets.push(left);
        packets.push(right);
    }

    let mut decoder_key = 1;

    packets.sort();
    for (i, packet) in packets.iter().enumerate() {
        if dividers.contains(packet) {
            decoder_key *= i + 1;
        }
    }

    (sum as _, decoder_key as _)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketPiece {
    List(Vec<PacketPiece>),
    Number(u32),
}

impl PacketPiece {
    fn from_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self, String>
    where
        I: Iterator<Item = &'a str>,
    {
        let first = tokens.next().unwrap();

        match first {
            "[" => {
                let mut list: Vec<PacketPiece> = Vec::new();

                loop {
                    if tokens.peek() == Some(&"") {
                        let _ = tokens.next();
                    }

                    if *tokens.peek().unwrap() == "]" {
                        let _ = tokens.next();
                        break;
                    } else {
                        list.push(PacketPiece::from_tokens(tokens)?);
                    }
                }

                Ok(PacketPiece::List(list))
            }
            c => {
                let val: u32 = c.parse().map_err(|_| format!("invalid value: {c}"))?;
                Ok(PacketPiece::Number(val))
            }
        }
    }
}

impl Ord for PacketPiece {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketPiece::Number(lhs), PacketPiece::Number(rhs)) => lhs.cmp(rhs),
            (PacketPiece::List(lhs), PacketPiece::List(rhs)) => {
                if lhs == rhs {
                    return Ordering::Equal;
                }

                let end = lhs.len().max(rhs.len());

                let mut i = 0;
                while i < end {
                    if i >= lhs.len() {
                        return Ordering::Less;
                    }

                    if i >= rhs.len() {
                        return Ordering::Greater;
                    }

                    if lhs[i] > rhs[i] {
                        return Ordering::Greater;
                    } else if lhs[i] < rhs[i] {
                        return Ordering::Less;
                    }

                    i += 1;
                }

                Ordering::Equal
            }
            (PacketPiece::Number(lhs), PacketPiece::List(_)) => {
                PacketPiece::List(vec![PacketPiece::Number(*lhs)]).cmp(other)
            }
            (PacketPiece::List(_), PacketPiece::Number(rhs)) => {
                self.cmp(&PacketPiece::List(vec![PacketPiece::Number(*rhs)]))
            }
        }
    }
}

impl PartialOrd for PacketPiece {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for PacketPiece {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // very hacky way to tokenize, probably a better way
        let new = s.replace("[", "[,").replace("]", ",]");
        let mut tokens = new.split(',').peekable();

        PacketPiece::from_tokens(&mut tokens)
    }
}
