use std::cmp::{
    max,
    min
};
use std::fmt::{Display, Formatter};

struct Reactor {
    steps: Vec<Step>,
    size: Option<isize>,
}

struct Step {
    action: bool,
    cuboid: Cuboid,
}

impl Reactor {
    pub fn new(size: Option<isize>) -> Reactor {
        Reactor {
            steps: vec![],
            size
        }
    }

    pub fn parse_step(&mut self, line: &str) {
        let (action, ranges) = line.split_once(' ').unwrap();

        let mut s = Step {
            action: action.starts_with("on"),
            cuboid: Cuboid {
                x: [0, 0],
                y: [0, 0],
                z: [0, 0],
            }
        };

        for v in ranges.split(',') {
            if v.starts_with('x') {
                let r = v.strip_prefix("x=").unwrap();
                let (start, end) = r.split_once("..").unwrap();
                s.cuboid.x = [start.parse().unwrap(), end.parse().unwrap()];
            } else if v.starts_with('y') {
                let r = v.strip_prefix("y=").unwrap();
                let (start, end) = r.split_once("..").unwrap();
                s.cuboid.y = [start.parse().unwrap(), end.parse().unwrap()];
            } else if v.starts_with('z') {
                let r = v.strip_prefix("z=").unwrap();
                let (start, end) = r.split_once("..").unwrap();
                s.cuboid.z = [start.parse().unwrap(), end.parse().unwrap()];
            }
        }

        if let Some(size) = self.size {
            match s.cuboid.limited_cuboid(-size, size) {
                None => return,
                Some(_) => {},
            }
        }

        self.steps.push(s);
    }

    // Return the number of cubes on
    fn run_steps(&mut self) -> isize {
        let mut cuboids: Vec<Cuboid> = vec![];

        for s in &self.steps {
            let mut new_cuboids = vec![];
            for c in &cuboids {
                new_cuboids.append(&mut c.remove(&s.cuboid));
            }

            if s.action {
                new_cuboids.push(s.cuboid);
            }

            cuboids = new_cuboids;
        }

        cuboids.iter().fold(0, |s, e| s + e.area())
    }
}

impl Display for Reactor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for s in &self.steps {
            if s.action {
                write!(f, "on ").unwrap();
            } else  {
                write!(f, "off ").unwrap();
            }
        }

        writeln!(f, "{} steps", self.steps.len())
    }
}

#[derive(Copy, Clone)]
struct Cuboid {
    x: [isize; 2],
    y: [isize; 2],
    z: [isize; 2],
}

impl Cuboid {
    // This could probably be simplified
    fn remove(&self, other: &Cuboid) -> Vec<Cuboid> {
        let mut cuboids = vec![];
        let mut new_x = self.x;
        let mut new_y = self.y;

        let x_range = self.x[0]..=self.x[1];
        let y_range = self.y[0]..=self.y[1];
        let z_range = self.z[0]..=self.z[1];

        if !self.intersects(other) && !other.intersects(self) {
            cuboids.push(self.clone());
            return cuboids;
        }

        if x_range.contains(&other.x[0]) {
            cuboids.push(Cuboid {
                x: [self.x[0], other.x[0] - 1],
                y: self.y,
                z: self.z,
            });
            new_x[0] = other.x[0];
        }

        if x_range.contains(&other.x[1]) {
            cuboids.push(Cuboid {
                x: [other.x[1] + 1, self.x[1]],
                y: self.y,
                z: self.z,
            });

            new_x[1] = other.x[1];
        }

        if y_range.contains(&other.y[0]) {
            cuboids.push(Cuboid {
                x: new_x,
                y: [self.y[0], other.y[0] - 1],
                z: self.z,
            });
            new_y[0] = other.y[0];
        }

        if y_range.contains(&other.y[1]) {
            cuboids.push(Cuboid {
                x: new_x,
                y: [other.y[1] + 1, self.y[1]],
                z: self.z,
            });

            new_y[1] = other.y[1];
        }

        if z_range.contains(&other.z[0]) {
            cuboids.push(Cuboid {
                x: new_x,
                y: new_y,
                z: [self.z[0], other.z[0] - 1],
            });
        }

        if z_range.contains(&other.z[1]) {
            cuboids.push(Cuboid {
                x: new_x,
                y: new_y,
                z: [other.z[1] + 1, self.z[1]],
            });
        }

        cuboids.into_iter().filter(|c| c.area() > 0).collect()
    }

    fn limited_cuboid(&self, start: isize, end: isize) -> Option<Cuboid> {
        if self.x[1] < start || self.y[1] < start || self.z[1] < start ||
            self.x[0] > end || self.y[0] > end || self.z[0] > end {
            return None;
        }

        Some(Cuboid {
            x: [min(end, max(start, self.x[0])),
                max(start, min(end, self.x[1]))],
            y: [min(end, max(start, self.y[0])),
                max(start, min(end, self.y[1]))],
            z: [min(end, max(start, self.z[0])),
                max(start, min(end, self.z[1]))],
        })
    }

    fn area(&self) -> isize {
        (self.x[1] - self.x[0] + 1) * (self.y[1] - self.y[0] + 1) * (self.z[1] - self.z[0] + 1)
    }

    fn intersects(&self, other: &Cuboid) -> bool {
        other.x[0] <= self.x[1] && other.x[1] >= self.x[0] &&
            other.y[0] <= self.y[1] && other.y[1] >= self.y[0] &&
            other.z[0] <= self.z[1] && other.z[1] >= self.z[0]
    }
}

impl Display for Cuboid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x={}..{},y={}..{},z={}..{}",
               self.x[0], self.x[1], self.y[0], self.y[1], self.z[0], self.z[1])
    }
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Reactor]...    ");

    let mut reactor1 = Reactor::new(Some(50));
    let mut reactor2 = Reactor::new(None);

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        reactor1.parse_step(line);
        reactor2.parse_step(line);
    }

    println!("{} {}", reactor1.run_steps(), reactor2.run_steps());

    Ok(())
}