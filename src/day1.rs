use crate::traits::AdventOfCode;

pub struct Day1;

impl AdventOfCode for Day1 {
    fn day(&self) -> u8 {
        1
    }

    fn run1(&mut self, input: Option<String>) -> anyhow::Result<String> {
        let mut pos = 50;
        let mut password = 0;
        for line in input.unwrap().lines() {
            let (op, count) = line.split_at(1);
            let count: i32 = count.parse().unwrap();
            match op {
                "R" => pos = (pos + count) % 100,
                "L" => pos = (pos - count) % 100,
                _ => continue,
            }
            if pos == 0 {
                password += 1;
            }
        }

        Ok(password.to_string())
    }

    fn run2(&mut self, input: Option<String>) -> anyhow::Result<String> {
        let mut pos = 50;
        let mut password = 0;
        for line in input.unwrap().lines() {
            let last_pos = pos;
            let (op, count) = line.split_at(1);
            let mut count: i32 = count.parse().unwrap();

            password += count / 100;
            count = count % 100;
            if count == 0 {
                continue;
            }

            match op {
                "R" => pos = pos + count,
                "L" => pos = pos - count,
                _ => continue,
            }

            if pos >= 100 {
                password += 1;
            }
            if last_pos > 0 && pos <= 0 {
                password += 1;
            }

            pos = pos % 100;
            if pos < 0 {
                pos += 100;
            }
        }

        Ok(password.to_string())
    }
}
