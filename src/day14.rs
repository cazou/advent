use crate::traits::AdventOfCode;
use anyhow::Result;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

struct Cave {
    units: HashSet<[isize; 2]>,
    min_x: isize,
    max_x: isize,
    max_y: isize,
    unreachables: HashSet<[isize; 2]>,
}

impl Cave {
    // return Some(true) if the sand block is resting
    // return Some(false) if the sand block could fall one step
    // None if the sand would fall indefinitely
    fn step(&mut self, sand: &mut [isize; 2]) -> Option<bool> {
        // are we falling forever ?
        self.units.iter().find(|u| u[1] > sand[1])?;

        // try down
        if !self.units.contains(&[sand[0], sand[1] + 1]) {
            sand[1] += 1;
            return Some(false);
        }

        //try left-down
        if !self.units.contains(&[sand[0] - 1, sand[1] + 1]) {
            sand[0] -= 1;
            sand[1] += 1;
            return Some(false);
        }

        //try left-down
        if !self.units.contains(&[sand[0] + 1, sand[1] + 1]) {
            sand[0] += 1;
            sand[1] += 1;
            return Some(false);
        }

        Some(true)
    }

    pub fn simulate(&mut self) -> usize {
        let mut done = false;
        let mut resting = 0;

        while !done {
            let mut sand = [500, 0];
            loop {
                match self.step(&mut sand) {
                    None => {
                        done = true;
                        break;
                    }
                    Some(true) => {
                        resting += 1;
                        break;
                    }
                    Some(false) => {
                        continue;
                    }
                }
            }
            self.units.insert(sand);
        }

        resting
    }

    pub fn build_unreachables(&mut self) {
        for y in 1..self.max_y + 2 {
            for x in self.min_x..=self.max_x {
                // If it is a rock, continue
                if self.units.contains(&[x, y]) {
                    continue;
                }

                // Check if the 3 units above are either a rock or unreachable
                let rocks = self
                    .units
                    .iter()
                    .filter(|r| r[1] == y - 1 && (x - 1..=x + 1).contains(&r[0]))
                    .count();
                let unreachables = self
                    .unreachables
                    .iter()
                    .filter(|r| r[1] == y - 1 && (x - 1..=x + 1).contains(&r[0]))
                    .count();
                if rocks + unreachables == 3 {
                    self.unreachables.insert([x, y]);
                }
            }
        }
    }

    pub fn compute2(&mut self) -> usize {
        let rocks = self.units.len();
        ((self.max_y + 2) * (self.max_y + 2)) as usize - rocks - self.unreachables.len()
    }
}

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut units = HashSet::new();
        let (mut min_x, mut max_x, mut max_y) = (isize::MAX, 0, 0);
        for line in s.lines() {
            let mut last: Option<[isize; 2]> = None;
            for coord in line.split(" -> ") {
                let vals: Vec<isize> = coord.split(',').map(|s| s.parse().unwrap()).collect();
                if let Some(old) = last {
                    let curr = [vals[0], vals[1]];

                    let range_x = if curr[0] < old[0] {
                        curr[0]..=old[0]
                    } else {
                        old[0]..=curr[0]
                    };

                    let range_y = if curr[1] < old[1] {
                        curr[1]..=old[1]
                    } else {
                        old[1]..=curr[1]
                    };

                    for x in range_x {
                        for y in range_y.clone() {
                            units.insert([x, y]);
                            if x < min_x {
                                min_x = x;
                            }
                            if x > max_x {
                                max_x = x;
                            }
                            if y > max_y {
                                max_y = y;
                            }
                        }
                    }
                }

                last = Some([vals[0], vals[1]]);
            }
        }

        Ok(Cave {
            units,
            min_x,
            max_x,
            max_y,
            unreachables: HashSet::new(),
        })
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.max_y + 3 {
            for x in self.min_x - 10..=self.max_x + 10 {
                if x == 500 && y == 0 {
                    write!(f, "+")?;
                } else if self.units.contains(&[x, y]) {
                    write!(f, "#")?;
                } else if self.unreachables.contains(&[x, y]) {
                    write!(f, "X")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct Day14;

impl AdventOfCode for Day14 {
    fn day(&self) -> u8 {
        14
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let mut cave: Cave = input.unwrap().parse().unwrap();
        Ok(cave.simulate().to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut cave: Cave = input.unwrap().parse().unwrap();
        cave.build_unreachables();
        Ok(cave.compute2().to_string())
    }
}
