use crate::traits::AdventOfCode;
use anyhow::Result;
use std::str::FromStr;

pub struct Day4;

struct Grid {
    rolls: Vec<Vec<u8>>,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut rolls = vec![];
        for line in s.lines() {
            rolls.push(line.chars().map(|r| if r == '@' { 1 } else { 0 }).collect());
        }

        Ok(Grid { rolls })
    }
}

impl Grid {
    fn width(&self) -> usize {
        self.rolls.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.rolls.len()
    }

    fn count_neighbors(&self, col: usize, row: usize) -> u8 {
        let mut neighbors = 0;

        if col > 0 {
            neighbors += self.rolls[row][col - 1];

            if row > 0 {
                neighbors += self.rolls[row - 1][col - 1];
            }
            if row < self.height() - 1 {
                neighbors += self.rolls[row + 1][col - 1];
            }
        }

        if col < self.width() - 1 {
            neighbors += self.rolls[row][col + 1];

            if row > 0 {
                neighbors += self.rolls[row - 1][col + 1];
            }
            if row < self.height() - 1 {
                neighbors += self.rolls[row + 1][col + 1];
            }
        }

        if row > 0 {
            neighbors += self.rolls[row - 1][col];
        }
        if row < self.height() - 1 {
            neighbors += self.rolls[row + 1][col];
        }

        neighbors
    }

    fn accessible_rolls(&self) -> usize {
        let mut rolls = 0;
        for (row, line) in self.rolls.iter().enumerate() {
            rolls = line
                .iter()
                .enumerate()
                .fold(rolls, |a, (col, r) |
                    if *r == 1 && self.count_neighbors(col, row) < 4 {
                        a+1
                    } else {
                        a
                    } as usize
                );
        }

        rolls
    }

    fn remove_accessible_rolls(&mut self) -> usize {
        let mut rolls = 0;
        let mut to_remove = vec![];

        for (row, line) in self.rolls.iter().enumerate() {
            for (col, r) in line.iter().enumerate() {
                if *r == 1 && self.count_neighbors(col, row) < 4 {
                    rolls += 1;
                    to_remove.push((col, row));
                }
            }
        }

        for (col, row) in to_remove {
            *self.rolls.get_mut(row).unwrap().get_mut(col).unwrap() = 0;
        }

        rolls
    }
}

impl AdventOfCode for Day4 {
    fn day(&self) -> u8 {
        4
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let grid: Grid = input.unwrap().parse().unwrap();
        Ok(grid.accessible_rolls().to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut grid: Grid = input.unwrap().parse().unwrap();
        let mut rolls = 0;

        loop {
            let removed = grid.remove_accessible_rolls();

            if removed == 0 {
                break;
            }

            rolls += removed;
        }

        Ok(rolls.to_string())
    }
}
