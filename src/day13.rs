use crate::traits::AdventOfCode;
use anyhow::{ensure, Result};
use std::cmp::{min, Ordering};
use std::str::FromStr;

enum ValueType {
    Integer(usize),
    List,
}

struct Packet {
    packet_type: ValueType,
    sub_packets: Vec<Packet>,
}

impl FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let iter = s.chars();
        let mut root = None;
        let mut parents = vec![];
        let mut current = None;
        let mut current_number = String::new();

        for c in iter {
            match c {
                '[' => {
                    if let Some(p) = current {
                        parents.push(p);
                    }

                    current = Some(Packet {
                        packet_type: ValueType::List,
                        sub_packets: vec![],
                    });
                }
                ']' => {
                    if !current_number.is_empty() {
                        current.as_mut().unwrap().sub_packets.push(Packet {
                            packet_type: ValueType::Integer(current_number.parse().unwrap()),
                            sub_packets: vec![],
                        });
                        current_number.clear()
                    }

                    let mut p = parents.pop();
                    if let Some(parent) = p.as_mut() {
                        parent.sub_packets.push(current.unwrap());
                        current = p;
                    } else {
                        root = current;
                        break;
                    }
                }
                '0'..='9' => {
                    current_number.push(c);
                }
                ',' => {
                    if !current_number.is_empty() {
                        //println!("{current_number}");
                        current.as_mut().unwrap().sub_packets.push(Packet {
                            packet_type: ValueType::Integer(current_number.parse().unwrap()),
                            sub_packets: vec![],
                        });
                        current_number.clear()
                    }
                }
                s => println!("/!\\skip '{s}'"),
            }
        }

        ensure!(root.is_some(), "Cannot parse input");

        Ok(root.unwrap())
    }
}

impl Eq for Packet {}

impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

impl Packet {
    pub fn compare(&self, second: &Packet) -> Ordering {
        match (&self.packet_type, &second.packet_type) {
            (ValueType::Integer(a), ValueType::Integer(b)) => a.cmp(b),
            (ValueType::List, ValueType::List) => {
                let len = min(self.sub_packets.len(), second.sub_packets.len());
                for i in 0..len {
                    let r = self.sub_packets[i].compare(&second.sub_packets[i]);
                    match r {
                        Ordering::Less | Ordering::Greater => return r,
                        _ => continue,
                    }
                }

                self.sub_packets.len().cmp(&second.sub_packets.len())
            }
            (ValueType::Integer(a), ValueType::List) => {
                let mut tmp = Packet {
                    sub_packets: vec![],
                    packet_type: ValueType::List,
                };

                tmp.sub_packets.push(Packet {
                    sub_packets: vec![],
                    packet_type: ValueType::Integer(*a),
                }); // TODO: Just copy self !

                tmp.compare(second)
            }
            (ValueType::List, ValueType::Integer(a)) => {
                let mut tmp = Packet {
                    sub_packets: vec![],
                    packet_type: ValueType::List,
                };

                tmp.sub_packets.push(Packet {
                    sub_packets: vec![],
                    packet_type: ValueType::Integer(*a),
                }); // TODO: Just copy second !

                self.compare(&tmp)
            }
        }
    }
}

pub struct Day13;

impl AdventOfCode for Day13 {
    fn day(&self) -> u8 {
        13
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let mut packets: Vec<Packet> = vec![];
        let mut count = 1;
        let mut sum = 0;
        for line in input.unwrap().lines() {
            if line.is_empty() {
                if packets[0] < packets[1] {
                    sum += count;
                }
                packets.clear();
                count += 1;
                continue;
            }
            let p: Packet = line.parse().unwrap();
            packets.push(p);
        }

        if !packets.is_empty() && packets[0] < packets[1] {
            sum += count;
        }

        Ok(sum.to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut packets = vec![];
        for line in input.unwrap().lines() {
            if line.is_empty() {
                continue;
            }
            let p: Packet = line.parse().unwrap();
            packets.push(p);
        }

        packets.push("[[2]]".parse().unwrap());
        packets.push("[[6]]".parse().unwrap());

        packets.sort();

        let mut ret = 1;
        for (idx, p) in packets.iter().enumerate() {
            if *p == "[[2]]".parse::<Packet>().unwrap() || *p == "[[6]]".parse::<Packet>().unwrap()
            {
                ret *= idx + 1;
            }
        }

        Ok(ret.to_string())
    }
}
