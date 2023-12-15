use crate::traits::AdventOfCode;
use anyhow::Result;
use std::str::FromStr;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn dist_to(&self, other: &Position) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

struct Universe {
    galaxies: Vec<Position>,
    width: usize,
    height: usize,
}

impl FromStr for Universe {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut galaxies = vec![];
        let mut width = 0;
        let mut height = 0;
        for (y, line) in s.lines().enumerate() {
            width = y + 1;
            for (x, space) in line.chars().enumerate() {
                if space == '#' {
                    galaxies.push(Position { x, y });
                }
                height = x + 1;
            }
        }

        Ok(Universe {
            galaxies,
            width,
            height,
        })
    }
}

impl Universe {
    pub fn expand(&mut self, expansion: usize) {
        let mut empty_cols = vec![];
        let mut empty_lines = vec![];

        for x in 0..self.width {
            if !self.galaxies.iter().any(|g| g.x == x) {
                empty_cols.push(x);
            }
        }
        for y in 0..self.height {
            if !self.galaxies.iter().any(|g| g.y == y) {
                empty_lines.push(y);
            }
        }

        let mut col_offset = 0;
        for col in empty_cols {
            self.width += expansion;
            self.galaxies = self
                .galaxies
                .iter()
                .map(|g| Position {
                    x: if g.x > col + col_offset {
                        g.x + expansion
                    } else {
                        g.x
                    },
                    y: g.y,
                })
                .collect();
            col_offset += expansion;
        }

        let mut line_offset = 0;
        for line in empty_lines {
            self.height += expansion;
            self.galaxies = self
                .galaxies
                .iter()
                .map(|g| Position {
                    x: g.x,
                    y: if g.y > line + line_offset {
                        g.y + expansion
                    } else {
                        g.y
                    },
                })
                .collect();
            line_offset += expansion;
        }
    }

    pub fn distances(&self) -> usize {
        let mut sum = 0;
        for g1 in &self.galaxies {
            for g2 in &self.galaxies {
                sum += g1.dist_to(g2);
            }
        }

        sum / 2
    }
}

pub struct Day11;

impl AdventOfCode for Day11 {
    fn day(&self) -> u8 {
        11
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let mut universe = input.unwrap().parse::<Universe>()?;
        universe.expand(1);
        Ok(universe.distances().to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut universe = input.unwrap().parse::<Universe>()?;
        universe.expand(999999);
        Ok(universe.distances().to_string())
    }
}
