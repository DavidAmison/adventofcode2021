pub enum Octopus {
    Charging(u32),
    Charged,
    Flashing,
}

impl Octopus {
    pub fn step(&mut self) {
        match self {
            Self::Charging(x) if *x < 9 => *self = Self::Charging(*x+1),
            Self::Charging(_) => *self = Self::Charged,
            Self::Charged => (),
            Self::Flashing => (),
        }
    }

    pub fn is_ready(&self) -> bool {
        match self {
            Self::Charged => true,
            _ => false,
        }
    }

    pub fn is_flashing(&self) -> bool {
        match self {
            Self::Flashing => true,
            _ => false
        }
    }

    /// Returns true if octopus actually flashed - false otherwise
    pub fn flash(&mut self) -> bool {
        match self {
            Self::Charged => { *self = Self::Flashing; true },
            _ => false,
        }
    }

    /// Reset flashing octopi to energy level of 0
    pub fn reset(&mut self) {
        match self {
            Self::Flashing => *self = Self::Charging(0),
            _ => (),
        }
    }
}

impl std::fmt::Debug for Octopus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Charging(x) => write!(f, "{}", x),
            Self::Charged => write!(f, "C"),
            Self::Flashing => write!(f, "F"),
        }
    }
}

pub type Point = (isize, isize);

pub struct OctopiMatrix {
    pub octopi: Vec<Vec<Octopus>>,
}

impl OctopiMatrix {
    pub fn new() -> Self {
        Self {
            octopi: Vec::new(),
        }
    }

    /// Try to get an element from the map (negative index allowed)
    /// returning None if the element doesn't exist otherwise returns
    /// Some containing the element
    ///
    /// # Arguments
    ///
    /// * `i` row of element to try and get
    /// * `j` column of element to try and get
    pub fn try_get(&self, p: Point) -> Option<&Octopus> {
        if self.octopi.is_empty() {
            None
        }
        else if p.0 < 0 || p.1 < 0 || p.0 >= self.octopi.len() as isize || p.1 >= self.octopi[0].len() as isize {
            None
        } else {
            Some(&self.octopi[p.0 as usize][p.1 as usize])
        }
    }

    /// Try to get an element from the map (negative index allowed)
    /// returning None if the element doesn't exist otherwise returns
    /// Some containing a mutable reference to the element
    ///
    /// # Arguments
    ///
    /// * `p` (i: isize, j: isize) row (i) and column (j) of element to try and get
    pub fn try_get_mut(&mut self, p: Point) -> Option<&mut Octopus> {
        if self.octopi.is_empty() {
            None
        }
        else if p.0 < 0 || p.1 < 0 || p.0 >= self.octopi.len() as isize || p.1 >= self.octopi[0].len() as isize {
            None
        } else {
            Some(&mut self.octopi[p.0 as usize][p.1 as usize])
        }
    }

    pub fn adjacent_to(&self, p: Point) -> Vec<Point> {
        let mut points = Vec::new();
        for i in (p.0-1)..=(p.0+1) {
            for j in (p.1-1)..=(p.1+1) {
                if i == p.0 && j == p.1 { continue; }
                if let Some(_) = self.try_get((i, j)) {
                    points.push((i, j));
                }
            }
        }
        points
    }

    pub fn count_flashing(&self) -> u32 {
        self.octopi.iter()
            .flatten()      // Flatten all rows into one vector
            .fold(0, |count, o| if o.is_flashing() { count + 1 } else { count } )    // Count flashing octpi
    }

    /// Returns the number of octopi flashing at the end of this step
    pub fn step(&mut self) -> u32 {
        for row in self.octopi.iter_mut() {
            for o in row.iter_mut() {
                o.step();
            }
        }

        let mut flashed = 0;
        while let Some(count) = self.trigger_flashes() {
            flashed += count;
        }

        // Reset flashed to 0
        for o in self.octopi.iter_mut().flatten() {
            o.reset();
        }

        flashed
    }

    /// Returns the number of octopi that flashed
    fn trigger_flashes(&mut self) -> Option<u32> {
        let mut count = 0;
        let mut to_step = Vec::new();
        for i in 0..self.octopi.len() as isize {
            for j in 0..self.octopi[0].len() as isize {
                if self.try_get((i, j)).unwrap().is_ready() {
                    to_step.append(&mut self.adjacent_to((i, j)));
                    if self.try_get_mut((i, j)).unwrap().flash() { count += 1 } else { println!("Tried to flash octopus that wasn't charged"); }
                }
            }
        }

        for o in to_step {
            self.try_get_mut(o).unwrap().step();
        }
        // println!("Triggered {} flashes", count);
        if count == 0 {
            None
        } else {
            Some(count)
        }
    }
}

