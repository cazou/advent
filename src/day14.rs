use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

struct Rule {
    from: Vec<char>,
    to: char
}

impl Rule {
    pub fn apply(&self, k: &String) -> Option<[Vec<char>; 2]> {
        let key: Vec<char> = k.chars().collect();
        if self.from == key {
            return Some([vec![self.from[0], self.to], vec![self.to, self.from[1]]]);
        }

        None
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<&str> = s.split(" -> ").collect();
        let base = vals[0];

        let mut to = base.to_string().clone();
        to.insert_str(1, vals[1]);

        let to_char: Vec<char> = vals[1].chars().collect();

        Ok(Rule {
            from: base.chars().collect(),
            to: to_char[0],
        })
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} -> {}", self.from, self.to)
    }
}

fn count_letters(template: &Vec<char>, rules: &Vec<Rule>, steps: usize) -> usize {
    let mut map: HashMap<String, usize> = HashMap::new();
    for i in 0..template.len()-1 {
        map.insert(String::from_iter(&template[i..i+2]), 1);
    }

    let mut new_map: HashMap<String, usize> = map.clone();

    for _ in 0..steps {
        for (k, v) in &map {
            if *v == 0 {
                continue;
            }
            for r in rules {
                match &r.apply(k) {
                    None => {}
                    Some(a) => {
                        let key1 = String::from_iter(a[0].iter());
                        let key2 = String::from_iter(a[1].iter());

                        // Add new elements
                        match new_map.get(&key1) {
                            None => {
                                new_map.insert(key1, *v);
                            },
                            Some(n) => {
                                let new_n = *n;
                                new_map.insert(key1, new_n + *v);
                            },
                        };

                        match new_map.get(&key2) {
                            None => {
                                new_map.insert(key2, *v);
                            },
                            Some(n) => {
                                let new_n = *n;
                                new_map.insert(key2, new_n + *v);
                            },
                        };

                        // Decrease this key
                        if let Some(n) = new_map.get(k) {
                            let new_n = *n;
                            new_map.insert(k.clone(), new_n - *v);
                        }
                    }
                }
            }
        }

        map = new_map.clone();
    }

    let mut counts: HashMap<char, usize> = HashMap::new();

    for c in 'A'..'Z' {
        counts.insert(c, 0);
    }

    for (x, y) in &map {
        for c in x.chars() {
            if let Some(v) = counts.get_mut(&c) {
                *v += *y;
            }
        }
    }

    counts.retain(|_, &mut v| v > 0);
    counts.iter_mut().for_each(|(_, b)| *b = (*b as f64 / 2.0).ceil() as usize);

    let max = counts.iter().max_by(|a,b| a.1.cmp(b.1)).unwrap();
    let min = counts.iter().min_by(|a,b| a.1.cmp(b.1)).unwrap();

    max.1 - min.1
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Poly]...        ");

    let mut template: Vec<char> = vec![];
    let mut rules = vec![];

    // Load input data
    for val in contents.lines() {
        if val.is_empty() {
            continue;
        }

        if template.is_empty() {
            template = val.chars().collect();
        } else {
            rules.push(Rule::from_str(val).unwrap());
        }

    }

    print!("{} ", count_letters(&template, &rules, 10));
    println!("{}", count_letters(&template, &rules, 40));

    Ok(())
}