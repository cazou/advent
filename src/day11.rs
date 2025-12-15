use crate::traits::AdventOfCode;
use anyhow::Result;
use std::{cell::RefCell, collections::HashMap, rc::Rc, str::FromStr};

#[derive(Debug)]
struct Device {
    name: String,
    output_names: Vec<String>,
    outputs: Vec<Rc<RefCell<Device>>>,
}

impl FromStr for Device {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (name, output_names) = s.split_once(':').unwrap();

        let output_names = output_names
            .trim_start()
            .split(' ')
            .map(|n| n.to_string())
            .collect();

        Ok(Device {
            name: name.to_string(),
            output_names,
            outputs: vec![],
        })
    }
}

impl Device {
    fn count_to(&self, memo: &mut HashMap<String, usize>, to: &str) -> usize {
        let mut outs = 0;

        if let Some(v) = memo.get(&self.name) {
            return *v;
        }

        if self.name == to {
            return 1;
        }

        for output in &self.outputs {
            outs += output.borrow().count_to(memo, to);
        }

        memo.insert(self.name.clone(), outs);

        outs
    }
}

struct Reactor {
    you: Option<Rc<RefCell<Device>>>,
    svr: Option<Rc<RefCell<Device>>>,
    dac: Option<Rc<RefCell<Device>>>,
    fft: Option<Rc<RefCell<Device>>>,
}

impl FromStr for Reactor {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut you = None;
        let mut svr = None;
        let mut dac = None;
        let mut fft = None;
        let mut devices = vec![Rc::new(RefCell::new(Device {
            name: "out".to_string(),
            output_names: vec![],
            outputs: vec![],
        }))];

        for l in s.lines() {
            let dev = Rc::new(RefCell::new(l.parse::<Device>().unwrap()));

            if dev.borrow().name == "you" {
                you = Some(dev.clone());
            }

            if dev.borrow().name == "svr" {
                svr = Some(dev.clone());
            }

            if dev.borrow().name == "dac" {
                dac = Some(dev.clone());
            }

            if dev.borrow().name == "fft" {
                fft = Some(dev.clone());
            }

            devices.push(dev);
        }

        for dev in &devices {
            let outputs = Self::find_nodes_with_names(&dev.borrow().output_names, &devices);
            dev.borrow_mut().outputs = outputs;
        }

        Ok(Reactor { you, svr, dac, fft })
    }
}

impl Reactor {
    fn find_nodes_with_names(
        names: &[String],
        devices: &[Rc<RefCell<Device>>],
    ) -> Vec<Rc<RefCell<Device>>> {
        let mut nodes = vec![];
        for dev in devices {
            if names.contains(&dev.borrow().name) {
                nodes.push(dev.clone());
            }
        }

        nodes
    }

    fn count_out(&self) -> usize {
        let mut visited = HashMap::new();
        self.you
            .as_ref()
            .unwrap()
            .borrow()
            .count_to(&mut visited, "out")
    }

    fn count_svr_out(&self) -> usize {
        let mut visited = HashMap::new();

        let a1 = self
            .svr
            .as_ref()
            .unwrap()
            .borrow()
            .count_to(&mut visited, "fft");

        visited = HashMap::new();
        let a2 = self
            .fft
            .as_ref()
            .unwrap()
            .borrow()
            .count_to(&mut visited, "dac");

        visited = HashMap::new();
        let a3 = self
            .dac
            .as_ref()
            .unwrap()
            .borrow()
            .count_to(&mut visited, "out");

        a1 * a2 * a3
    }
}

pub struct Day11;

impl AdventOfCode for Day11 {
    fn day(&self) -> u8 {
        11
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let reactor: Reactor = input.unwrap().parse().unwrap();
        Ok(reactor.count_out().to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let reactor: Reactor = input.unwrap().parse().unwrap();
        Ok(reactor.count_svr_out().to_string())
    }
}
