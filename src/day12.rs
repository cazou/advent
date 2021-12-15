use std::cell::RefCell;
use std::rc::Rc;

struct Cave {
    name: String,
    neighbours: Vec<Rc<RefCell<Cave>>>
}

impl Cave {
    fn from_name(name: &String) -> Cave {
        Cave {
            name: name.clone(),
            neighbours: vec![]
        }
    }

    fn add_neighbour(&mut self, n: Rc<RefCell<Cave>>) {
        self.neighbours.push(n);
    }
}

struct Graph {
    start: Rc<RefCell<Cave>>
}

impl Graph {
    fn get_cave(caves: &mut Vec<Rc<RefCell<Cave>>>, name: &String) -> Rc<RefCell<Cave>> {
        match caves.iter().find(|x| *x.borrow().name == *name.as_str()) {
            Some(c) => Rc::clone(c),
            None => {
                let cave = Rc::new(RefCell::new(Cave::from_name(&name)));
                caves.push(Rc::clone(&cave));
                cave
            }
        }
    }

    pub fn from_data(graph: &str) -> Result<Graph, String> {
        let mut caves: Vec<Rc<RefCell<Cave>>> = vec![];
        let mut start: Option<Rc<RefCell<Cave>>> = None;

        for link in graph.lines() {
            let linked_caves: Vec<&str> = link.split('-').collect();
            let cave0 = Graph::get_cave(&mut caves, &String::from(linked_caves[0]));
            let cave1 = Graph::get_cave(&mut caves, &String::from(linked_caves[1]));
            cave0.borrow_mut().add_neighbour(Rc::clone(&cave1));
            cave1.borrow_mut().add_neighbour(Rc::clone(&cave0));

            if cave0.borrow().name == "start" {
                start = Some(Rc::clone(&cave0));
            }

            if cave1.borrow().name == "start" {
                start = Some(Rc::clone(&cave1));
            }
        }

        if let Some(s) = start {
            Ok(Graph {
                start: s
            })
        } else {
            Err(String::from("No start node found"))
        }
    }

    fn is_small_cave(name: &String) -> bool {
        &name.to_ascii_lowercase() == name &&
            &name.as_str() != &"start" &&
            &name.as_str() != &"end"
    }

    fn find_paths(s: &Cave, path: &mut Vec<String>, level: usize, path_list: &mut Vec<Vec<String>>, visited_twice: bool) {
        path.push(s.name.clone());

        if s.name == String::from("end") {
            path_list.push(path.clone());
            path.pop();
            return;
        }
        for n in &s.neighbours {
            if !path.contains(&n.borrow().name.to_ascii_lowercase()) {
                Graph::find_paths(&n.borrow(), path, level+1, path_list, visited_twice);
            } else if Graph::is_small_cave(&n.borrow().name) && !visited_twice {
                Graph::find_paths(&n.borrow(), path, level+1, path_list, true);
            }
        }

        path.pop();
    }

    pub fn find_all_paths(&self, include_double_visits: bool) -> Vec<Vec<String>> {

        let c = self.start.borrow();
        let mut path = vec![];
        let mut path_list = vec![];
        Graph::find_paths(&c, &mut path, 0, &mut path_list, !include_double_visits);

        path_list
    }
}

pub fn run(contents: &str) -> Result<(), String> {
    print!("[Caves]...       ");

    // Load input data
    let graph = match Graph::from_data(contents) {
        Ok(g) => g,
        Err(e) => {
            println!("{}", e);
            return Err(e);
        }
    };
    let paths1 = graph.find_all_paths(false);
    let paths2 = graph.find_all_paths(true);

    println!("{} {}", paths1.len(), paths2.len());

    Ok(())
}