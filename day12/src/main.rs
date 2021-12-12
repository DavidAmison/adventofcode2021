use utils::{files, parse_field_unwrap};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

type Fields = (String, String);

fn main() {
    let filename = "input";
    let input = files::read_in_lines(filename);
    let connections: Vec<Fields> = input.iter().map(|l| parse_field_unwrap!(l => String, "-" | String, "")).collect();

    // Get list of unique caves
    let mut cave_ids: Vec<&String> = connections.iter().map(|(l, r)| vec!(l, r)).flatten().collect();
    cave_ids.sort();
    cave_ids.dedup();

    // Create all caves and store in HashMap for access by id (makes finding each one easier)
    // Each cave is stored in an Rc<RefCell<T>> to allow interior mutability
    let create_cave = |id: &str| {
        if id.to_uppercase() == id {
            // BIG CAVE
            Cave::new(id, CaveType::Big)
        } else {
            // SMALL CAVE
            Cave::new(id, CaveType::Small(CaveState::NotVisited))
        }
    };
    let mut caves: HashMap<String, Rc<RefCell<Cave>>> = cave_ids.iter()
        .map(|id| (String::from(*id), create_cave(id)))
        .collect();

    // Add connections to all caves
    for (l, r) in connections.iter() {
        let c1 = caves.get(l).unwrap();
        let c2 = caves.get(r).unwrap();
        // Start can only have outgoing connections
        // End can only have incoming connections
        if c1.borrow().id() != "end" && c2.borrow().id() != "start" {
            c1.borrow_mut().add_connection(c2.clone());
        }
        if c2.borrow().id() != "end" && c1.borrow().id() != "start" {
            c2.borrow_mut().add_connection(c1.clone());
        }
    }

    println!("----- PART 1 -----");

    fn get_routes(caves: &HashMap<String, Rc<RefCell<Cave>>>, start: &Rc<RefCell<Cave>>) -> Vec<Vec<String>> {
        let mut routes = Vec::new();
        start.borrow_mut().visit();

        // Check if is end and return early
        if start.borrow().id() == "end" {
            routes.push(vec!(String::from("end")));
        }

        let visitable = start.borrow().can_visit(false);
        for c in visitable.iter() {
            for route in get_routes(caves, c).iter_mut().filter(|r| r.last() == Some(&String::from("end"))) {
                // println!("{:?}", route);
                route.insert(0, String::from(start.borrow().id()));
                routes.push(route.clone());
            }
        }
        start.borrow_mut().unvisit();
        routes
    }

    // Start from start
    let start = caves.get("start").unwrap();
    let routes = get_routes(&caves, start);
    // for route in routes.iter() {
    //     println!("{:?}", route);
    // }

    println!("Part 1 Answer: {}", routes.len());

    println!("\n\n----- PART 2 -----");
    // We must walk over only one route at a time! We are going to achieve this by
    // using a single vector representing the current taken route and pushing to
    // and popping from that vector as we move cave to cave.
    // While I feel the concept is elegant the execution feels lacking in neatness!
    // Potentially this can be refactored as recursive - but need to be wary of stack
    // overflows (i.e. make sure that tail-call optimization is applicable)
    let mut count = 0;
    let mut route = vec!(start.clone());
    let mut last_popped: Option<String> = None;
    let mut allow_double_visit = (true, String::new());
    while let Some(current_cave) = route.pop() {   // Pop to avoid borrow checking errors later
        // for cave in route.iter() {
        //     print!("{}{}->", cave.borrow().id(), if cave.borrow().visited() { "'" } else { "" });
        // }
        // println!("{}", current_cave.borrow().id());
        if last_popped == None {
            // Try and move forward
            if let Some(next_cave) = current_cave.borrow().can_visit(allow_double_visit.0).first() {
                // We can visit one small cave twice
                if next_cave.borrow_mut().visited() {
                    allow_double_visit.0 = false;
                    allow_double_visit.1 = String::from(next_cave.borrow().id());
                }

                // We have to repush the current cave as well since we
                // popped it at the top of the loop
                next_cave.borrow_mut().visit();
                route.push(current_cave.clone());
                route.push(next_cave.clone());
            } else {
                // No caves left to visit - pop one if possible
                // But first check if we are at `end`
                if current_cave.borrow().id() == "end" {
                    count += 1;
                    // print!("ROUTE {}:   ", count);
                    // for cave in route.iter() {
                    //     print!("{}{}->", cave.borrow().id(), if cave.borrow().visited() { "'" } else { "" });
                    // }
                    // println!("{}", current_cave.borrow().id());
                }

                last_popped = Some(String::from(current_cave.borrow().id()));
            }
            if last_popped.is_some() {
                // Check if we have freed the node that was double visited
                // if so we don't unvisit or get infinite loop syndrome
                if current_cave.borrow().id() == allow_double_visit.1 {
                    allow_double_visit = (true, String::new());
                } else {
                    current_cave.borrow_mut().unvisit();
                }
            }

        } else {
            // We just moved backwards through the list so lets try a different route
            let mut take_next = false;
            for next_cave in current_cave.borrow().can_visit(allow_double_visit.0).iter() {
                if Some(String::from(next_cave.borrow().id())) == last_popped {
                    take_next = true;
                    last_popped = None;
                } else if take_next {
                    // println!("Visiting(2) {:?}", next_cave.borrow().id());

                    // Check if this would be a double visit
                    if next_cave.borrow_mut().visited() {
                        allow_double_visit.0 = false;
                        allow_double_visit.1 = String::from(next_cave.borrow().id());
                        // println!("Trapped(2) {:?}", allow_double_visit);
                    }

                    // We have to repush the current cave as well since we
                    // popped it at the top of the loop
                    next_cave.borrow_mut().visit();
                    route.push(current_cave.clone());
                    route.push(next_cave.clone());

                    take_next = false;  // Reset flag to prevent popping below
                    break;
                }
            }

            if take_next {
                // We reached the end of the list so we will need to pop again
                last_popped = Some(String::from(current_cave.borrow().id()));

                // Check if we have freed the node that was double visited
                // We don't unvisit that cave otherwise we cause problems
                if current_cave.borrow().id() == allow_double_visit.1 {
                    allow_double_visit = (true, String::new());
                } else {
                    current_cave.borrow_mut().unvisit();
                }
            }
        }
    }
    println!("Part 2 Answer: {}", count);

}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CaveState {
    Visited,
    NotVisited,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CaveType {
    Big,
    Small(CaveState),
}


#[derive(PartialEq)]
struct Cave {
    id: String,
    ty: CaveType,
    connected: Vec<Rc<RefCell<Cave>>>
}

impl std::fmt::Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cave")
            .field("id", &self.id)
            .field("type", &self.ty)
            .field("connected", &self.connected.iter().map(|c| String::from(c.borrow().id())).collect::<Vec<String>>())
            .finish()
    }
}

impl Cave {
    pub fn new(id: &str, ty: CaveType) -> Rc<RefCell<Self>> {
        let c = Self {
            id: String::from(id),
            ty,
            connected: Vec::new(),
        };
        Rc::new(RefCell::new(c))
    }

    pub fn add_connection(&mut self, c: Rc<RefCell<Cave>>) {
        self.connected.push(c);
    }

    pub fn can_visit(&self, allow_all: bool) -> Vec<Rc<RefCell<Cave>>> {
        // Filter out caves that have already been visited
        self.connected.iter()
            .filter(|c| allow_all || !c.borrow().visited())
            .map(|c| c.clone())
            .collect()
    }

    pub fn visit(&mut self) {
        match self.cave_type() {
            CaveType::Small(_) => self.ty = CaveType::Small(CaveState::Visited),
            _ => (),
        }
    }

    pub fn unvisit(&mut self) {
        match self.cave_type() {
            CaveType::Small(_) => self.ty = CaveType::Small(CaveState::NotVisited),
            _ => (),
        }
    }

    pub fn visited(&self) -> bool {
        match self.cave_type() {
            CaveType::Small(CaveState::Visited) => true,
            _ => false,
        }
    }

    pub fn cave_type(&self) -> CaveType {
        self.ty
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

struct BranchedNode {
    id: String,
    nodes: Vec<BranchedNode>,
}

impl BranchedNode {
    pub fn new(id: &str) -> Self {
        Self {
            id: String::from(id),
            nodes: Vec::new(),
        }
    }

    pub fn add_branch(&mut self, node: BranchedNode) {
        self.nodes.push(node);
    }
}
