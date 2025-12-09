use crate::traits::AdventOfCode;
use anyhow::Result;
use std::{cell::RefCell, fmt::Display, rc::Rc, str::FromStr};

pub struct Day8;

#[derive(Clone, Debug)]
struct JunctionBox {
    x: isize,
    y: isize,
    z: isize,
    connections: Vec<Rc<RefCell<JunctionBox>>>,
    id: u8,
}

impl FromStr for JunctionBox {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut vals = s.split(',').map(|v| v.parse::<isize>().unwrap());

        Ok(JunctionBox {
            x: vals.next().unwrap(),
            y: vals.next().unwrap(),
            z: vals.next().unwrap(),
            connections: vec![],
            id: 0,
        })
    }
}

impl JunctionBox {
    fn distance_to(&self, other: &JunctionBox) -> isize {
        (other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)
    }

    fn connected_to(&self, other: &JunctionBox) -> bool {
        other
            .connections
            .iter()
            .any(|j| j.borrow().x == self.x && j.borrow().y == self.y && j.borrow().z == self.z)
    }

    fn connect(first: Rc<RefCell<JunctionBox>>, second: Rc<RefCell<JunctionBox>>) {
        if !first.borrow().connected_to(&second.borrow()) {
            first.borrow_mut().connections.push(second.clone());
            second.borrow_mut().connections.push(first.clone());
        } else {
            println!("Already connected");
        }
    }
}

impl Display for JunctionBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}), connected to: [", self.x, self.y, self.z)?;

        for c in &self.connections {
            let c = &c.borrow();
            write!(f, "({}, {}, {}) ", c.x, c.y, c.z)?;
        }

        write!(f, "]")
    }
}

#[derive(Clone, Debug)]
struct Circuit {
    boxes: Vec<Rc<RefCell<JunctionBox>>>,
    id: usize,
}

struct Hall {
    boxes: Vec<Rc<RefCell<JunctionBox>>>,
    circuits: Vec<Rc<RefCell<Circuit>>>,
}

impl FromStr for Hall {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut circuits = vec![];
        let mut boxes = vec![];

        for (id, line) in s.lines().enumerate() {
            let j_box = Rc::new(RefCell::new(line.parse::<JunctionBox>().unwrap()));
            j_box.borrow_mut().id = id as u8;

            circuits.push(Rc::new(RefCell::new(Circuit {
                boxes: vec![j_box.clone()],
                id,
            })));

            boxes.push(j_box);
        }

        Ok(Hall { circuits, boxes })
    }
}

impl Hall {
    fn find_circuit_of(&self, jbox: &JunctionBox) -> Rc<RefCell<Circuit>> {
        self.circuits
            .iter()
            .find(|c| {
                c.borrow().boxes.iter().any(|b| {
                    b.borrow().x == jbox.x && b.borrow().y == jbox.y && b.borrow().z == jbox.z
                })
            })
            .unwrap()
            .clone()
    }

    fn connect(&mut self) -> usize {
        for _ in 0..1000 {
            let mut new_boxes = vec![];
            let mut min_dist = None;
            let mut b1 = None;
            let mut b2 = None;

            while !self.boxes.is_empty() {
                let first = self.boxes.remove(0);
                for c in &self.boxes {
                    if c.borrow().connected_to(&first.borrow()) {
                        continue;
                    }
                    let dist = first.borrow().distance_to(&c.borrow());

                    if let Some(min) = min_dist {
                        if dist < min {
                            min_dist = Some(dist);
                            b1 = Some(first.clone());
                            b2 = Some(c.clone());
                        }
                    } else {
                        min_dist = Some(dist);
                        b1 = Some(first.clone());
                        b2 = Some(c.clone());
                    }
                }

                new_boxes.push(first);
            }

            self.boxes = new_boxes;

            let b1 = b1.unwrap();
            let b2 = b2.unwrap();

            JunctionBox::connect(b1.clone(), b2.clone());
            let c1 = self.find_circuit_of(&b1.borrow());
            let c2 = self.find_circuit_of(&b2.borrow());

            if c1.borrow().id != c2.borrow().id {
                c1.borrow_mut().boxes.append(&mut c2.borrow_mut().boxes);
                c2.borrow_mut().boxes = vec![];
            }
        }

        self.circuits
            .sort_by(|a, b| b.borrow().boxes.len().cmp(&a.borrow().boxes.len()));
        self.circuits[0].borrow().boxes.len()
            * self.circuits[1].borrow().boxes.len()
            * self.circuits[2].borrow().boxes.len()
    }

    fn connect_all(&mut self) -> usize {
        let mut circuits_count = self.circuits.len();
        loop {
            let mut new_boxes = vec![];
            let mut min_dist = None;
            let mut b1 = None;
            let mut b2 = None;

            while !self.boxes.is_empty() {
                let first = self.boxes.remove(0);
                for c in &self.boxes {
                    if c.borrow().connected_to(&first.borrow()) {
                        continue;
                    }
                    let dist = first.borrow().distance_to(&c.borrow());

                    if let Some(min) = min_dist {
                        if dist < min {
                            min_dist = Some(dist);
                            b1 = Some(first.clone());
                            b2 = Some(c.clone());
                        }
                    } else {
                        min_dist = Some(dist);
                        b1 = Some(first.clone());
                        b2 = Some(c.clone());
                    }
                }

                new_boxes.push(first);
            }

            self.boxes = new_boxes;

            let b1 = b1.unwrap();
            let b2 = b2.unwrap();

            JunctionBox::connect(b1.clone(), b2.clone());
            let c1 = self.find_circuit_of(&b1.borrow());
            let c2 = self.find_circuit_of(&b2.borrow());

            if c1.borrow().id != c2.borrow().id {
                c1.borrow_mut().boxes.append(&mut c2.borrow_mut().boxes);
                c2.borrow_mut().boxes = vec![];
                circuits_count -= 1;
            }

            if circuits_count == 1 {
                return (b1.borrow().x * b2.borrow().x) as usize;
            }
        }
    }
}

impl AdventOfCode for Day8 {
    fn day(&self) -> u8 {
        8
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let mut hall: Hall = input.unwrap().parse().unwrap();
        Ok(hall.connect().to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut hall: Hall = input.unwrap().parse().unwrap();
        Ok(hall.connect_all().to_string())
    }
}
