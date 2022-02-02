use std::cmp::max;
use std::fmt::{Display, Formatter};

type Point = [isize; 3];

const ROTATIONS: [[[isize; 3]; 3]; 24]  = [
    [[ 1,  0,  0], [ 0,  1,  0], [ 0,  0,  1]],
    [[ 0,  0,  1], [ 0,  1,  0], [-1,  0,  0]],
    [[-1,  0,  0], [ 0,  1,  0], [ 0,  0, -1]],
    [[ 0,  0, -1], [ 0,  1,  0], [ 1,  0,  0]],

    [[ 0, -1,  0], [ 1,  0,  0], [ 0,  0,  1]],
    [[ 0,  0,  1], [ 1,  0,  0], [ 0,  1,  0]],
    [[ 0,  1,  0], [ 1,  0,  0], [ 0,  0, -1]],
    [[ 0,  0, -1], [ 1,  0,  0], [ 0, -1,  0]],

    [[ 0,  1,  0], [-1,  0,  0], [ 0,  0,  1]],
    [[ 0,  0,  1], [-1,  0,  0], [ 0, -1,  0]],
    [[ 0, -1,  0], [-1,  0,  0], [ 0,  0, -1]],
    [[ 0,  0, -1], [-1,  0,  0], [ 0,  1,  0]],

    [[ 1,  0,  0], [ 0,  0, -1], [ 0,  1,  0]],
    [[ 0,  1,  0], [ 0,  0, -1], [-1,  0,  0]],
    [[-1,  0,  0], [ 0,  0, -1], [ 0, -1,  0]],
    [[ 0, -1,  0], [ 0,  0, -1], [ 1,  0,  0]],

    [[ 1,  0,  0], [ 0, -1,  0], [ 0,  0, -1]],
    [[ 0,  0, -1], [ 0, -1,  0], [-1,  0,  0]],
    [[-1,  0,  0], [ 0, -1,  0], [ 0,  0,  1]],
    [[ 0,  0,  1], [ 0, -1,  0], [ 1,  0,  0]],


    [[ 1,  0,  0], [ 0,  0,  1], [ 0, -1,  0]],
    [[ 0, -1,  0], [ 0,  0,  1], [-1,  0,  0]],
    [[-1,  0,  0], [ 0,  0,  1], [ 0,  1,  0]],
    [[ 0,  1,  0], [ 0,  0,  1], [ 1,  0,  0]],
];

fn vector_rotate(p: &Point, nrot: usize) -> Point {
    let rot = ROTATIONS[nrot];

    [p[0] * rot[0][0] + p[1] * rot[0][1] + p[2] * rot[0][2],
    p[0] * rot[1][0] + p[1] * rot[1][1] + p[2] * rot[1][2],
    p[0] * rot[2][0] + p[1] * rot[2][1] + p[2] * rot[2][2]]
}

pub struct Scanner {
    id: usize,
    beacons: Vec<Vec<Point>>, // one list for each rotations
    pub position: Point,
    pub orientation: usize,
    pub checked: bool,
}

impl Clone for Scanner {
    fn clone(&self) -> Self {
        Scanner {
            id: self.id,
            beacons: self.beacons.clone(),
            position: self.position,
            orientation: self.orientation,
            checked: false
        }
    }
}

impl Scanner {
    fn from_input(data: &str) -> (Vec<Scanner>, Vec<Scanner>) {
        let mut scanners_p = vec![];
        let mut scanners = vec![];
        for val in data.lines() {
            if val.is_empty() {
                continue;
            }

            if val.starts_with("---") {
                let vals: Vec<&str> = val.split(" ").collect();
                let id: usize = vals[2].parse().unwrap();
                let current_scanner = Scanner {
                    id,
                    beacons: vec![vec![]],
                    position: [0, 0, 0],
                    orientation: 0,
                    checked: false
                };
                if id == 0 {
                    scanners_p.push(current_scanner);
                } else {
                    scanners.push(current_scanner);
                }
                continue;
            }

            let vals: Vec<&str> = val.split(',').collect();
            let a: isize = vals[0].parse().unwrap();
            let b: isize = vals[1].parse().unwrap();
            let c: isize = vals[2].parse().unwrap();
            if scanners.is_empty() {
                scanners_p.last_mut().unwrap().beacons[0].push([a, b, c]);
            } else {
                scanners.last_mut().unwrap().beacons[0].push([a, b, c]);
            }
        }

        for scanner in scanners.iter_mut() {
            for rot_id in 1..24 { // First rotation is no rotation
                let mut new_rot = vec![];
                for p in &scanner.beacons[0] {
                    new_rot.push(vector_rotate(p, rot_id));
                }
                scanner.beacons.push(new_rot);
            }
        }

        (scanners_p, scanners)
    }

    fn beacons(&self) -> &[Point] {
        self.beacons[self.orientation].as_slice()
    }

    fn rotate(&self, rot_id: usize) -> Scanner {
        Scanner {
            id: self.id,
            beacons: self.beacons.clone(),
            position: self.position,
            orientation: rot_id,
            checked: self.checked,
        }
    }

    fn find_rel_scanner_position(&self, scanner: &Scanner) -> Option<Scanner> {
        let mut i = 0;

        while i < 24 {
            let mut test_scan = scanner.rotate(i);

            i+=1;

            let mut all_pos = vec![];
            for b1 in self.beacons() {
                for b2 in test_scan.beacons() {
                    all_pos.push([b1[0] - b2[0], b1[1] - b2[1], b1[2] - b2[2]]);
                }
            }

            for p1 in &all_pos {
                let mut occ = 0;
                for p2 in &all_pos {
                    if p1[0] == p2[0] && p1[1] == p2[1] && p1[2] == p2[2] {
                        occ += 1;
                    }
                }
                if occ == 12 {
                    test_scan.position = [
                        p1[0] + self.position[0],
                        p1[1] + self.position[1],
                        p1[2] + self.position[2]
                    ];
                    return Some(test_scan);
                }
            }
        }

        None
    }

    fn find_beacons(scanners: &Vec<Scanner>) -> Vec<Point> {
        let mut ret: Vec<Point> = vec![];
        for s in scanners {
            for b in s.beacons() {
                let beacon = [b[0] + s.position[0], b[1] + s.position[1], b[2] + s.position[2]];
                if ! ret.iter().any(|&p| p[0] == beacon[0] && p[1] == beacon[1] && p[2] == beacon[2]) {
                    ret.push(beacon);
                }
            }
        }

        ret
    }

    fn manhattan(&self, other: &Scanner) -> isize {
        (self.position[0] - other.position[0]).abs() +
            (self.position[1] - other.position[1]).abs() +
            (self.position[2] - other.position[2]).abs()
    }
}

impl Display for Scanner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match writeln!(f, "--- scanner {} ---", self.id) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }

        for point in self.beacons() {
            match writeln!(f, "{},{},{}", point[0], point[1], point[2]) {
                Ok(_) => {},
                Err(e) => return Err(e)
            }
        }

        Ok(())
    }
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Beacon]...    ");

    let (mut positionned, mut not_positionned) = Scanner::from_input(contents);
    let mut found = true;

    while found {
        let mut positionned_new = vec![];
        let mut not_positionned_new = vec![];

        for scan_p in positionned.iter_mut() {
            if scan_p.checked {
                continue;
            }
            scan_p.checked = true;

            for scan_u in &not_positionned {
                match scan_p.find_rel_scanner_position(scan_u) {
                    Some(s) => positionned_new.push(s.clone()),
                    None => not_positionned_new.push(scan_u.clone()),
                }
            }

            not_positionned.clear();
            not_positionned.append(&mut not_positionned_new);
        }

        found = !positionned_new.is_empty();

        positionned.append(&mut positionned_new);
    }

    let mut max_dist = 0;
    for s in &positionned {
        for s2 in &positionned {
            max_dist = max(max_dist, s.manhattan(s2));
        }
    }

    let beacons = Scanner::find_beacons(&positionned);

    println!("{} {}", beacons.len(), max_dist);
    Ok(())
}
