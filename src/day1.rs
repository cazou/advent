use std::fs;

const WINDOW_SIZE: usize = 3;

struct Window {
    sum: usize,
    elem_count: usize,
}

impl Window {
    pub fn new() -> Window {
        Window {
            sum: 0,
            elem_count: 0
        }
    }

    ///
    /// Add a value and return true if this is the sum of 3 elements
    ///
    pub fn add_val(&mut self, val: usize) -> bool {
        self.sum += val;
        self.elem_count += 1;

        self.elem_count == WINDOW_SIZE
    }

    pub fn sum(&mut self) -> usize {
        self.sum
    }

    pub fn reset(&mut self) {
        self.sum = 0;
        self.elem_count = 0;
    }
}

/// return an option that is either:
/// None => No action to be taken
/// Some(val: usize) => val is the next window value to compare with the previous one.
fn add_value_in_windows(windows: &mut Vec<Window>, value: usize) -> Option<usize> {
    let mut ret = None;

    if windows.len() < WINDOW_SIZE {
        windows.push(Window::new())
    }

    for w in windows {
        if w.add_val(value) {
            ret = Some(w.sum());
            w.reset();
        }
    }

    ret
}

pub fn run(inputfile: &str) -> Result<(), String> {
    print!("[Sonar sweep]... ");

    let contents = match fs::read_to_string(inputfile) {
        Ok(c) => c,
        Err(e) =>
            return Err(e.to_string())
    };

    let mut windows: Vec<Window> = vec![];
    let mut last_val: Option<usize> = None;
    let mut increased_count = 0;

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        let val: usize = match line.parse() {
            Ok(v) => v,
            Err(e) => {
                println!("FAILED");
                return Err(e.to_string());
            }
        };

        let val = match add_value_in_windows(&mut windows, val) {
            Some(v) => v,
            None => continue
        };

        match last_val {
            Some(v) => {
                if val > v {
                    increased_count += 1;
                }
            },
            None => {}
        };

        last_val = Some(val);
    }

    println!("{}", increased_count);

    Ok(())
}
