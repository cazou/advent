use std::str::FromStr;
use anyhow::Result;

enum Outcome {
    Loose,
    Draw,
    Win
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.trim() {
            "X" => Ok(Outcome::Loose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(s.to_string() + " is not a valid outcome")
        }
    }
}

enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    // What this move wins against the other move
    pub fn against(&self, other: &Move) -> usize {
        self.points() + self.win(&other)
    }

    fn points(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn win(&self, other: &Move) -> usize {
        match (self, other) {
            (Move::Rock, Move::Paper) | (Move::Paper, Move::Scissors) | (Move::Scissors, Move::Rock) => 0,
            (Move::Rock, Move::Rock) | (Move::Paper, Move::Paper) | (Move::Scissors, Move::Scissors) => 3,
            _ => 6
        }
    }

    fn to_play(&self, outcome: &Outcome) -> Move {
        match (self, outcome) {
            (Move::Rock, Outcome::Draw) => Move::Rock,
            (Move::Paper, Outcome::Draw) => Move::Paper,
            (Move::Scissors, Outcome::Draw) => Move::Scissors,
            (Move::Rock, Outcome::Loose) => Move::Scissors,
            (Move::Paper, Outcome::Loose) => Move::Rock,
            (Move::Scissors, Outcome::Loose) => Move::Paper,
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Scissors, Outcome::Win) => Move::Rock,
        }
    }

}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.trim() {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err(s.to_string() + " is not a valid move")
        }
    }
}

struct Strategy {
    rounds: Vec<(Move, Move)>
}

impl Strategy {
    fn play(&self) -> usize {
        let mut ret = 0;

        for (m1, m2) in &self.rounds {
            ret += m2.against(m1);
        }

        ret
    }
}

impl FromStr for Strategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rounds = vec![];
        for line in s.split("\n") {
            let (m1, m2) = line.split_at(1);
            let m1: Move = m1.parse()?;
            let outcome: Outcome = m2.parse()?;
            let m2 = m1.to_play(&outcome);

            rounds.push((m1, m2));
        }

        Ok(Strategy {
            rounds
        })
    }
}

pub fn run(input: &str) -> Result<()> {
    let strategy: Strategy = input.parse().unwrap();

    println!("Points: {}", strategy.play());

    Ok(())
}