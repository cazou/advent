use crate::traits::AdventOfCode;
use anyhow::{bail, Result};
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

enum Command {
    Ls,
    Cd(String),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.starts_with("$ ls") {
            Ok(Command::Ls)
        } else if s.starts_with("$ cd") {
            Ok(Command::Cd(s.split_at(5).1.to_string()))
        } else {
            bail!("Invalid command {}", s);
        }
    }
}

#[derive(Default)]
struct Dir {
    name: String,
    files: Vec<(String, usize)>,
    dirs: Vec<Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
    size: Option<usize>,
}

impl Dir {
    fn cd(rel_dir: &Rc<RefCell<Dir>>, dir: &str) -> Rc<RefCell<Dir>> {
        if dir == ".." {
            Rc::clone(rel_dir.borrow().parent.as_ref().unwrap())
        } else if dir == "/" {
            // Recursively go to "/"
            if rel_dir.borrow().parent.as_ref().is_some() {
                Self::cd(rel_dir.borrow().parent.as_ref().unwrap(), dir)
            } else {
                Rc::clone(rel_dir)
            }
        } else {
            Rc::clone(
                rel_dir
                    .borrow()
                    .dirs
                    .iter()
                    .find(|d| d.borrow().name == dir)
                    .unwrap(),
            )
        }
    }

    // Create a file
    fn touch(&mut self, name: &str, size: usize) {
        self.files.push((name.to_string(), size));
    }

    // Create a dir
    fn mkdir(dir: &Rc<RefCell<Dir>>, sub_name: &str) {
        let parent = Some(Rc::clone(dir));
        dir.borrow_mut().dirs.push(Rc::new(RefCell::new(Dir {
            name: sub_name.to_string(),
            parent,
            ..Default::default()
        })));
    }

    fn size(&mut self) -> usize {
        if self.size.is_some() {
            return self.size.unwrap();
        }

        let mut size = self.files.iter().fold(0, |i, a| i + a.1);
        for dir in &mut self.dirs {
            size += dir.borrow_mut().size();
        }

        self.size = Some(size);

        size
    }

    fn max_size(dir: &mut Rc<RefCell<Dir>>, max_size: usize, acc: &mut Vec<Rc<RefCell<Dir>>>) {
        if dir.borrow_mut().size() <= max_size {
            acc.push(Rc::clone(dir))
        }

        for sub in &mut dir.borrow_mut().dirs {
            Self::max_size(sub, max_size, acc);
        }
    }

    fn min_size(dir: &mut Rc<RefCell<Dir>>, max_size: usize, acc: &mut Vec<Rc<RefCell<Dir>>>) {
        if dir.borrow_mut().size() >= max_size {
            acc.push(Rc::clone(dir))
        }

        for sub in &mut dir.borrow_mut().dirs {
            Self::min_size(sub, max_size, acc);
        }
    }
}

struct FileSystem {
    root: Rc<RefCell<Dir>>,
}

impl FileSystem {
    fn max_size(&mut self, max_size: usize) -> usize {
        let mut dirs = vec![];
        Dir::max_size(&mut self.root, max_size, &mut dirs);

        dirs.iter().fold(0, |acc, e| acc + e.borrow_mut().size())
    }

    fn find_dir_to_delete(&mut self) -> usize {
        let fs_size = 70000000;
        let update_size = 30000000;
        let used_space = self.root.borrow_mut().size();
        let needed_space = update_size - (fs_size - used_space);

        let mut candidates = vec![];
        Dir::min_size(&mut self.root, needed_space, &mut candidates);

        candidates.sort_by_key(|c| c.borrow_mut().size());

        return candidates.first().unwrap().borrow_mut().size();
    }

    fn is_command(line: &str) -> bool {
        line.starts_with("$ ")
    }
}

impl FromStr for FileSystem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Rc::new(RefCell::new(Dir {
            name: "/".to_string(),
            ..Default::default()
        }));

        let mut current_dir = Rc::clone(&root);
        let mut current_cmd: Option<Command> = None;

        for line in s.lines() {
            match &current_cmd {
                None => {
                    current_cmd = Some(line.parse()?);
                }
                Some(Command::Ls) => {
                    if Self::is_command(line) {
                        current_cmd = Some(line.parse()?);
                    } else if line.starts_with("dir ") {
                        Dir::mkdir(&current_dir, line.split_at(4).1);
                    } else {
                        let info: Vec<&str> = line.splitn(2, ' ').collect();
                        current_dir.borrow_mut().touch(info[1], info[0].parse()?);
                    }
                }
                _ => {}
            }

            if let Some(Command::Cd(dir)) = &current_cmd {
                current_dir = Dir::cd(&current_dir, dir);
                current_cmd = None;
            }
        }

        Ok(FileSystem {
            root: Rc::clone(&root),
        })
    }
}

pub struct Day7;

impl AdventOfCode for Day7 {
    fn day(&self) -> u8 {
        7
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let mut fs: FileSystem = input.unwrap().parse().unwrap();
        Ok(fs.max_size(100000).to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut fs: FileSystem = input.unwrap().parse().unwrap();
        Ok(fs.find_dir_to_delete().to_string())
    }
}
