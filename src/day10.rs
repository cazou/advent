// Disclaimer: Probably one of my worst piece of code ever

use crate::traits::AdventOfCode;
use anyhow::{bail, Result};
use std::str::FromStr;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Orientation {
    North,
    South,
    East,
    West,
}

impl Orientation {
    pub fn connects_to(&self, other: Orientation) -> bool {
        match self {
            Orientation::North => other == Orientation::South,
            Orientation::South => other == Orientation::North,
            Orientation::East => other == Orientation::West,
            Orientation::West => other == Orientation::East,
        }
    }
}

#[derive(Debug)]
struct Pipe {
    link1: Orientation,
    link2: Orientation,
    visited: u8,
    direction: i8, // +1/-1
}

impl Pipe {
    pub fn can_connect_to(&self, other: &Pipe) -> bool {
        self.link1.connects_to(other.link1)
            || self.link1.connects_to(other.link2)
            || self.link2.connects_to(other.link1)
            || self.link2.connects_to(other.link2)
    }

    pub fn is_vertical(&self) -> bool {
        !(self.link1 == Orientation::East && self.link2 == Orientation::West
            || self.link2 == Orientation::West && self.link1 == Orientation::East)
    }
}

#[derive(Debug)]
enum MazePiece {
    Ground(u8),
    Start,
    Pipe(Pipe),
}

impl MazePiece {
    pub fn visited(&self) -> u8 {
        match self {
            MazePiece::Pipe(p) => p.visited,
            MazePiece::Ground(v) => *v,
            _ => 0,
        }
    }

    pub fn set_visited(&mut self, visited: u8) {
        match self {
            MazePiece::Pipe(p) => p.visited = visited,
            MazePiece::Ground(v) => *v = visited,
            _ => {}
        };
    }

    pub fn inc_visited(&mut self) {
        match self {
            MazePiece::Pipe(p) => p.visited += 1,
            MazePiece::Ground(v) => *v += 1,
            _ => {}
        };
    }
}

impl FromStr for MazePiece {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "|" => Ok(MazePiece::Pipe(Pipe {
                visited: 0,
                link1: Orientation::North,
                link2: Orientation::South,
                direction: 0,
            })),
            "-" => Ok(MazePiece::Pipe(Pipe {
                visited: 0,
                link1: Orientation::East,
                link2: Orientation::West,
                direction: 0,
            })),
            "L" => Ok(MazePiece::Pipe(Pipe {
                visited: 0,
                link1: Orientation::North,
                link2: Orientation::East,
                direction: 0,
            })),
            "J" => Ok(MazePiece::Pipe(Pipe {
                visited: 0,
                link1: Orientation::North,
                link2: Orientation::West,
                direction: 0,
            })),
            "7" => Ok(MazePiece::Pipe(Pipe {
                visited: 0,
                link1: Orientation::West,
                link2: Orientation::South,
                direction: 0,
            })),
            "F" => Ok(MazePiece::Pipe(Pipe {
                visited: 0,
                link1: Orientation::East,
                link2: Orientation::South,
                direction: 0,
            })),
            "." => Ok(MazePiece::Ground(0)),
            "S" => Ok(MazePiece::Start),
            _ => bail!("Invalid maze piece: {s}"),
        }
    }
}

struct Maze {
    sources: Vec<Vec<char>>,
    pieces: Vec<Vec<MazePiece>>,
    start_pos: [usize; 2],
}

impl Maze {
    fn orientation_to_coordinates(
        &self,
        x: usize,
        y: usize,
        orientation: Orientation,
    ) -> Option<[usize; 2]> {
        match orientation {
            Orientation::North => {
                if y > 0 {
                    Some([x, y - 1])
                } else {
                    None
                }
            }
            Orientation::South => {
                if y < self.pieces.len() {
                    Some([x, y + 1])
                } else {
                    None
                }
            }
            Orientation::East => {
                if x < self.pieces[0].len() {
                    Some([x + 1, y])
                } else {
                    None
                }
            }
            Orientation::West => {
                if x > 0 {
                    Some([x - 1, y])
                } else {
                    None
                }
            }
        }
    }

    fn piece_at(&self, x: usize, y: usize) -> &MazePiece {
        &self.pieces[y][x]
    }

    fn letter_at(sources: &Vec<Vec<char>>, x: usize, y: usize) -> char {
        sources[y][x]
    }

    fn piece_at_mut(&mut self, x: usize, y: usize) -> &mut MazePiece {
        &mut self.pieces[y][x]
    }

    fn find_next_pos(&self, pos: &[usize; 2], visited_count: u8) -> Option<[usize; 2]> {
        let current_piece = self.piece_at(pos[0], pos[1]);

        match current_piece {
            MazePiece::Start => {
                // Look everywhere around
                if let Some(m) = self.orientation_to_coordinates(pos[0], pos[1], Orientation::North)
                {
                    if let MazePiece::Pipe(other) = self.piece_at(m[0], m[1]) {
                        if other.link1 == Orientation::South || other.link2 == Orientation::South {
                            return Some(m);
                        }
                    }
                }
                if let Some(m) = self.orientation_to_coordinates(pos[0], pos[1], Orientation::South)
                {
                    if let MazePiece::Pipe(other) = self.piece_at(m[0], m[1]) {
                        if other.link1 == Orientation::North || other.link2 == Orientation::North {
                            return Some(m);
                        }
                    }
                }
                if let Some(m) = self.orientation_to_coordinates(pos[0], pos[1], Orientation::East)
                {
                    if let MazePiece::Pipe(other) = self.piece_at(m[0], m[1]) {
                        if other.link1 == Orientation::West || other.link2 == Orientation::West {
                            return Some(m);
                        }
                    }
                }
                if let Some(m) = self.orientation_to_coordinates(pos[0], pos[1], Orientation::West)
                {
                    if let MazePiece::Pipe(other) = self.piece_at(m[0], m[1]) {
                        if other.link1 == Orientation::East || other.link2 == Orientation::East {
                            return Some(m);
                        }
                    }
                }
                None
            }
            MazePiece::Pipe(p) => {
                //return if orientation is compatible
                if let Some(m) = self.orientation_to_coordinates(pos[0], pos[1], p.link1) {
                    if let MazePiece::Pipe(other) = self.piece_at(m[0], m[1]) {
                        if other.can_connect_to(p) && other.visited < visited_count {
                            return Some(m);
                        }
                    }
                }
                if let Some(m) = self.orientation_to_coordinates(pos[0], pos[1], p.link2) {
                    if let MazePiece::Pipe(other) = self.piece_at(m[0], m[1]) {
                        if other.can_connect_to(p) && other.visited < visited_count {
                            return Some(m);
                        }
                    }
                }

                None
            }
            MazePiece::Ground(_) => None,
        }
    }

    pub fn walk(&mut self) -> usize {
        let mut current_pos = self.start_pos;
        let mut steps = 0;
        let sources = self.sources.clone();

        loop {
            let last_pos = current_pos;
            steps += 1;
            current_pos = match self.find_next_pos(&current_pos, 1) {
                Some(c) => c,
                None => break,
            };

            if let MazePiece::Pipe(p) = self.piece_at_mut(current_pos[0], current_pos[1]) {
                p.visited = 1;
                if p.is_vertical() {
                    // On same line, this is a turn
                    if last_pos[1] == current_pos[1] {
                        p.direction =
                            match Self::letter_at(&sources, current_pos[0], current_pos[1]) {
                                'J' | 'L' => 1,
                                '7' | 'F' => -1,
                                _ => 0,
                            };
                    } else {
                        p.direction = if last_pos[1] < current_pos[1] { -1 } else { 1 };
                    }
                }
            }
        }

        steps
    }

    pub fn search(&mut self) -> usize {
        let mut current_pos = self.start_pos;
        let mut steps = 0;
        loop {
            current_pos = match self.find_next_pos(&current_pos, 2) {
                Some(c) => c,
                None => break,
            };
            if let MazePiece::Pipe(p) = self.piece_at_mut(current_pos[0], current_pos[1]) {
                p.visited += 1;
                if p.direction == 1 {
                    for i in current_pos[0] + 1..self.pieces[0].len() {
                        let piece = self.piece_at_mut(i, current_pos[1]);
                        if piece.visited() == 0 && !matches!(piece, MazePiece::Start) {
                            piece.inc_visited();
                            steps += 1;
                        } else {
                            break;
                        }
                    }
                } else if p.direction == -1 {
                    for i in (0..current_pos[0]).rev() {
                        let piece = self.piece_at_mut(i, current_pos[1]);
                        if piece.visited() == 0 && !matches!(piece, MazePiece::Start) {
                            piece.inc_visited();
                            steps += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        steps
    }
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut pieces: Vec<Vec<MazePiece>> = vec![];
        let mut sources = vec![];
        let mut start: Option<[usize; 2]> = None;
        for line in s.lines() {
            if line.is_empty() {
                continue;
            }
            sources.push(line.chars().collect());
            pieces.push(
                line.chars()
                    .map(|p| p.to_string().parse().unwrap())
                    .collect(),
            );

            if let Some(s_x) = pieces
                .last()
                .unwrap()
                .iter()
                .enumerate()
                .find_map(|(x, p)| {
                    if let MazePiece::Start = p {
                        Some(x)
                    } else {
                        None
                    }
                })
            {
                start = Some([s_x, pieces.len() - 1]);
            }
        }

        Ok(Maze {
            sources,
            pieces,
            start_pos: start.unwrap(),
        })
    }
}

pub struct Day10;

impl AdventOfCode for Day10 {
    fn day(&self) -> u8 {
        10
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let mut maze: Maze = input.unwrap().parse()?;
        Ok((maze.walk() / 2).to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut maze: Maze = input.unwrap().parse()?;
        maze.walk();
        Ok(maze.search().to_string())
    }
}
