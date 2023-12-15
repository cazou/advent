use crate::traits::AdventOfCode;
use anyhow::Result;
use std::str::FromStr;

struct Report {
    histories: Vec<History>,
}

impl FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut histories = vec![];
        for line in s.lines() {
            histories.push(History {
                values: line.split(" ").map(|v| v.parse().unwrap()).collect(),
            });
        }

        Ok(Report { histories })
    }
}

impl Report {
    pub fn predictions(&self, direction: u8) -> isize {
        self.histories.iter().map(|h| h.predict(direction)).sum()
    }
}

struct History {
    values: Vec<isize>,
}

impl History {
    fn predict_rec(values: &[isize], direction: u8) -> isize {
        if values.iter().all(|v| *v == 0) {
            return 0;
        }

        let mut new_vec = vec![];
        for i in 0..values.len() - 1 {
            new_vec.push(values[i + 1] - values[i]);
        }

        if direction == 0 {
            Self::predict_rec(&new_vec, direction) + values.last().unwrap()
        } else {
            values.first().unwrap() - Self::predict_rec(&new_vec, direction)
        }
    }

    pub fn predict(&self, direction: u8) -> isize {
        Self::predict_rec(&self.values, direction)
    }
}

pub struct Day9;

impl AdventOfCode for Day9 {
    fn day(&self) -> u8 {
        9
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        Ok(input.unwrap().parse::<Report>()?.predictions(0).to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        Ok(input.unwrap().parse::<Report>()?.predictions(1).to_string())
    }
}
