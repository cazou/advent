use std::cell::RefCell;
use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::day18::SnailNumberType::{Array, Single};

#[derive(Copy, Clone)]
enum Side {
    Left,
    Right,
    Parent
}

type SnailNumber = Rc<RefCell<SnailNumberNode>>;

struct SnailNumberNode {
    number_type: SnailNumberType,
    left: Option<SnailNumber>,
    right: Option<SnailNumber>,
    parent: Option<SnailNumber>,
    side: Side,
}

#[derive(Copy, Clone)]
enum SnailNumberType {
    Array,
    Single(usize),
}

impl SnailNumberNode {
    fn from_str_rec(s: Vec<char>, parent: Option<SnailNumber>, consumed: &mut usize, side: Side)
        -> Result<SnailNumber, ()> {
        let c = s[0];
        if c == '[' {
            *consumed = 1;
            let mut cons = 0;
            let number = Rc::new(RefCell::new(SnailNumberNode {
                number_type: Array,
                left: None,
                right: None,
                parent,
                side
            }));

            let p1 = SnailNumberNode::from_str_rec(s[*consumed..].to_vec(),
                                                   Some(Rc::clone(&number)),
                                                   &mut cons,
                                                   Side::Left).unwrap();
            *consumed += cons;
            assert_eq!(s[*consumed..].to_vec()[0], ',');
            *consumed += 1;

            let p2 = SnailNumberNode::from_str_rec(s[*consumed..].to_vec(),
                                                   Some(Rc::clone(&number)),
                                                   &mut cons,
                                                   Side::Right).unwrap();
            *consumed += cons;

            assert_eq!(s[*consumed..].to_vec()[0], ']');
            *consumed += 1;

            number.borrow_mut().left = Some(Rc::clone(&p1));
            number.borrow_mut().right = Some(Rc::clone(&p2));

            Ok(Rc::clone(&number))
        } else {
            for v in s.splitn(2, |x| *x == ',' || *x == ']') {
                *consumed = v.len();
                let num_str: String = v.iter().collect();
                return Ok(Rc::new(RefCell::new(SnailNumberNode {
                    number_type: Single(num_str.parse().unwrap()),
                    left: None,
                    right: None,
                    parent,
                    side,
                })));
            }

            return Err(());
        }
    }

    fn clone(&self) -> SnailNumber {
        let t = self.number_type;
        match t {
            Single(a) => {
                let mut ret = SnailNumberNode {
                    number_type: SnailNumberType::Single(a),
                    left: None,
                    right: None,
                    parent: None,
                    side: self.side
                };

                ret.parent = match &self.parent {
                    None => None,
                    Some(r) => Some(Rc::clone(r))
                };

                Rc::new(RefCell::new(ret))
            },
            Array => {
                let new_node = SnailNumberNode {
                    number_type: SnailNumberType::Array,
                    left: None,
                    right: None,
                    parent: None,
                    side: self.side
                };

                let ret = Rc::new(RefCell::new(new_node));

                ret.borrow_mut().parent = match &self.parent {
                    None => None,
                    Some(r) => Some(Rc::clone(r))
                };

                ret.borrow_mut().left = match &self.left {
                    None => None,
                    Some(r) => {
                        let n = Rc::clone(&r.borrow().clone());
                        n.borrow_mut().parent = Some(Rc::clone(&ret));
                        Some(n)
                    },
                };

                ret.borrow_mut().right = match &self.right {
                    None => None,
                    Some(r) => {
                        let n = Rc::clone(&r.borrow().clone());
                        n.borrow_mut().parent = Some(Rc::clone(&ret));
                        Some(n)
                    },
                };

                ret
            }
        }
    }

    fn add(a: SnailNumber, b: SnailNumber) -> SnailNumber {
        let new_parent = Rc::new(RefCell::new(
            SnailNumberNode {
                number_type: SnailNumberType::Array,
                parent: None,
                left: None,
                right: None,
                side: Side::Parent,
            }
        ));

        a.borrow_mut().parent = Some(Rc::clone(&new_parent));
        b.borrow_mut().parent = Some(Rc::clone(&new_parent));

        a.borrow_mut().side = Side::Left;
        b.borrow_mut().side = Side::Right;

        new_parent.borrow_mut().left = Some(Rc::clone(&a.borrow().clone()));
        new_parent.borrow_mut().right = Some(Rc::clone(&b.borrow().clone()));

        SnailNumberNode::reduce(Rc::clone(&new_parent));

        new_parent
    }

    fn find_neighbor(node: &SnailNumber, side: Side) -> Option<SnailNumber> {
        // Go up until we are not the right child
        let mut current_node = Rc::clone(&node);
        loop {
            let parent= match &current_node.borrow().parent {
                None => return None,
                Some(ref n) => Rc::clone(n)
            };

            let child_side = current_node.borrow().side;

            current_node = parent;

            match side {
                Side::Right => match child_side {
                    Side::Left => break,
                    Side::Right => {},
                    Side::Parent => return None,
                },
                Side::Left => match child_side {
                    Side::Left => {},
                    Side::Right => break,
                    Side::Parent => return None,
                },
                Side::Parent => return None,
            };
        }

        // Go down to the correct child
        current_node = match side {
            Side::Right => Rc::clone(current_node.borrow().right.as_ref().unwrap()),
            Side::Left => Rc::clone(current_node.borrow().left.as_ref().unwrap()),
            _ => return None
        };

        // Go to the other side child until it is a Single(_)
        loop {
            current_node = match side {
                Side::Right => match &current_node.borrow().left {
                    None => break,
                    Some(c) => Rc::clone(c),
                },
                Side::Left => match &current_node.borrow().right {
                    None => break,
                    Some(c) => Rc::clone(c),
                },
                _ => return None
            };
        }

        Some(current_node)
    }

    fn explode_rec(this: SnailNumber, level: usize) -> bool {
        let t = this.borrow().number_type;
        if let Array = t {
                let left = match &this.borrow().left {
                    Some(a) => Rc::clone(a),
                    None => return false //This should never happen
                };

                let right = match &this.borrow().right {
                    Some(a) => Rc::clone(a),
                    None => return false //This should never happen
                };

                return if level == 4 {
                    // Explode
                    if let Single(v) = left.borrow().number_type {
                        match SnailNumberNode::find_neighbor(&left, Side::Left) {
                            None => {},
                            Some(l) => {
                                if let Single(ref mut v2) = l.borrow_mut().number_type {
                                    *v2 += v;
                                }
                            }
                        }
                    }

                    if let Single(v) = right.borrow().number_type {
                        match SnailNumberNode::find_neighbor(&right, Side::Right) {
                            None => {},
                            Some(l) => {
                                if let Single(ref mut v2) = l.borrow_mut().number_type {
                                    *v2 += v;
                                }
                            }
                        }
                    }

                    this.borrow_mut().number_type = Single(0);
                    this.borrow_mut().left = None;
                    this.borrow_mut().right = None;

                    true
                } else {
                    if SnailNumberNode::explode_rec(left, level + 1) {
                        return true;
                    }

                    SnailNumberNode::explode_rec(right, level + 1)
                }
            }

        false
    }

    fn split_rec(this: SnailNumber) -> bool {
        let t = this.borrow().number_type;
        match t {
            Single(a) => {
                if a > 9 {
                    // Split the value
                    this.borrow_mut().number_type = Array;
                    let pad = a % 2;
                    this.borrow_mut().left = Some(Rc::new(RefCell::new(SnailNumberNode {
                        number_type: Single(a / 2),
                        left: None,
                        right: None,
                        parent: Some(Rc::clone(&this)),
                        side: Side::Left,
                    })));
                    this.borrow_mut().right = Some(Rc::new(RefCell::new(SnailNumberNode {
                        number_type: Single(a / 2 + pad),
                        left: None,
                        right: None,
                        parent: Some(Rc::clone(&this)),
                        side: Side::Right,
                    })));

                    return true;
                }
            },
            Array => {
                let left = match &this.borrow().left {
                    Some(a) => Rc::clone(a),
                    None => return false //This should never happen
                };

                let right = match &this.borrow().right {
                    Some(a) => Rc::clone(a),
                    None => return false //This should never happen
                };

                if SnailNumberNode::split_rec(left) {
                    return true;
                }

                return SnailNumberNode::split_rec(right)
            }
        };

        false
    }

    // Reduce the number, return false if the number is already reduced, true otherwise
    fn reduce_rec(this: SnailNumber) -> bool {
        if SnailNumberNode::explode_rec(Rc::clone(&this), 0) {
            return SnailNumberNode::reduce_rec(Rc::clone(&this));
        } else {
            if SnailNumberNode::split_rec(Rc::clone(&this)) {
                SnailNumberNode::reduce_rec(Rc::clone(&this));
            }
        }

        false
    }

    fn reduce(number: SnailNumber) {
        SnailNumberNode::reduce_rec(number);
    }

    fn magnitude(number: SnailNumber) -> usize {
        let t = number.borrow().number_type;
        match t {
            Single(a) => a,
            Array => {
                3 * SnailNumberNode::magnitude(
                    Rc::clone(&number.borrow().left.as_ref().unwrap())
                ) +
                    2 * SnailNumberNode::magnitude(
                        Rc::clone(&number.borrow().right.as_ref().unwrap())
                    )
            }
        }
    }

    fn from_str(s: &str) -> Result<SnailNumber, ()> {
        let num = s.clone().chars().collect();
        let mut cons = 0;
        SnailNumberNode::from_str_rec(num, None, &mut cons, Side::Parent)
    }
}

impl Display for SnailNumberNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.number_type {
            Single(c) => write!(f, "{}", *c),
            Array => write!(f, "[{},{}]",
                            self.left.as_ref().unwrap().borrow(),
                            self.right.as_ref().unwrap().borrow())
        }
    }
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Snailfish]...   ");

    let mut sum: Option<SnailNumber> = None;
    let mut numbers: Vec<SnailNumber> = vec![];

    for val in contents.lines() {
        if val.is_empty() {
            continue;
        }

        let num = match SnailNumberNode::from_str(val) {
            Err(_) => {
                return Err("Cannot parse value".to_string())
            },
            Ok(a) => Rc::clone(&a)
        };

        numbers.push(num);
    }

    for num in &numbers {
        sum = match &sum {
            None => Some(Rc::clone(num)),
            Some(c) => {
                let new_sum =
                    SnailNumberNode::add(Rc::clone(c), Rc::clone(num));
                Some(new_sum)
            }
        };
    }

    match sum {
        None => eprintln!("No sum computed"),
        Some(s) => print!("{}", SnailNumberNode::magnitude(Rc::clone(&s))),
    }

    let mut max_mag = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }

            let sum = SnailNumberNode::add(Rc::clone(&numbers[i]),
                                                       Rc::clone(&numbers[j]));
            let mag = SnailNumberNode::magnitude(sum);
            max_mag = max(mag, max_mag);

            let sum = SnailNumberNode::add(Rc::clone(&numbers[j]),
                                                       Rc::clone(&numbers[i]));
            let mag = SnailNumberNode::magnitude(sum);
            max_mag = max(mag, max_mag);
        }
    }

    println!(", {}", max_mag);

    Ok(())
}