use crate::day9::Move::{Down, Right, Up};
use crate::traits::AdventOfCode;
use anyhow::{bail, ensure, Result};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::str::FromStr;

/*
(0,0) is on the bottom left.
 */
enum Move {
    Up(i32),
    Left(i32),
    Right(i32),
    Down(i32),
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let dist: i32 = s.split_at(2).1.parse().unwrap();
        match s.chars().next().unwrap() {
            'U' => Ok(Up(dist)),
            'D' => Ok(Down(dist)),
            'L' => Ok(Move::Left(dist)),
            'R' => Ok(Right(dist)),
            _ => bail!("Invalid move".to_string()),
        }
    }
}

impl Move {
    fn move_next_knot(head: &[i32; 2], tail: &mut [i32; 2]) {
        if tail[0] != head[0] && tail[1] != head[1] {
            if (tail[0] - head[0]).abs() > 1 || (tail[1] - head[1]).abs() > 1 {
                // Move diagonally
                if (head[0] - tail[0]).is_negative() {
                    tail[0] -= 1;
                } else {
                    tail[0] += 1;
                }

                if (head[1] - tail[1]).is_negative() {
                    tail[1] -= 1;
                } else {
                    tail[1] += 1;
                }
            }
        } else if (tail[0] - head[0]).abs() > 1 {
            if (head[0] - tail[0]).is_negative() {
                tail[0] -= 1;
            } else {
                tail[0] += 1;
            }
        } else if (tail[1] - head[1]).abs() > 1 {
            if (head[1] - tail[1]).is_negative() {
                tail[1] -= 1;
            } else {
                tail[1] += 1;
            }
        }
    }

    pub fn apply(
        &self,
        head: &Rc<RefCell<[i32; 2]>>,
        tail: &Rc<RefCell<[i32; 2]>>,
        rope: &[Rc<RefCell<[i32; 2]>>],
    ) -> Vec<[i32; 2]> {
        let (idx, i, dist) = match self {
            Right(d) => (0, 1, *d),
            Move::Left(d) => (0, -1, *d),
            Up(d) => (1, 1, *d),
            Down(d) => (1, -1, *d),
        };
        let mut positions = vec![*tail.borrow()];

        for _ in 0..dist {
            head.borrow_mut()[idx] += i;

            let mut prev = Rc::clone(head);
            for knot in rope {
                Move::move_next_knot(&prev.borrow(), &mut knot.borrow_mut());
                prev = Rc::clone(knot);
            }
            Move::move_next_knot(&prev.borrow(), &mut tail.borrow_mut());

            positions.push(*tail.borrow());
        }

        positions
    }
}

fn run(input: &str, knot_count: usize) -> Result<String> {
    ensure!(
        knot_count >= 2,
        format!(
            "The rope must have at least 2 knots ({} configured)",
            knot_count
        )
    );
    let mut rope = vec![];
    let head = Rc::new(RefCell::new([0; 2]));
    let tail = Rc::new(RefCell::new([0; 2]));
    let mut positions = HashSet::new();

    for _ in 0..knot_count - 2 {
        rope.push(Rc::new(RefCell::new([0; 2])));
    }

    for line in input.lines() {
        let m: Move = line.parse()?;
        let pos_list = m.apply(&head, &tail, &rope);
        for p in pos_list {
            positions.insert(p);
        }
    }

    Ok(positions.len().to_string())
}

pub struct Day9;

impl AdventOfCode for Day9 {
    fn day(&self) -> u8 {
        9
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        run(&input.unwrap(), 2)
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        run(&input.unwrap(), 10)
    }
}
