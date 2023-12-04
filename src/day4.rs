use crate::traits::AdventOfCode;
use anyhow::Result;
use regex::Regex;
use std::str::FromStr;

struct Lottery {
    draws: Vec<Draw>,
}

impl Lottery {
    pub fn winnings(&self) -> usize {
        self.draws.iter().fold(0, |a, b| a + b.winnings())
    }

    pub fn winnings2(&mut self) -> usize {
        let mut copies: Vec<usize> = vec![1; self.draws.len()];
        for draw in &mut self.draws {
            let m = draw.matches();
            for i in 0..m {
                copies[draw.id + i] += copies[draw.id - 1];
            }
        }

        copies.iter().sum()
    }
}

impl FromStr for Lottery {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut draws = vec![];
        for line in s.lines() {
            if s.is_empty() {
                continue;
            }
            draws.push(line.parse::<Draw>()?);
        }

        Ok(Lottery { draws })
    }
}

struct Draw {
    id: usize,
    draw: Vec<usize>,
    bet: Vec<usize>,
    matches: Option<usize>,
}

impl Draw {
    pub fn winnings(&self) -> usize {
        self.bet
            .iter()
            .fold(1, |a, b| if self.draw.contains(b) { a * 2 } else { a })
            / 2
    }

    pub fn matches(&mut self) -> usize {
        if let Some(m) = self.matches {
            m
        } else {
            let m: usize = self
                .bet
                .iter()
                .fold(0, |a, b| if self.draw.contains(b) { a + 1 } else { a });
            self.matches = Some(m);
            m
        }
    }
}

impl FromStr for Draw {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let regex = Regex::new(r"^Card\s*(?<id>\d+): (?<draw>[^|]+)\|(?<bet>[^|]+)$")?;
        let caps = regex.captures(s).unwrap();

        Ok(Draw {
            matches: None,
            id: caps["id"].parse()?,
            draw: caps["draw"]
                .split_whitespace()
                .filter_map(|v| {
                    if v.trim().is_empty() {
                        None
                    } else {
                        Some(v.trim().parse::<usize>().unwrap())
                    }
                })
                .collect(),
            bet: caps["bet"]
                .split_whitespace()
                .filter_map(|v| {
                    if v.trim().is_empty() {
                        None
                    } else {
                        Some(v.trim().parse::<usize>().unwrap())
                    }
                })
                .collect(),
        })
    }
}

pub struct Day4;

impl AdventOfCode for Day4 {
    fn day(&self) -> u8 {
        4
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let lottery: Lottery = input.unwrap().parse()?;
        Ok(lottery.winnings().to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut lottery: Lottery = input.unwrap().parse()?;
        Ok(lottery.winnings2().to_string())
    }
}
