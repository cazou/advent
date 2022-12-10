use std::collections::HashMap;

struct Die {
    roll_count: i32,
    size: i32,
}

impl Die {
    fn new(size: i32) -> Die {
        Die {
            roll_count: 0,
            size,
        }
    }

    fn roll(&mut self) -> i32 {
        let ret = (self.roll_count % self.size) + 1;
        self.roll_count += 1;

        ret
    }

    fn roll3(&mut self) -> i32 {
        self.roll() + self.roll() + self.roll()
    }
}

fn play(pos: &mut [i32; 2], scores: &mut [i32; 2], roll: i32, turn: usize) {
    pos[turn] = (pos[turn] + roll) % 10;
    scores[turn] += pos[turn] + 1;
}

pub fn run1() {
    print!("[Dirac]...     ");

    let start_pos = [5, 6];
    let winner_score = 1000;

    let mut pos = [start_pos[0] - 1, start_pos[1] - 1];
    let mut scores = [0, 0];
    let mut turn = 0;

    let mut d = Die::new(100);

    while scores[0] < winner_score && scores[1] < winner_score {
        let roll = d.roll3();
        play(&mut pos, &mut scores, roll, turn);
        turn = (turn + 1) % 2;
    }

    let p1 = d.roll_count * scores[turn];

    println!("{}", p1);
}

#[derive(Debug)]
struct Universe {
    count: usize,
    pos: [u8; 2],
    scores: [u8; 2],
    turn: u8,
}

impl Universe {
    fn winner(&self) -> Option<u8> {
        if self.scores[0] >= 21 {
            Some(0)
        } else if self.scores[1] >= 21 {
            Some(1)
        } else {
            None
        }
    }

    fn play_dirac(&self, roll: u32, count: usize) -> Universe {
        let mut new_pos = self.pos;
        new_pos[self.turn as usize] = ((self.pos[self.turn as usize] as u32 + roll) % 10) as u8;
        let mut new_scores = self.scores;
        new_scores[self.turn as usize] += (new_pos[self.turn as usize] + 1) as u8;

        Universe {
            pos: new_pos,
            scores: new_scores,
            turn: (self.turn + 1) % 2,
            count: self.count * count,
        }
    }
}

pub fn run2() {
    let start_pos = [5, 6];

    let start = Universe {
        pos: [start_pos[0] - 1, start_pos[1] - 1],
        scores: [0, 0],
        turn: 0,
        count: 1,
    };
    let mut universes = vec![start];
    let mut wins: [usize; 2] = [0, 0];
    let mut rolls: HashMap<u32, usize> = HashMap::new();
    for r1 in [1, 2, 3] {
        for r2 in [1, 2, 3] {
            for r3 in [1, 2, 3] {
                let roll = r1 + r2 + r3;
                *rolls.entry(roll).or_insert(0) += 1;
            }
        }
    }

    while !universes.is_empty() {
        let universe = universes.pop().unwrap();
        for (roll, count) in &rolls {
            let new = universe.play_dirac(*roll, *count);
            match new.winner() {
                Some(x) => wins[x as usize] += new.count,
                None => universes.push(new),
            };
        }
    }

    println!(
        "{} (the other one wins {} times)",
        wins.iter().max().unwrap(),
        wins.iter().min().unwrap()
    );
}

pub fn run() -> Result<(), String> {
    run1();
    run2();

    Ok(())
}
