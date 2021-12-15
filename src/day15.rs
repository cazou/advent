use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

fn find_neighbors(cases: &Vec<Vec<usize>>, i: usize, j: usize) -> Vec<[usize; 2]> {
    let mut points = vec![];
    if i > 0 {
        points.push([i-1, j]);
    }
    if j > 0 {
        points.push([i, j-1]);
    }
    if i < cases.len() - 1 {
        points.push([i+1, j]);
    }
    if j < cases.len() - 1 {
        points.push([i, j+1]);
    }

    points
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: [usize; 2],
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_path(cases: &Vec<Vec<usize>>) -> Option<usize> {
    let mut dist: Vec<Vec<usize>> = (0..cases.len()).map(|e| (0..cases.len()).map(|_| usize::MAX).collect() ).collect();
    let mut heap = BinaryHeap::new();

    dist[0][0] = 0;
    heap.push(State { cost: 0, position: [0, 0] });

    while let Some(State { cost, position }) = heap.pop() {
        if position == [cases.len() - 1, cases.len() - 1] {
            return Some(cost);
        }

        if cost > dist[position[0]][position[1]] {
            continue;
        }

        for n in find_neighbors(cases, position[0], position[1]) {
            let next = State { cost: cost + cases[n[0]][n[1]], position: n };

            if next.cost < dist[next.position[0]][next.position[1]] {
                heap.push(next);
                dist[next.position[0]][next.position[1]] = next.cost;
            }
        }
    }

    None
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Chiton]...      ");

    let mut cases: Vec<Vec<usize>> = vec![];

    // Load input data
    for val in contents.lines() {
        if val.is_empty() {
            continue;
        }
        cases.push(Vec::from_iter(val.chars().map(|x| x.to_string().parse().unwrap() )));
    }

    if let Some(v) = find_path(&cases) {
        print!("{} ", v);
    }

    let mut cases2: Vec<Vec<usize>> = vec![];
    for v in &cases {
        cases2.push(vec![]);
        let len = v.len();
        for j in 0..5 {
            let mut new_vec: Vec<usize> = v.iter().map(|x| if (*x + j) >= 10 {((*x + j) % 10) + 1} else {*x + j}).collect();
            cases2.last_mut().unwrap().append(&mut new_vec);
        }
    }

    let len = cases2.len();
    for j in 1..5 {
        for i in 0..len {
            let new_vec: Vec<usize> = cases2[i].iter().map(|x| if (*x + j) >= 10 {((*x + j) % 10) + 1} else {*x + j}).collect();
            cases2.push(new_vec);
        }
    }

    if let Some(v) = find_path(&cases2) {
        println!("{}", v);
    }

    Ok(())
}