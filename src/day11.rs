use crate::traits::AdventOfCode;
use anyhow::{bail, Result};
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Clone)]
enum Operation {
    Add(Option<usize>),
    Mul(Option<usize>),
}

impl Operation {
    pub fn eval(&self, val: usize) -> usize {
        match self {
            Operation::Add(None) => val + val,
            Operation::Mul(None) => val * val,
            Operation::Add(Some(v)) => val + v,
            Operation::Mul(Some(v)) => val * v,
        }
    }
}

struct Tester {
    div: usize,
    when_true: usize,
    when_false: usize,
}

impl Tester {
    pub fn eval(&self, worry: usize) -> usize {
        if worry % self.div == 0 {
            self.when_true
        } else {
            self.when_false
        }
    }
}

struct Monkey {
    id: u8,
    item_ids: Vec<usize>,
    operation: Operation,
    test: Tester,
    inspect_count: usize,
}

impl Monkey {
    pub fn play(
        &mut self,
        relief: usize,
        items: &mut HashMap<usize, Vec<usize>>,
    ) -> Vec<([usize; 2], Operation)> {
        let mut throws = vec![];

        while !self.item_ids.is_empty() {
            let id = self.item_ids.pop().unwrap();
            let wl = items[&(self.id as usize)][id];
            let wl2 = self.operation.eval(wl) / relief;
            let m = self.test.eval(wl2);

            self.inspect_count += 1;

            throws.push(([m, id], self.operation.clone()));
        }

        throws
    }

    pub fn catch(&mut self, item_id: usize) {
        self.item_ids.push(item_id);
    }

    fn build(s: &str, items: &mut Vec<usize>) -> Result<Self> {
        let mut id: u8 = 0;
        let mut item_ids: Vec<usize> = vec![];
        let mut operation: Operation = Operation::Add(None);
        let mut test = Tester {
            div: 0,
            when_true: 0,
            when_false: 0,
        };

        if s.is_empty() {
            bail!("Unexpected empty line at line {}", 0);
        }

        let regexs = vec![
            Regex::new(r"Monkey (?P<id>\d+):")?,
            Regex::new(r"  Starting items: (?P<items>.*)")?,
            Regex::new(r"  Operation: new = old (?P<op>[+*]) (?P<value>old|\d+)")?,
            Regex::new(r"  Test: divisible by (?P<div>\d+)")?,
            Regex::new(r"    If true: throw to monkey (?P<when_true>\d+)")?,
            Regex::new(r"    If false: throw to monkey (?P<when_false>\d+)")?,
        ];

        for (n, line) in s.lines().enumerate() {
            if line.is_empty() && n == 6 {
                break;
            } else if line.is_empty() {
                bail!("Unexpected empty line at line {}", n);
            }

            let re = &regexs[n];
            let matches = re.captures(line).unwrap();

            if n == 0 {
                id = matches.name("id").unwrap().as_str().parse()?;
            } else if n == 1 {
                for item in matches
                    .name("items")
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|v| v.parse::<usize>().unwrap())
                {
                    item_ids.push(items.len());
                    items.push(item);
                }
            } else if n == 2 {
                let value = match matches.name("value").unwrap().as_str() {
                    "old" => None,
                    s => Some(s.parse::<usize>()?),
                };

                operation = match matches.name("op").unwrap().as_str() {
                    "+" => Operation::Add(value),
                    "*" => Operation::Mul(value),
                    s => bail!("{} is not a valid operator", s),
                }
            } else if n == 3 {
                test.div = matches.name("div").unwrap().as_str().parse()?;
            } else if n == 4 {
                test.when_true = matches.name("when_true").unwrap().as_str().parse()?;
            } else if n == 5 {
                test.when_false = matches.name("when_false").unwrap().as_str().parse()?;
            }
        }

        Ok(Monkey {
            id,
            item_ids,
            operation,
            test,
            inspect_count: 0,
        })
    }
}

struct Troop {
    monkeys: Vec<Rc<RefCell<Monkey>>>,
    // These are the items as seen by each monkey (basically, the item value % the tester div value)
    items: HashMap<usize, Vec<usize>>,
}

impl FromStr for Troop {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut monkeys = vec![];
        let mut item_list = vec![];
        let mut items = HashMap::new();

        for m in s.split("\n\n") {
            monkeys.push(Rc::new(RefCell::new(Monkey::build(m, &mut item_list)?)));
        }

        for m in &monkeys {
            items.insert(m.borrow().id as usize, item_list.clone());
        }

        Ok(Troop { monkeys, items })
    }
}

/*
 * Fun fact: A group of monkeys is is called a Troop. It can also be called a Tribe, a Cartload
 * or a Barrel.
 */
impl Troop {
    fn update_item(
        items: &mut HashMap<usize, Vec<usize>>,
        monkeys: &[Rc<RefCell<Monkey>>],
        item_id: usize,
        op: Operation,
        relief: usize,
    ) {
        for (monkey, b) in items {
            if relief > 1 {
                b[item_id] = op.eval(b[item_id]) / relief;
            } else {
                b[item_id] = op.eval(b[item_id]) % monkeys[*monkey].borrow().test.div;
            }
        }
    }

    fn play(&mut self, rounds: usize, relief: usize) -> Result<String> {
        for _round in 0..rounds {
            for m in &self.monkeys {
                let throws = m.borrow_mut().play(relief, &mut self.items);
                for ([dest, item_id], op) in throws {
                    self.monkeys[dest].borrow_mut().catch(item_id);
                    Self::update_item(&mut self.items, &self.monkeys, item_id, op, relief);
                }
            }
        }

        self.monkeys.sort_by_key(|m| m.borrow().inspect_count);
        self.monkeys.reverse();

        let x = Ok((self.monkeys[0].borrow().inspect_count
            * self.monkeys[1].borrow().inspect_count)
            .to_string());
        x
    }
}

pub struct Day11;

impl AdventOfCode for Day11 {
    fn day(&self) -> u8 {
        11
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let mut troop: Troop = input.unwrap().parse()?;
        troop.play(20, 3)
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut troop: Troop = input.unwrap().parse()?;
        troop.play(10000, 1)
    }
}
