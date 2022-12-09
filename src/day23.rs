use std::cmp::{max, min, Ordering};
use std::collections::BinaryHeap;
use std::fmt::{Display, Formatter};

//TODO: Add cave depth variable (autodetect when loading input)

const MAX_Y: i8 = 5;

#[derive(Debug, Clone, Eq, PartialEq)]
enum AmphipodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Clone, Debug)]
struct Amphipod {
    a_type: AmphipodType,
    position: [i8; 2],
}

impl Amphipod {
    fn new(t: char, x: i8, y: i8) -> Amphipod {
        let a_type = match t {
            'A' => AmphipodType::Amber,
            'B' => AmphipodType::Bronze,
            'C' => AmphipodType::Copper,
            'D' => AmphipodType::Desert,
            _ => AmphipodType::Amber, //Should be an error
        };

        Amphipod {
            a_type,
            position: [x, y],
        }
    }

    fn cave_pos(&self) -> i8 {
        match self.a_type {
            AmphipodType::Amber => 3,
            AmphipodType::Bronze => 5,
            AmphipodType::Copper => 7,
            AmphipodType::Desert => 9,
        }
    }

    fn energy_cost(&self) -> u32 {
        match self.a_type {
            AmphipodType::Amber => 1,
            AmphipodType::Bronze => 10,
            AmphipodType::Copper => 100,
            AmphipodType::Desert => 1000,
        }
    }

    fn letter(&self) -> String {
        match self.a_type {
            AmphipodType::Amber => "A",
            AmphipodType::Bronze => "B",
            AmphipodType::Copper => "C",
            AmphipodType::Desert => "D",
        }
        .to_string()
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
    score: u32,
}

impl State {
    fn solved(&self) -> bool {
        !self
            .amphipods
            .iter()
            .any(|a| a.cave_pos() != a.position[0] || a.position[1] == 1)
    }
}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
            && self.amphipods.len() == other.amphipods.len()
            && self.amphipods == other.amphipods
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

struct Burrow {
    // All amphipods in the burrow
    amphipods: Vec<Amphipod>,
    // All available (taken or not) positions in the burrow
    positions: Vec<[i8; 2]>,
}

impl Burrow {
    fn from_input(input: &str) -> Result<Burrow, String> {
        let mut ret = Burrow {
            amphipods: vec![],
            positions: vec![],
        };

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => ret.positions.push([x as i8, y as i8]),
                    'A' | 'B' | 'C' | 'D' => {
                        ret.amphipods.push(Amphipod::new(c, x as i8, y as i8));
                        ret.positions.push([x as i8, y as i8]);
                    }
                    _ => {}
                };
            }
        }

        Ok(ret)
    }

    fn extend_corridor(
        &self,
        pos_to_check: &mut Vec<[i8; 2]>,
        amphipod: &Amphipod,
        amphipods: &[Amphipod],
    ) -> Vec<[i8; 2]> {
        let mut ret = vec![];
        let x = amphipod.position[0];
        let y = amphipod.position[1];

        while !pos_to_check.is_empty() {
            let pos = pos_to_check.pop().unwrap();

            for p_x in pos[0] - 1..=pos[0] + 1 {
                let p_y = 1;
                if p_x == x && p_y == y {
                    continue;
                }

                if self.positions.contains(&[p_x, p_y]) && !ret.contains(&[p_x, p_y]) {
                    // Do not enter another cave
                    if (p_y > 1) && (p_x != amphipod.cave_pos()) {
                        continue;
                    }
                    // Other amphipod can block the way
                    if amphipods.iter().any(|c| c.position == [p_x, p_y]) {
                        continue;
                    }
                    ret.push([p_x, p_y]);
                    pos_to_check.push([p_x, p_y]);
                }
            }
        }

        ret
    }

    fn get_reachable_pos(&self, amphipod: &Amphipod, amphipods: &[Amphipod]) -> Vec<[i8; 2]> {
        let mut ret = vec![];
        let mut pos_to_check = vec![];
        let x = amphipod.position[0];
        let y = amphipod.position[1];

        if y == 1 {
            // Check if there is a blocker on the way
            let r1 = min(amphipod.cave_pos(), x);
            let r2 = max(amphipod.cave_pos(), x);
            if amphipods.iter().any(|a| {
                a.position != [x, y] && (r1..=r2).contains(&a.position[0]) && a.position[1] == 1
            }) {
                return vec![];
            }

            // Check if there is is no room in the destination cave
            if amphipods.iter().any(|a| {
                a.position[0] == amphipod.cave_pos()
                    && a.position[1] > 1
                    && amphipod.cave_pos() != a.cave_pos()
            }) {
                return vec![];
            }
            // Go the further down possible
            for i in 2..=MAX_Y {
                if !amphipods
                    .iter()
                    .any(|a| a.position == [amphipod.cave_pos(), MAX_Y - i + 2])
                {
                    ret.push([amphipod.cave_pos(), MAX_Y - i + 2]);
                    break;
                }
            }
            return ret;
        } else if amphipods
            .iter()
            .any(|a| a.position[0] == x && a.position[1] < y)
        {
            // We are blocked in the cave, can't move
            return vec![];
        } else if amphipod.cave_pos() == x
            && amphipods
                .iter()
                .any(|a| a.position[0] == x && a.position[1] > y && a.cave_pos() != x)
        {
            // We are blocking amphipods that are in the wrong cave, gotta move
            pos_to_check.push([x, 1]);
        } else if amphipod.cave_pos() == x {
            // We are at the right place and not blocking anyone
            return vec![];
        } else {
            pos_to_check.push([x, y]);
        }

        ret = self.extend_corridor(&mut pos_to_check, amphipod, amphipods);

        // Don't let an amphipod in front of a cave
        ret.into_iter()
            .filter(|p| !([3, 5, 7, 9].contains(&p[0]) && p[1] == 1))
            .collect()
    }

    fn sort(&mut self) -> u32 {
        let mut heap = BinaryHeap::new();

        heap.push(State {
            amphipods: self.amphipods.clone(),
            score: 0,
        });

        while !heap.is_empty() {
            let state = heap.pop().unwrap();

            if state.solved() {
                println!("SOLVED !");
                print!("{}", state);
                return state.score;
            }

            for a in &state.amphipods {
                for pos in self.get_reachable_pos(a, &state.amphipods) {
                    let mut new_map = state.amphipods.clone();
                    let score_delta = a.energy_cost()
                        * ((a.position[0] - pos[0]).abs() + (a.position[1] - pos[1]).abs()) as u32;
                    let mut new_amphipod = new_map
                        .iter_mut()
                        .find(|c| c.position == [a.position[0], a.position[1]])
                        .unwrap();
                    new_amphipod.position = pos;

                    let new_state = State {
                        amphipods: new_map.clone(),
                        score: state.score + score_delta,
                    };

                    heap.push(new_state);
                }
            }
        }
        0
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..=MAX_Y + 1 {
            for x in 0..=12 {
                if let Some(a) = self.amphipods.iter().find(|a| a.position == [x, y]) {
                    if let Err(e) = write!(f, "{}", a.letter()) {
                        return Err(e);
                    }
                } else if (y == 1 && (1..12).contains(&x))
                    || ((2..=MAX_Y).contains(&y) && [3, 5, 7, 9].contains(&x))
                {
                    if let Err(e) = write!(f, ".") {
                        return Err(e);
                    }
                } else if let Err(e) = write!(f, "#") {
                    return Err(e);
                }
            }
            writeln!(f).unwrap();
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

    // This one needs some adaptations on the length of each cave
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
  #D#C#B#A#
  #D#B#A#C#
  #B#A#B#C#
  #########";

    let mut burrow = Burrow::from_input(input2).unwrap();
    println!("Score: {}", burrow.sort());

    Ok(())
}
