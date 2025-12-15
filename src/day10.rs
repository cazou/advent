use crate::traits::AdventOfCode;
use anyhow::Result;
use regex::Regex;
use std::str::FromStr;

pub struct Day10;

#[derive(Debug)]
struct Machine {
    leds: u16,
    buttons: Vec<u16>,
    jbuttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl FromStr for Machine {
    type Err = ();

    //[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re = Regex::new(r"^\[(?<leds>[\.#]*)\] (?<buttons>.*) \{(?<joltages>.*)\}$").unwrap();
        let mut leds = 0;
        let mut buttons = vec![];
        let mut jbuttons = vec![];
        let mut joltages = vec![];

        let caps = re.captures(s).unwrap();
        for led in caps["leds"].chars().rev() {
            leds = leds << 1;
            if led == '#' {
                leds |= 1;
            }
        }

        for button in caps["buttons"].split(' ') {
            let mut switches = 0;
            let mut jswitches: Vec<usize> = caps["leds"].chars().map(|_| 0).collect();
            // (1,2,3)
            for c in button.chars() {
                let led_id: usize = match c.to_string().parse() {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                switches |= 1 << led_id;
                *jswitches.get_mut(led_id).unwrap() = 1;
            }

            buttons.push(switches);
            jbuttons.push(jswitches);
        }

        for j in caps["joltages"].split(',') {
            joltages.push(j.to_string().parse().unwrap());
        }

        Ok(Machine {
            leds,
            buttons,
            jbuttons,
            joltages,
        })
    }
}

impl Machine {
    fn run_n_pushes(&self, start: u16, buttons: &mut Vec<u16>, pushes: usize) -> bool {
        if pushes == 0 {
            return start == self.leds;
        }

        for i in 0..buttons.len() {
            let button = buttons.remove(i);
            let new_leds = start ^ button;
            if self.run_n_pushes(new_leds, buttons, pushes - 1) {
                return true;
            }
            buttons.insert(i, button);
        }

        false
    }

    fn find_min_pushes(&self) -> usize {
        let mut pushes = 1;
        let mut buttons = self.buttons.clone();

        while !self.run_n_pushes(0, &mut buttons, pushes) {
            pushes += 1;
        }

        pushes
    }

    /* Implementation completely based on
     * https://github.com/icub3d/advent-of-code/blob/main/aoc_2025/src/bin/day10.rs
     */
    fn find_min_joltage_pushes(&self) -> usize {
        let matrix = Matrix::from_machine(&self);

        // Now we can DFS over a much smaller solution space.
        let max = *self.joltages.iter().max().unwrap() + 1;
        let mut min = usize::MAX;
        let mut values = vec![0; matrix.independents.len()];

        dfs(&matrix, 0, &mut values, &mut min, max);

        min
    }
}

const EPSILON: f64 = 1e-9;

struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
    dependents: Vec<usize>,
    independents: Vec<usize>,
}

impl Matrix {
    // Make a matrix, do a Gaussian elimination and setup the fixed and free variables.
    fn from_machine(machine: &Machine) -> Self {
        let rows = machine.joltages.len();
        let cols = machine.jbuttons.len();
        let mut data = vec![vec![0.0; cols + 1]; rows];

        // Add all of our buttons.
        for (c, button) in machine.jbuttons.iter().enumerate() {
            for (r, v) in button.iter().enumerate() {
                data[r][c] = *v as f64;
            }
        }

        // Add our joltages to the last column
        for (r, &val) in machine.joltages.iter().enumerate() {
            data[r][cols] = val as f64;
        }

        let mut matrix = Self {
            data,
            rows,
            cols,
            dependents: Vec::new(),
            independents: Vec::new(),
        };

        matrix.gaussian_elimination();
        matrix
    }

    // https://en.wikipedia.org/wiki/Gaussian_elimination
    fn gaussian_elimination(&mut self) {
        let mut pivot = 0;

        let mut col = 0;
        while pivot < self.rows && col < self.cols {
            // Find the best pivot row for this column.
            let (best_row, best_value) = self
                .data
                .iter()
                .enumerate()
                .skip(pivot)
                .map(|(r, row)| (r, row[col].abs()))
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            // If the best value is zero, this is a free variable.
            if best_value < EPSILON {
                self.independents.push(col);
                col += 1;
                continue;
            }

            // Swap rows and mark this column as dependent.
            self.data.swap(pivot, best_row);
            self.dependents.push(col);

            // Normalize pivot row.
            let pivot_value = self.data[pivot][col];
            for val in &mut self.data[pivot][col..=self.cols] {
                *val /= pivot_value;
            }

            // Eliminate this column in all other rows.
            for r in 0..self.rows {
                if r != pivot {
                    let factor = self.data[r][col];
                    if factor.abs() > EPSILON {
                        let pivot_row = self.data[pivot][col..=self.cols].to_vec();
                        self.data[r][col..=self.cols]
                            .iter_mut()
                            .zip(&pivot_row)
                            .for_each(|(val, &pivot_val)| {
                                *val -= factor * pivot_val;
                            });
                    }
                }
            }

            pivot += 1;
            col += 1;
        }

        // Any remaining columns are free variables
        self.independents.extend(col..self.cols);
    }

    // Check if the given values for our independent variables are valid. If so, return the total button presses.
    fn valid(&self, values: &[usize]) -> Option<usize> {
        // We start with how many times we've pressed the free variables.
        let mut total = values.iter().sum::<usize>();

        // Calculate dependent variable values based on independent variables.
        for row in 0..self.dependents.len() {
            // Calculate this dependent by subtracting the sum of the free variable pushes from the solution.
            let val = self
                .independents
                .iter()
                .enumerate()
                .fold(self.data[row][self.cols], |acc, (i, &col)| {
                    acc - self.data[row][col] * (values[i] as f64)
                });

            // We need non-negative, whole numbers for a valid solution.
            if val < -EPSILON {
                return None;
            }
            let rounded = val.round();
            if (val - rounded).abs() > EPSILON {
                return None;
            }

            total += rounded as usize;
        }

        Some(total)
    }
}

fn dfs(matrix: &Matrix, idx: usize, values: &mut [usize], min: &mut usize, max: usize) {
    // When we've assigned all independent variables, check if it's a valid solution.
    if idx == matrix.independents.len() {
        if let Some(total) = matrix.valid(values) {
            *min = (*min).min(total);
        }
        return;
    }

    // Try different values for the current independent variable.
    let total: usize = values[..idx].iter().sum();
    for val in 0..max {
        // Optimization: If we ever go above our min, we can't possibly do better.
        if total + val >= *min {
            break;
        }
        values[idx] = val;
        dfs(matrix, idx + 1, values, min, max);
    }
}

impl AdventOfCode for Day10 {
    fn day(&self) -> u8 {
        10
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let mut pushes = 0;
        for line in input.unwrap().lines() {
            let machine: Machine = line.parse().unwrap();
            pushes += machine.find_min_pushes();
        }

        Ok(pushes.to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut pushes = 0;
        for line in input.unwrap().lines() {
            let machine: Machine = line.parse().unwrap();
            pushes += machine.find_min_joltage_pushes();
        }

        Ok(pushes.to_string())
    }
}
