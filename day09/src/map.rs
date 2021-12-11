// Point in the map i.e. x and y coordinates
pub type Point = (usize, usize);

#[derive(std::cmp::PartialOrd, std::cmp::PartialEq, std::cmp::Ord, std::cmp::Eq)]
pub struct Basin {
    size: usize,
    points: Vec<Point>,
}

impl Basin {
    pub fn new() -> Self {
        Self {
            size: 0,
            points: Vec::new(),
        }
    }

    pub fn add_point(&mut self, p: Point) {
        self.points.push(p);
        self.points.sort();
        self.size += 1;
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl std::fmt::Debug for Basin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current_row = self.points.iter().fold(9999999999, |min_r, p| if p.0 < min_r { p.0 } else { min_r });
        let minimum_column = self.points.iter().fold(9999999999, |min_c, p| if p.1 < min_c { p.1 } else { min_c });
        let mut current_column = minimum_column;
        for p in self.points.iter() {
            while current_row < p.0 {
                writeln!(f, "")?;
                current_row += 1;
                current_column = minimum_column;
            }
            while current_column < p.1 {
                write!(f, " ")?;
                current_column += 1;
            }
            write!(f, "#")?;
            current_column += 1;
        }
        writeln!(f)
    }
}

#[derive(Debug, Clone)]
// Map holding a matrix of u32 elements
pub struct Map {
    pub elements: Vec<Vec<u32>>,
}

impl Map {
    /// Try to get an element from the map (negative index allowed)
    /// returning None if the element doesn't exist otherwise returns
    /// Some containing the element
    ///
    /// # Arguments
    ///
    /// * `i` row of element to try and get
    /// * `j` column of element to try and get
    pub fn try_get(&self, i: isize, j: isize) -> Option<&u32> {
        if self.elements.is_empty() {
            None
        }
        else if i < 0 || j < 0 || i >= self.elements.len() as isize || j >= self.elements[0].len() as isize {
            None
        } else {
            Some(&self.elements[i as usize][j as usize])
        }
    }

    /// Returns a vector containing all low points in the map.
    ///
    /// A low point is defined as being surrounded by only 'higher' points
    /// i.e. those with a greater value. Only points directly above, below
    /// or to the sides are considered (not diagonals)
    pub fn find_low_points(&self) -> Vec<Point> {
        let mut p = Vec::new();

        for (i, r) in self.elements.iter().enumerate() {
            for (j, e) in r.iter().enumerate() {
                let mut result = true;
                for (x, y) in self.adjacent_to((i, j)) {
                    result &= self.elements[x][y] > *e;
                }
                if result {
                    p.push((i, j));
                }
            }
        }
        p
    }

    /// Find all basins in the map returning a vector conti
    pub fn find_basins(&self) -> Vec<Basin> {
        if self.elements.is_empty() {
            return Vec::new()
        }
        // Create a mutable copy for out search
        let mut copy = self.clone();

        // Perform recursive search - modifying the copy as each point is evaluated
        fn search_basin(map: &mut Map, point: Point) -> Vec<Point> {
            let mut points = vec!(point);
            for p in map.adjacent_to(point) {
                if map.elements[p.0][p.1] != 9 {
                    map.elements[p.0][p.1] = 9;
                    points.append(&mut search_basin(map, p));
                }
            };
            points
        }

        let mut basins = Vec::new();
        for i in 0..copy.elements.len() {
            for j in 0..copy.elements[0].len() {
                if copy.elements[i][j] != 9 {
                    let mut basin = Basin::new();
                    for p in search_basin(&mut copy, (i, j)) {
                        basin.add_point(p);
                    }
                    basins.push(basin);
                }
            }
        }
        basins
    }

    pub fn adjacent_to(&self, p: Point) -> Vec<Point> {
        let (i, j) = (p.0 as isize, p.1 as isize);
        if self.elements.is_empty() {
            return Vec::new()
        }

        let mut p = Vec::new();
        if let Some(_) = self.try_get(i+1, j) {
            p.push((i as usize + 1, j as usize));
        }
        if let Some(_) = self.try_get(i, j-1) {
            p.push((i as usize, j as usize - 1));
        }
        if let Some(_) = self.try_get(i, j+1) {
            p.push((i as usize, j as usize + 1));
        }
        if let Some(_) = self.try_get(i-1, j) {
            p.push((i as usize - 1, j as usize));
        }
        p
    }
}