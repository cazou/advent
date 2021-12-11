
fn points(t: char) -> usize {
    match t {
        ']' => 57,
        '}' => 1197,
        ')' => 3,
        '>' => 25137,
        _ => 0
    }
}

fn score_map(t: char) -> usize {
    match t {
        ']' => 2,
        '}' => 3,
        ')' => 1,
        '>' => 4,
        _ => 0
    }
}

fn closing(t: char) -> char {
        match t {
            '[' => ']',
            '{' => '}',
            '(' => ')',
            '<' => '>',
        _ => 'X'
    }
}

fn is_opening(t: char) -> bool {
    match t {
        ']' | '}' | ')' | '>' => false,
        _ => true
    }
}

fn line_check(val: &str) -> [usize; 2] {
    let mut stack: Vec<char> = vec![];
    let mut p = 0;
    let mut score = 0;

    for c in val.chars() {
        if is_opening(c) {
            stack.push(c)
        } else {
            match stack.pop() {
                Some(t) => {
                    if c != closing(t) {
                        p = points(c);
                        break;
                    }
                },
                None => println!("Line is too long")
            }
        }
    }

    if p == 0 {
        while !stack.is_empty() {
            score *= 5;
            score += score_map(closing(stack.pop().unwrap()));
        }
    }

    [p, score]
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Syntax]...      ");

    let mut points = 0;
    let mut scores = vec![];

    // Load input data
    for val in contents.lines() {
        if val.is_empty() {
            continue;
        }

        let [p, s] = line_check(val);
        points += p;

        if p == 0 {
            scores.push(s);
        }
    }

    scores.sort();

    let middle_score = scores[scores.len() / 2];

    println!("{} {}", points, middle_score);

    Ok(())
}