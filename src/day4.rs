
const BOARD_SIZE: usize = 5;

#[derive(Copy, Clone)]
struct BoardElement {
    pub value: usize,
    pub checked: bool
}

impl BoardElement {
    pub fn new(value: usize) -> BoardElement {
        BoardElement {
            value,
            checked: false
        }
    }
}

struct Board {
    values: [[BoardElement; BOARD_SIZE]; BOARD_SIZE],
    row_check_count: [usize; BOARD_SIZE],
    col_check_count: [usize; BOARD_SIZE],
    next_line: usize
}

impl Board {
    pub fn new() -> Board {
        Board {
            values: [[BoardElement {value: 0, checked: false}; 5]; 5],
            next_line: 0,
            row_check_count: [0; BOARD_SIZE],
            col_check_count: [0; BOARD_SIZE],
        }
    }

    pub fn add_line(&mut self, line_txt: &str) {
        let mut elems = &mut self.values[self.next_line];
        let mut i = 0;

        for v in line_txt.split(' ') {
            if v.is_empty() {
                continue;
            }

            elems[i].value = v.parse().unwrap();
            i += 1;
        }

        self.next_line += 1;
    }

    pub fn ready(&self) -> bool {
        self.next_line == BOARD_SIZE
    }

    pub fn check_value(&mut self, val: usize) -> bool {
        let mut row: usize = 0;
        for x in &mut self.values {
            let mut col: usize = 0;
            for y in x {
                if y.value == val {
                    y.checked = true;
                    self.col_check_count[col] += 1;
                    self.row_check_count[row] += 1;

                    // This considers that a board has the same value only once
                    return self.col_check_count[col] == BOARD_SIZE ||
                            self.row_check_count[row] == BOARD_SIZE;
                }
                col += 1;
            }
            row += 1;
        }

        false
    }

    pub fn unchecked_sum(&self) -> usize {
        let mut sum = 0;
        for x in & self.values {
            for y in x {
                if !y.checked {
                    sum += y.value;
                }
            }
        }

        sum
    }
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Bingo]...       ");

    let mut first_line = true;
    let mut boards = vec![];
    let mut values= "";

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        if first_line {
            first_line = false;

            values = line.clone();

            boards.push(Board::new());

            continue
        }

        let mut board = boards.last_mut().unwrap();
        board.add_line(line);

        if board.ready() {
            boards.push(Board::new());
        }
    }

    let mut first_winner: Option<usize> = None;
    let mut last_winner: Option<usize> = None;

    for v in values.split(',') {
        if v.is_empty() {
            continue;
        }

        let v: usize = v.parse().unwrap();
        let mut i: usize = 0;
        let mut to_remove: Vec<usize> = vec![];

        for board in &mut boards {
            if board.check_value(v) {
                // This is a winning board
                first_winner = match first_winner {
                    None => Some(v * board.unchecked_sum()),
                    _ => first_winner
                };

                last_winner = Some(v * board.unchecked_sum());

                to_remove.push(i);
            }
            i += 1;
        }

        // Start by removing the last one
        to_remove.reverse();

        for idx in to_remove {
            boards.remove(idx);
        }
    }

    println!("First: {}, Last: {}", first_winner.unwrap(), last_winner.unwrap());

    Ok(())
}