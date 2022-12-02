use std::str::FromStr;

pub fn main(input: &str) -> (u32, u32) {
    let mut score1 = 0;
    let mut score2 = 0;
    for line in input.lines() {
        score1 += score_round_naive(&line);
        score2 += score_round_advanced(&line);
    }

    (score1, score2)
}

fn score_round_naive(line: &str) -> u32 {
    let moves: Vec<Move> = line
        .split_whitespace()
        .map(|s| Move::from_str(s).unwrap())
        .collect();

    let them = &moves[0];
    let you = &moves[1];

    you.selection_score() + you.outcome(&them).score()
}

fn score_round_advanced(line: &str) -> u32 {
    let mut directions = line.split_whitespace();
    let them = Move::from_str(directions.next().unwrap()).unwrap();
    let outcome = Outcome::from_str(directions.next().unwrap()).unwrap();
    let you = Move::from_outcome(&them, &outcome);
    you.selection_score() + outcome.score()
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(format!("Invalid outcome: {s}")),
        }
    }
}

impl Move {
    fn selection_score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn from_outcome(other: &Move, outcome: &Outcome) -> Self {
        use Move::*;
        use Outcome::*;

        match (other, outcome) {
            (Rock, Lose) => Scissors,
            (Rock, Draw) => Rock,
            (Rock, Win) => Paper,
            (Paper, Lose) => Rock,
            (Paper, Draw) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Lose) => Paper,
            (Scissors, Draw) => Scissors,
            (Scissors, Win) => Rock,
        }
    }

    fn outcome(&self, other: &Move) -> Outcome {
        use Move::*;
        use Outcome::*;

        match (self, other) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Lose,
            (Scissors, Rock) => Lose,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }

    // fn from_outcome(other: &Move)
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(format!("Invalid move: {s}")),
        }
    }
}
