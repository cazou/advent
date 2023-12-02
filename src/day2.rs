use crate::traits::AdventOfCode;
use anyhow::Result;
use std::str::FromStr;

#[derive(Debug)]
struct Game {
    id: usize,
    reveals: Vec<[usize; 3]>,
}

impl Game {
    pub fn is_valid_game(&self) -> bool {
        let counts = [12, 13, 14];
        for run in &self.reveals {
            if counts[0] < run[0] || counts[1] < run[1] || counts[2] < run[2] {
                return false;
            }
        }

        true
    }
    pub fn minimum_power(&self) -> usize {
        let mut powers = [0; 3];
        for run in &self.reveals {
            if powers[0] < run[0] {
                powers[0] = run[0]
            }
            if powers[1] < run[1] {
                powers[1] = run[1]
            }
            if powers[2] < run[2] {
                powers[2] = run[2]
            }
        }

        powers.iter().fold(1, |a, b| a * b)
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let mut reveals = vec![];

        let split = s.split(":").collect::<Vec<&str>>();
        let id: usize = split[0].trim_start_matches("Game ").parse()?;

        for run in split[1].split(";") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            //3 blue, 4 red
            for cube in run.split(",") {
                // 3 blue
                let cube = cube.trim();
                match cube.split_once(" ").unwrap() {
                    (count, "red") => red += count.parse::<usize>()?,
                    (count, "green") => green += count.parse::<usize>()?,
                    (count, "blue") => blue += count.parse::<usize>()?,
                    _ => {}
                }
            }
            reveals.push([red, green, blue]);
        }

        Ok(Game { id, reveals })
    }
}

pub struct Day2;

impl AdventOfCode for Day2 {
    fn day(&self) -> u8 {
        2
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let mut sum = 0;
        for line in input.unwrap().lines() {
            if line.is_empty() {
                continue;
            }
            let game: Game = line.parse()?;
            if game.is_valid_game() {
                sum += game.id;
            }
        }

        Ok(sum.to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut sum = 0;
        for line in input.unwrap().lines() {
            if line.is_empty() {
                continue;
            }
            let game: Game = line.parse()?;
            sum += game.minimum_power();
        }
        Ok(sum.to_string())
    }
}
