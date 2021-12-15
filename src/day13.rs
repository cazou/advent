use std::cmp::Ordering;
use std::str::FromStr;
use std::collections::BTreeSet;

enum Fold {
    X(usize),
    Y(usize)
}

impl Fold {
    pub fn from_str(text: &str) -> Fold {
        let (_, info) = text.split_at(10);
        let parts: Vec<&str> = info.trim().split('=').collect();
        match parts[0] {
            "x" => Fold::X(parts[1].parse().unwrap()),
            _ => Fold::Y(parts[1].parse().unwrap())
        }
    }

    pub fn apply(&self, points: &BTreeSet<Point>) -> BTreeSet<Point> {
        let mut new_vec: BTreeSet<Point> = BTreeSet::new(); //TODO
        let p_index;
        let val;

        match self {
            Fold::X(v) => {
                p_index = 0;
                val = v;
            },
            Fold::Y(v) => {
                p_index = 1;
                val = v;
            }
        }
        for p in points {
            if p.p[p_index] > *val {
                let mut new_p: Point = p.clone();
                new_p.p[p_index] = *val - (p.p[p_index] - *val);
                new_vec.insert(Point {p: new_p.p});
            }
            else {
                new_vec.insert(p.clone());
            }
        }

        new_vec
    }
}

#[derive(Copy, Clone)]
struct Point {
    pub p: [usize; 2],
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<&str> = s.split(',').collect();

        Ok(Point{p: [vals[0].parse().unwrap(), vals[1].parse().unwrap()]})
    }
}

impl Eq for Point {}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.p[0] == other.p[0] && self.p[1] == other.p[1]
    }
}

impl PartialOrd<Self> for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.p[0] == other.p[0] {
            if self.p[1] < other.p[1] {
                Ordering::Less
            } else if self.p[1] > other.p[1] {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        } else if self.p[0] < other.p[0] {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

fn print_points(points: &BTreeSet<Point>) {
    let mut max_x:usize = 0;
    let mut max_y:usize = 0;
    for p in points {
        if max_x < p.p[0] {
            max_x = p.p[0];
        }
        if max_y < p.p[1] {
            max_y = p.p[1];
        }
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&Point{p:[x,y]}) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Origami]...     ");

    let mut points = BTreeSet::new();
    let mut first_fold: Option<usize> = None;
    //let mut folds = vec![];

    // Load input data
    for val in contents.lines() {
        if val.is_empty() {
            continue;
        }

        if val.starts_with("fold") {
            points = Fold::from_str(val).apply(&points);
            first_fold = match first_fold {
                Some(t) => Some(t),
                None => Some(points.len())
            };
        } else {
            points.insert(Point::from_str(val).unwrap());
        }

    }

    println!("{}", first_fold.unwrap());
    print_points(&points);

    Ok(())
}