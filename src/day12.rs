use crate::traits::AdventOfCode;
use anyhow::Result;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Default, Debug)]
struct Node {
    pub height: i8,
    pub distance: usize,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
    id: [usize; 2],
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[{},{}]", self.id[0], self.id[1])
    }
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

struct Map {
    start: Rc<RefCell<Node>>,
    end: Rc<RefCell<Node>>,
    map: Vec<Vec<Rc<RefCell<Node>>>>,
    starts: Vec<Rc<RefCell<Node>>>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut map = vec![];
        let mut starts = vec![];
        let mut start = None;
        let mut end = None;

        for (y, line) in s.lines().enumerate() {
            let mut map_line = vec![];

            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        let pos = Rc::new(RefCell::new(Node {
                            height: 0,
                            distance: 0,
                            id: [x, y],
                            ..Default::default()
                        }));
                        start = Some(Rc::clone(&pos));
                        starts.push(Rc::clone(&pos));
                        map_line.push(pos);
                    }
                    'E' => {
                        let pos = Rc::new(RefCell::new(Node {
                            height: 25,
                            distance: usize::MAX,
                            id: [x, y],
                            ..Default::default()
                        }));
                        end = Some(Rc::clone(&pos));
                        map_line.push(pos);
                    }
                    h => {
                        let pos = Rc::new(RefCell::new(Node {
                            height: h as i8 - 'a' as i8,
                            distance: usize::MAX,
                            id: [x, y],
                            ..Default::default()
                        }));
                        if h == 'a' {
                            starts.push(Rc::clone(&pos));
                        }
                        map_line.push(pos);
                    }
                }
            }
            map.push(map_line);
        }

        for (y, line) in map.iter().enumerate() {
            for (x, node) in line.iter().enumerate() {
                Self::find_neighbours(&mut node.borrow_mut(), x, y, &map);
            }
        }

        Ok(Map {
            start: start.unwrap(),
            end: end.unwrap(),
            starts,
            map,
        })
    }
}

impl Map {
    fn find_neighbours(node: &mut Node, x: usize, y: usize, map: &[Vec<Rc<RefCell<Node>>>]) {
        let mut ret = vec![];
        let pos_height = node.height;

        if y > 0 && map[y - 1][x].borrow().height - pos_height <= 1 {
            ret.push(Rc::clone(&map[y - 1][x]));
        }

        if y < map.len() - 1 && map[y + 1][x].borrow().height - pos_height <= 1 {
            ret.push(Rc::clone(&map[y + 1][x]));
        }

        if x > 0 && map[y][x - 1].borrow().height - pos_height <= 1 {
            ret.push(Rc::clone(&map[y][x - 1]));
        }

        if x < map[0].len() - 1 && map[y][x + 1].borrow().height - pos_height <= 1 {
            ret.push(Rc::clone(&map[y][x + 1]));
        }

        node.neighbors = ret;
    }

    pub fn find_path(&self, start: Rc<RefCell<Node>>) -> usize {
        let mut queue = BinaryHeap::new();
        let mut dists = HashMap::new();

        // Reinit the nodes distance
        for l in &self.map {
            for node in l {
                node.borrow_mut().distance = usize::MAX;
            }
        }

        start.borrow_mut().distance = 0;

        queue.push(start.clone());
        dists.insert(start.borrow().id, 0);

        while let Some(n) = queue.pop() {
            if n == self.end {
                break;
            }

            for neigh in &n.borrow().neighbors {
                if dists.contains_key(&neigh.borrow().id) {
                    //TODO: Use an Option<usize> in distance instead.
                    continue;
                }

                let c_dist = dists[&n.borrow().id] + 1;
                neigh.borrow_mut().distance = c_dist;
                dists.insert(neigh.borrow().id, c_dist);
                queue.push(neigh.clone());
            }
        }

        if dists.contains_key(&self.end.borrow().id) {
            dists[&self.end.borrow().id]
        } else {
            usize::MAX
        }
    }

    pub fn run1(&self) -> usize {
        self.find_path(Rc::clone(&self.start))
    }

    pub fn run2(&self) -> usize {
        // FIXME: A better way would be to start from the end and keep all shortest paths to starts
        let mut min = usize::MAX;
        for s in &self.starts {
            let r = self.find_path(Rc::clone(s));
            if r < min {
                min = r;
            }
        }

        min
    }
}

pub struct Day12;

impl AdventOfCode for Day12 {
    fn day(&self) -> u8 {
        12
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let map: Map = input.unwrap().parse()?;
        Ok(map.run1().to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let map: Map = input.unwrap().parse()?;
        Ok(map.run2().to_string())
    }
}
