use crate::traits::AdventOfCode;
use anyhow::Result;
use std::{cell::RefCell, rc::Rc, str::FromStr};

pub struct Day8;

#[derive(Clone, Debug, Eq, PartialEq)]
struct JunctionBox {
    x: isize,
    y: isize,
    z: isize,
    connections: Vec<Rc<RefCell<JunctionBox>>>,
    circuit: Option<Rc<RefCell<Circuit>>>,
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
            circuit: None,
        })
    }
}

impl JunctionBox {
    fn distance_to(&self, other: &JunctionBox) -> isize {
        (other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)
    }

    fn connect(first: Rc<RefCell<JunctionBox>>, second: Rc<RefCell<JunctionBox>>) {
        first.borrow_mut().connections.push(second.clone());
        second.borrow_mut().connections.push(first.clone());
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

            let circuit = 
            Rc::new(RefCell::new(Circuit {
                boxes: vec![j_box.clone()],
                id,
            }));

            circuits.push(circuit.clone());
            j_box.borrow_mut().circuit = Some(circuit);

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

    fn sorted_distances(&self) -> Vec<(Rc<RefCell<JunctionBox>>, Rc<RefCell<JunctionBox>>, isize)> {
        let mut distances = vec![];
        for (bid, b1) in self.boxes.iter().enumerate() {
            for id in (bid + 1)..self.boxes.len() {
                let b2 = self.boxes.get(id).unwrap();
                distances.push((b1.clone(), b2.clone(), b1.borrow().distance_to(&b2.borrow())));
            }
        }

        distances.sort_by(|(_, _, d1), (_, _, d2)| d1.cmp(d2));

        distances
    }

    fn connect(&mut self) -> usize {
        let distances = self.sorted_distances();

        for i in 0..1000 {
            let (b1, b2, _dist) = &distances[i];
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
        let distances = self.sorted_distances();

        let mut circuits_count = self.boxes.len();
        for (b1, b2, _dist) in &distances {
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

        0
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
