use crate::traits::AdventOfCode;
use anyhow::{bail, Result};
use num::Integer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug)]
struct Node {
    name: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> ({}, {})",
            self.name,
            self.left.as_ref().unwrap().borrow().name,
            self.right.as_ref().unwrap().borrow().name
        )
    }
}

struct Network {
    paths: HashMap<String, Vec<String>>,
    turns: Vec<char>,
    start_nodes: Vec<Rc<RefCell<Node>>>,
}

impl FromStr for Network {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut turns: Option<Vec<char>> = None;
        let mut paths = HashMap::new();
        for line in s.lines() {
            if line.is_empty() {
                continue;
            }
            if let None = turns {
                turns = Some(line.chars().collect());
            } else {
                let (node, node_paths) = line.split_once("=").unwrap();
                let (left, right) = node_paths
                    .strip_prefix(" (")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split_once(", ")
                    .unwrap();

                paths.insert(
                    node.trim().to_string(),
                    vec![left.to_string(), right.to_string()],
                );
            }
        }

        Ok(Network {
            turns: turns.unwrap(),
            paths,
            start_nodes: vec![],
        })
    }
}

impl Network {
    fn make_node(
        existing_nodes: &mut HashMap<String, Rc<RefCell<Node>>>,
        paths: &mut HashMap<String, Vec<String>>,
        node: &str,
    ) -> Rc<RefCell<Node>> {
        if existing_nodes.contains_key(node) {
            return Rc::clone(existing_nodes.get(node).unwrap());
        }

        let new_node = Rc::new(RefCell::new(Node {
            name: node.to_string(),
            left: None,
            right: None,
        }));

        existing_nodes.insert(node.to_string(), Rc::clone(&new_node));

        let children = paths[node].clone();
        let left = Self::make_node(existing_nodes, paths, &children[0]);
        let right = Self::make_node(existing_nodes, paths, &children[1]);

        new_node.borrow_mut().left = Some(left);
        new_node.borrow_mut().right = Some(right);

        new_node
    }

    fn build_network(&mut self) {
        let mut existing_nodes: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
        let keys = self.paths.clone().into_keys().collect::<Vec<String>>();

        for name in &keys {
            let node = Self::make_node(&mut existing_nodes, &mut self.paths, name);
            if node.borrow().name.ends_with("A") {
                self.start_nodes.push(Rc::clone(&node));
            }
        }
    }

    fn lcm(vals: &[usize]) -> usize {
        vals.iter().fold(1, |a, v| a.lcm(v))
    }

    pub fn navigate(&self) -> Result<usize> {
        let mut next_node = Rc::clone(
            self.start_nodes
                .iter()
                .find(|n| n.borrow().name == "AAA")
                .unwrap(),
        );
        let mut found = false;
        let mut steps = 0;

        while !found {
            for turn in &self.turns {
                steps += 1;
                let new_node = match *turn {
                    'L' => Rc::clone(&next_node.borrow().left.as_ref().unwrap()),
                    'R' => Rc::clone(&next_node.borrow().right.as_ref().unwrap()),
                    _ => bail!("Wrong instruction: {turn}"),
                };

                next_node = new_node;

                if next_node.borrow().name == "ZZZ" {
                    found = true;
                    break;
                }
            }
        }

        Ok(steps)
    }

    pub fn navigate_lcm(&self) -> Result<usize> {
        let mut steps_all = vec![];

        for start_node in &self.start_nodes {
            let mut current_node = Rc::clone(start_node);

            let mut done = false;
            let mut steps = 0;

            while !done {
                for turn in &self.turns {
                    steps += 1;
                    let new_node = match *turn {
                        'L' => Rc::clone(&current_node.borrow().left.as_ref().unwrap()),
                        'R' => Rc::clone(&current_node.borrow().right.as_ref().unwrap()),
                        _ => bail!("Wrong instruction: {turn}"),
                    };

                    current_node = new_node;

                    if current_node.borrow().name.ends_with("Z") {
                        done = true;
                        break;
                    }
                }
            }

            steps_all.push(steps);
        }

        Ok(Self::lcm(&steps_all))
    }
}

pub struct Day8;

impl AdventOfCode for Day8 {
    fn day(&self) -> u8 {
        8
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let mut network: Network = input.unwrap().parse()?;
        network.build_network();
        Ok(network.navigate().unwrap().to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut network: Network = input.unwrap().parse()?;
        network.build_network();
        Ok(network.navigate_lcm().unwrap().to_string())
    }
}
