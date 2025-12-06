use crate::traits::AdventOfCode;
use anyhow::Result;
use std::str::FromStr;

pub struct Day6;

#[derive(Clone)]
enum Operation {
    Mul,
    Add,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Mul),
            _ => Err(()),
        }
    }
}
struct Math {
    cols: Vec<Vec<usize>>,
    ops: Vec<Operation>,
}

impl FromStr for Math {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut cols = vec![];
        let mut ops = vec![];
        let mut first_line = true;
        for line in s.lines() {
            if line.starts_with('+') {
                for op in line.split_whitespace() {
                    ops.push(op.parse().unwrap());
                }
                continue;
            }
            for (col, num) in line.split_whitespace().enumerate() {
                if first_line {
                    cols.push(vec![num.parse().unwrap()]);
                } else {
                    cols.get_mut(col).unwrap().push(num.parse().unwrap());
                }
            }
            first_line = false;
        }

        Ok(Math { cols, ops })
    }
}

impl Math {
    fn solve(&self) -> usize {
        let mut total = 0;
        for (col, values) in self.cols.iter().enumerate() {
            total += match self.ops[col] {
                Operation::Mul => values.iter().product::<usize>() as usize,
                Operation::Add => values.iter().sum::<usize>() as usize,
            }
        }

        total
    }
}

struct Block {
    vals: Vec<usize>,
    op: Option<Operation>,
}

struct CephalopodMath {
    problem: Vec<Block>,
}

impl FromStr for CephalopodMath {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ops = vec![];
        let line_count = s
            .lines()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .len();

        // Do the inversion
        let mut inverted_lines = vec![];
        for _ in 0..line_count {
            inverted_lines.push(vec![]);
        }
        for line in s.lines() {
            for (l, c) in line.chars().enumerate() {
                if let Ok(op) = c.to_string().parse::<Operation>() {
                    ops.push(op);
                    continue;
                }

                if c != ' ' {
                    inverted_lines.get_mut(l).unwrap().push(c);
                }
            }
        }

        // Parse the inversion
        let mut current_block = Block {
            vals: vec![],
            op: None,
        };
        let mut problem = vec![];
        for l in &inverted_lines {
            let line: String = l.iter().collect();

            if line.is_empty() {
                problem.push(current_block);
                current_block = Block {
                    vals: vec![],
                    op: None,
                };
                continue;
            }

            current_block.vals.push(line.parse().unwrap());
        }

        problem.push(current_block);

        for (block, op) in ops.iter().enumerate() {
            problem.get_mut(block).unwrap().op = Some(op.clone());
        }

        Ok(CephalopodMath { problem })
    }
}

impl CephalopodMath {
    fn solve(&self) -> usize {
        let mut total = 0;
        for p in &self.problem {
            total += match p.op.as_ref().unwrap() {
                Operation::Mul => p.vals.iter().product::<usize>() as usize,
                Operation::Add => p.vals.iter().sum::<usize>() as usize,
            }
        }

        total
    }
}

impl AdventOfCode for Day6 {
    fn day(&self) -> u8 {
        6
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let math: Math = input.unwrap().parse().unwrap();
        Ok(math.solve().to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let math: CephalopodMath = input.unwrap().parse().unwrap();
        Ok(math.solve().to_string())
    }
}
