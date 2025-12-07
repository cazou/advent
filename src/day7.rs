use crate::traits::AdventOfCode;
use anyhow::Result;

pub struct Day7;

impl AdventOfCode for Day7 {
    fn day(&self) -> u8 {
        7
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let input = input.unwrap();
        let mut current_beams = vec![];
        let mut splits = 0;

        for (num, line) in input.lines().enumerate() {
            if num == 0 {
                for c in line.chars() {
                    if c == 'S' {
                        current_beams.push(true);
                    } else {
                        current_beams.push(false);
                    }
                }

                continue;
            }

            for (beam, action) in line.chars().enumerate() {
                match action {
                    '^' => {
                        if current_beams[beam] {
                            *current_beams.get_mut(beam - 1).unwrap() = true;
                            *current_beams.get_mut(beam).unwrap() = false;
                            *current_beams.get_mut(beam + 1).unwrap() = true;
                            splits += 1;
                        }
                    }
                    _ => continue,
                }
            }
        }

        Ok(splits.to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let input = input.unwrap();
        let mut current_beam_counts = vec![];

        for (num, line) in input.lines().enumerate() {
            if num == 0 {
                for c in line.chars() {
                    if c == 'S' {
                        current_beam_counts.push(1);
                    } else {
                        current_beam_counts.push(0);
                    }
                }

                continue;
            }

            for (beam, action) in line.chars().enumerate() {
                match action {
                    '^' => {
                        let beams = current_beam_counts[beam];
                        *current_beam_counts.get_mut(beam - 1).unwrap() += beams;
                        *current_beam_counts.get_mut(beam).unwrap() = 0;
                        *current_beam_counts.get_mut(beam + 1).unwrap() += beams;
                    }
                    _ => continue,
                }
            }
        }

        Ok(current_beam_counts.iter().sum::<usize>().to_string())
    }
}
