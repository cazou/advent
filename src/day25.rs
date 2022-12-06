use std::str::FromStr;

#[derive(Clone)]
enum SeaCucumber {
    East,
    South,
    Empty,
}

struct SeaFloor {
    sea_cucumbers: Vec<Vec<SeaCucumber>>,
}

impl FromStr for SeaFloor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // x and y values increase east for x, south for y
        let mut sea_cucumbers = vec![];

        for line in s.lines() {
            let mut line_vec = vec![];

            for c in line.chars() {
                let sc = match c {
                    '>' => SeaCucumber::East,
                    'v' => SeaCucumber::South,
                    '.' => SeaCucumber::Empty,
                    _ => return Err("Invalid Floor".to_string()),
                };

                line_vec.push(sc);
            }

            sea_cucumbers.push(line_vec);
        }

        Ok(SeaFloor { sea_cucumbers })
    }
}

impl SeaFloor {
    fn width(&self) -> usize {
        self.sea_cucumbers[0].len()
    }

    fn height(&self) -> usize {
        self.sea_cucumbers.len()
    }

    fn can_move(&self, x: usize, y: usize) -> bool {
        let (tx, ty) = match &self.sea_cucumbers[y][x] {
            SeaCucumber::Empty => return false,
            SeaCucumber::East => (x + 1, y),
            SeaCucumber::South => (x, y + 1),
        };

        let tx = tx % self.width();
        let ty = ty % self.height();

        if let SeaCucumber::Empty = self.sea_cucumbers[ty][tx] {
            return true;
        }

        false
    }

    fn move_to(&mut self, x: usize, y: usize, tx: usize, ty: usize) -> bool {
        let tx = tx % self.width();
        let ty = ty % self.height();

        match self.sea_cucumbers[ty][tx] {
            SeaCucumber::Empty => {}
            _ => return false,
        };

        let to_move = self.sea_cucumbers[y][x].clone();
        self.sea_cucumbers[ty][tx] = to_move;
        self.sea_cucumbers[y][x] = SeaCucumber::Empty;

        true
    }

    fn move_cucumber_at(&mut self, x: usize, y: usize) -> bool {
        return match &self.sea_cucumbers[y][x] {
            SeaCucumber::Empty => false,
            SeaCucumber::East => self.move_to(x, y, x + 1, y),
            SeaCucumber::South => self.move_to(x, y, x, y + 1),
        };
    }

    // return false when no cucumbers can move
    fn step(&mut self) -> bool {
        let mut to_be_moved_east = vec![];
        let mut to_be_moved_south = vec![];
        let mut y: usize = 0;
        let mut has_moved = false;

        for sea_cucumber_line in &self.sea_cucumbers {
            let mut x: usize = 0;
            for sea_cucumber in sea_cucumber_line {
                if let SeaCucumber::East = sea_cucumber {
                    if self.can_move(x, y) {
                        to_be_moved_east.push((x, y));
                    }
                }
                x += 1;
            }
            y += 1;
        }

        for (x, y) in &to_be_moved_east {
            has_moved = self.move_cucumber_at(*x, *y) || has_moved;
        }

        y = 0;
        for sea_cucumber_line in &self.sea_cucumbers {
            let mut x: usize = 0;
            for sea_cucumber in sea_cucumber_line {
                if let SeaCucumber::South = sea_cucumber {
                    if self.can_move(x, y) {
                        to_be_moved_south.push((x, y));
                    }
                }
                x += 1;
            }
            y += 1;
        }

        for (x, y) in &to_be_moved_south {
            has_moved = self.move_cucumber_at(*x, *y) || has_moved;
        }

        has_moved
    }

    fn run(&mut self) -> usize {
        let mut ret = 1;

        while self.step() {
            ret += 1;
        }

        println!();

        ret
    }
}

pub fn run(contents: &str) -> Result<(), String> {
    let mut floor: SeaFloor = contents.parse()?;
    println!("Count: {}", floor.run());

    Ok(())
}
