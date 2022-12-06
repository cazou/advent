use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, BTreeSet};
use std::fmt::{Display, Formatter};

//TODO: Add cave depth variable (autodetect when loading input)

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq, PartialEq)]
enum AmphipodType {
    Amber,
    Bronze,
    Copper,
    Desert
}

#[derive(Clone)]
#[derive(Debug)]
struct Amphipod {
    a_type: AmphipodType,
    position: [isize; 2],
}

impl Amphipod {
    fn new(t: char, x: isize, y: isize) -> Amphipod {
        let a_type = match t {
            'A' => AmphipodType::Amber,
            'B' => AmphipodType::Bronze,
            'C' => AmphipodType::Copper,
            'D' => AmphipodType::Desert,
            _ => AmphipodType::Amber //Should be an error
        };

        Amphipod {
            a_type,
            position: [x, y]
        }
    }

    fn cave_pos(&self) -> isize {
        match self.a_type {
            AmphipodType::Amber => 3,
            AmphipodType::Bronze => 5,
            AmphipodType::Copper => 7,
            AmphipodType::Desert => 9
        }
    }

    fn energy_cost(&self) -> usize {
        match self.a_type {
            AmphipodType::Amber => 1,
            AmphipodType::Bronze => 10,
            AmphipodType::Copper => 100,
            AmphipodType::Desert => 1000
        }
    }

    fn letter(&self) -> String {
        match self.a_type {
            AmphipodType::Amber => "A",
            AmphipodType::Bronze => "B",
            AmphipodType::Copper => "C",
            AmphipodType::Desert => "D"
        }.to_string()
    }
}

impl PartialEq for Amphipod {
    fn eq(&self, other: &Self) -> bool {
        self.a_type == other.a_type && self.position == other.position
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} @ {:?}", self.a_type, self.position)
    }
}

struct State {
    amphipods: Vec<Amphipod>,
    moves: Vec<[isize; 4]>,
    score: usize,
    max_y: isize,
}

impl State {
    fn solved(&self) -> bool {
        !self.amphipods.iter().any(|a| a.cave_pos() != a.position[0] || a.position[1] == 1)
    }
}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.amphipods.len() == other.amphipods.len() && self.amphipods == other.amphipods
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.score.cmp(&self.score))
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

/*impl State {
    fn same(&self, other: &State) -> bool {
        self.score == other.score && self.amphipods.len() == other.amphipods.len() && self.amphipods == other.amphipods
    }
}*/

struct Burrow {
    // All amphipods in the burrow
    amphipods: Vec<Amphipod>,
    // All available (taken or not) positions in the burrow
    positions: Vec<[isize; 2]>,
    max_y: isize,
}

impl Burrow {
    fn from_input(input: &str) -> Result<Burrow, String> {
        let mut y = 0;
        let mut ret = Burrow {
            amphipods: vec![],
            positions: vec![],
            max_y: 0
        };

        for line in input.lines() {
            let mut x = 0;
            for c in line.chars() {
                match c {
                    '.' => ret.positions.push([x, y]),
                    'A' | 'B' | 'C' | 'D' => {
                        ret.amphipods.push(Amphipod::new(c, x, y));
                        ret.positions.push([x, y]);
                        ret.max_y = y;
                    },
                    _ => {},
                };
                x += 1;
            }
            y += 1;
        }

        Ok(ret)
    }

    fn get_reachable_pos(&self, amphipod: &Amphipod, amphipods: &Vec<Amphipod>) -> Vec<[isize; 2]> {
        let mut ret = vec![];
        let mut pos_to_check = vec![];
        let x = amphipod.position[0];
        let y = amphipod.position[1];

        if y == 1 {
            // Check if there is a blocker on the way
            let r1 = min(amphipod.cave_pos(), x);
            let r2 = max(amphipod.cave_pos(), x);
            if amphipods.iter().any(|a| a.position != [x, y] && (r1..=r2).contains(&a.position[0]) && a.position[1] == 1) {
                return vec![];
            }

            // Check if there is is no room in the destination cave
            if amphipods.iter().any(|a| a.position[0] == amphipod.cave_pos() && a.position[1] > 1 && amphipod.cave_pos() != a.cave_pos()) {
                return vec![];
            }
            // Go the further down possible
            for i in 2..=self.max_y {
                if !amphipods.iter().any(|a| a.position == [amphipod.cave_pos(), self.max_y - i + 2]) {
                    ret.push([amphipod.cave_pos(), self.max_y - i + 2]);
                    break;
                }
            }
            return ret;
        } else if amphipods.iter().any(|a| a.position[0] == x && a.position[1] < y) {
            // We are blocked in the cave, can't move
            return vec![];
        } else if amphipod.cave_pos() == x && amphipods.iter().any(|a| a.position[0] == x && a.position[1] < y && a.cave_pos() != x) {
            // We are blocking amphipods that are in the wrong cave, gotta move
            pos_to_check.push([x, 1]);
        } else if amphipod.cave_pos() == x {
            // We are at the right place and not blocking anyone
            return vec![];
        } else {
            pos_to_check.push([x, y]);
        }

        while !pos_to_check.is_empty() {
            let pos = pos_to_check.pop().unwrap();

            for p_x in pos[0]-1..=pos[0]+1 {
                let p_y = 1;
                if p_x == x && p_y == y {
                    continue;
                }

                if self.positions.contains(&[p_x, p_y]) && !ret.contains(&[p_x, p_y]) {
                    // Other amphipod can block the way
                    if amphipods.iter().any(|c| c.position == [p_x, p_y]) {
                        continue;
                    }
                    // Do not enter another cave
                    if (p_y > 1) && (p_x != amphipod.cave_pos()) {
                        continue;
                    }
                    ret.push([p_x, p_y]);
                    pos_to_check.push([p_x, p_y]);
                }
            }
        }

        // Don't let an amphipod in front of a cave
        let ret2 = ret.into_iter().filter(|p| !([3,5,7,9].contains(&p[0]) && p[1] == 1)).collect();
        println!("{} -> {:?}", amphipod, ret2);
        ret2
    }

    fn sort(&mut self) -> usize {
        let mut heap = BinaryHeap::new();
        let mut set = BTreeSet::new();

        heap.push(State {
            amphipods: self.amphipods.clone(),
            moves: vec![],
            score: 0,
            max_y: self.max_y
        });
        print!("{}", heap.peek().unwrap());

        while heap.len() > 0 {
            let state = heap.pop().unwrap();
            set.remove(&state);
            assert_eq!(heap.len(), set.len());

            if state.solved() {
                print!("{}", state);
                return state.score;
            }

            //print!("{}", state);

            for a in &state.amphipods {
                for pos in self.get_reachable_pos(a, &state.amphipods) {
                    //println!("{} -> {:?}", a, pos);
                    let mut new_map = state.amphipods.clone();
                    let score_delta = a.energy_cost() * ((a.position[0]-pos[0]).abs() + (a.position[1]-pos[1]).abs()) as usize;
                    let mut new_amphipod = new_map.iter_mut().find(|c| c.position == [a.position[0],a.position[1]]).unwrap();
                    let mut new_moves = state.moves.clone();
                    new_moves.push([new_amphipod.position[0], new_amphipod.position[1], pos[0], pos[1]]);
                    new_amphipod.position = pos;

                    let new_state1 = State { amphipods: new_map.clone(), score: state.score + score_delta, moves: new_moves, max_y: self.max_y};
                    println!("{}", new_state1);
                    let new_state2 = State { amphipods: new_map, score: state.score + score_delta, moves: vec![], max_y: self.max_y};
                    if !set.contains(&new_state1) {
                        heap.push(new_state1);
                        set.insert(new_state2);
                    }
                }
            }
        }
        0
    }
}


impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.max_y+1 {
            for x in 0..=12 {
                if let Some(a) = self.amphipods.iter().find(|a| a.position == [x, y]) {
                    if let Err(e) = write!(f, "{}", a.letter()) {
                        return Err(e);
                    }
                } else if (y == 1 && (1..12).contains(&x)) ||
                          ((2..=self.max_y).contains(&y) && [3,5,7,9].contains(&x)) {
                    if let Err(e) = write!(f, ".") {
                        return Err(e);
                    }
                } else if let Err(e) = write!(f, "#") {
                    return Err(e);
                }
            }
            writeln!(f, "").unwrap();
        }

        for m in &self.moves {
            if let Err(e) = writeln!(f, "{},{} -> {},{}", m[0],m[1],m[2],m[3]) {
                return Err(e);
            }
        }
        for a in &self.amphipods {
            if let Err(e) = writeln!(f, "{}", a) {
                return Err(e);
            }
        }

        writeln!(f, "Score: {}", self.score)
    }
}

pub fn run() -> Result<(), String> {

    let example = "\
#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########";

    let debug1 = "\
#############
#.....B.A...#
###.#.#C#D###
  #A#B#C#D#
  #########";

    let debug2 = "\
#############
#...B.A.....#
###.#.#C#D###
  #A#B#C#D#
  #########";

    let input1 = "\
#############
#...........#
###C#D#D#A###
  #B#A#B#C#
  #########";

    let input2 = "\
#############
#...........#
###C#D#D#A###
  #D#B#A#C#
  #A#D#C#A#
  #B#A#B#C#
  #########";

    let mut burrow = Burrow::from_input(debug2).unwrap();
    //println!("{:?}\n{:?}\n{:?}", burrow.positions,
    //         burrow.get_reachable_pos(burrow.amphipods.iter().find(|a| a.position == [8,1]).unwrap(), &burrow.amphipods),
    //         burrow.get_reachable_pos(burrow.amphipods.iter().find(|a| a.position == [6,1]).unwrap(), &burrow.amphipods));
    //println!("{}", burrow);
    //println!("{:?}", (5..=2));
    println!("Score: {}", burrow.sort());

    Ok(())
}

/*
#############
#...........#
###B#C#B#D###
###A#D#C#A#
###########

#.........D.#
###B#C#B#.###
###A#D#C#A#

#.......A.D.#
###B#C#B#.###
###A#D#C#.#

#...B...A.D.#
###.#C#B#.###
###A#D#C#.#

#...B...A.D.#
###.#C#B#.###
###A#D#C#.#


*/