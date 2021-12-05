use utils::{files, parse_field};
use std::collections::HashMap;


#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Debug)]
pub enum ParsePointError {
    ParseError,
}

impl std::str::FromStr for Point {
    type Err = ParsePointError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_field!(s => u32, "," | u32, "") {
            (Some(x), Some(y)) => Ok(Self::new(x, y)),
            _ => Err(Self::Err::ParseError),
        }
    }
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Self {
            p1,
            p2,
        }
    }

    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn is_diagonal(&self) -> bool {
        !self.is_horizontal() && !self.is_vertical()
    }

    fn generate_points(&self) -> Vec<Point> {
        let x_st = self.p1.x as i32;
        let x_ed = self.p2.x as i32;
        let y_st = self.p1.y as i32;
        let y_ed = self.p2.y as i32;

        let x_range = x_st - x_ed;
        let y_range = y_st - y_ed;
        if x_range != 0 && y_range != 0 && x_range.abs() != y_range.abs() {
            println!("CANNOT RETURN POINTS FOR LINE '{:?}'", self);
            println!("x_range {} : y_range {}", x_range, y_range);
        }

        let mut points = Vec::new();
        let x_iter = if x_st == x_ed { 0 } else if x_st < x_ed { 1 } else { -1 };
        let y_iter = if y_st == y_ed { 0 } else if y_st < y_ed { 1 } else { -1 };
        let mut x = x_st;
        let mut y = y_st;
        loop {
            points.push(Point::new(x as u32, y as u32));
            if x == x_ed && y == y_ed {
                break;
            }
            x += x_iter;
            y += y_iter;
        }
        points
    }
}

#[derive(Debug)]
pub enum ParseLineError {
    ParseError,
}

impl std::str::FromStr for Line {
    type Err = ParsePointError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_field!(s => Point, " -> " | Point, "") {
            (Some(p1), Some(p2)) => Ok(Self::new(p1, p2)),
            _ => Err(Self::Err::ParseError),
        }
    }
}

fn main() {
    let filename = "src/input.txt";
    let lines = files::read_in_lines_as::<Line>(filename);

    println!("----- PART 1 -----");
    let mut points = HashMap::new();
    for line in lines.iter() {
        if line.is_horizontal() {
            for p in line.generate_points() {
                let v = points.entry(p).or_insert(0);
                *v += 1;
            }
        } else if line.is_vertical() {
            for p in line.generate_points() {
                let v = points.entry(p).or_insert(0);
                *v += 1;
            }
        } else {
            // Must be diagonal
            ()
        }
    }

    let mut count = 0;
    for (_, v) in points.iter() {
        if *v >= 2 {
            count += 1;
        }
    }

    println!("Part 1 Answer: {}", count);

    println!("\n\n----- PART 2 -----");

    for line in lines.iter() {
        if line.is_diagonal() {
            for p in line.generate_points() {
                let v = points.entry(p).or_insert(0);
                *v += 1;
            }
        }
    }

    let mut count = 0;
    for (_, v) in points.iter() {
        if *v >= 2 {
            count += 1;
        }
    }

    println!("Part 2 Answer: {}", count);

}
