
struct Die {
    roll_count: i32,
    size: i32
}

impl Die {
    fn new(size: i32) -> Die {
        Die {
            roll_count: 0,
            size
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

pub fn run() -> Result<(), String> {
    print!("[Dirac]...     ");

    let start_pos = [5, 6];
    let winner_score = 1000;

    let mut pos = [start_pos[0]-1, start_pos[1]-1];
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

    Ok(())
}