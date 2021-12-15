use utils::{files, parse_field_unwrap};

fn main() {
    let filename = "input";
    let input = files::read_in_matrix_as::<usize>(filename);

    let mut map: Vec<Vec<(usize, Distance)>> = input.iter().map(
        |r| r.iter().map(|v| (*v, Distance::Infinite)).collect()
    ).collect();

    println!("----- PART 1 -----");
    // Calculate the shortest distance - we will use Dijkstra's algorithm
    map[0][0].1 = Distance::Value(0);
    let rows = map.len();
    let cols = map[0].len();
    let result = dijkstra(&mut map, (0, 0), (rows-1, cols-1));
    // for row in map.iter() {
    //     println!("{:?}", row);
    // }

    println!("Part 1 Answer: {:?}", result);

    println!("\n\n----- PART 2 -----");
    let repeats: usize = 5;
    // Generate 5x5 version of the input map
    let mut tiles: Vec<Map> = Vec::new();
    for i in 0..repeats {
        for j in 0..repeats {
            tiles.push(input.iter()
                .map(|r| r.iter()
                    .map( |v|
                        if *v + j + i > 9 {
                            (*v + i + j - 9, Distance::Infinite)
                        } else {
                            (*v + i + j, Distance::Infinite)
                        }
                    ).collect()
                ).collect()
            );
        }
    }

    // for tile in tiles.iter() {
    //     for row in tile.iter() {
    //         for e in row.iter() {
    //             print!("{}", e.0);
    //         }
    //         println!();
    //     }
    //     println!();
    // }

    let mut big_map = Vec::new();
    for i in 0..repeats {
        for j in 0..repeats {
            for (r, row) in tiles[i*repeats + j].iter_mut().enumerate() {
                if j == 0 {
                    big_map.push(row.clone())
                } else {
                    big_map[i*cols + r].append(row)
                }
            }
        }
    }

    // for row in big_map.iter() {
    //     for e in row.iter() {
    //         print!("{}", e.0);
    //     }
    //     println!();
    // }
    // println!();

    let rows = big_map.len();
    let cols = big_map[0].len();
    big_map[0][0].1 = Distance::Value(0);
    let result = dijkstra(&mut big_map, (0, 0), (rows-1, cols-1));

    // for row in big_map.iter() {
    //     for e in row.iter() {
    //         print!("{:?} ", e.1);
    //     }
    //     println!();
    // }
    // println!();

    println!("Part 2 Answer: {:?}", result);

}

fn dijkstra(points: &mut Map, start: Point, end: Point) -> Distance {
    // Not highly optimised but good enough
    // Get co-ordinate of minimum distance point
    let mut points_to_visit: Vec<Point> = Vec::new();
    points_to_visit.push(start);
    while let Some(s) = find_minimum_distance_point(points, &mut points_to_visit) {
        let d_s = points[s.0][s.1].1.inner().unwrap();
        for p in adjacent_to(points, s) {
            let risk = points[p.0][p.1].0;
            match points[p.0][p.1].1 {
                Distance::Infinite => {
                    points[p.0][p.1].1 = Distance::Value(risk + d_s);
                    points_to_visit.push((p.0, p.1));
                }
                Distance::Value(x) => {
                    let d_p = x;
                    if d_p > risk + d_s {
                        points[p.0][p.1].1 = Distance::Value(risk + d_s);
                    }
                }
                Distance::Out(_) => (),
            }
        }
        points[s.0][s.1].1.to_out();
    }
    points[end.0][end.1].1
}

// Return row and column of the point with the minimum distance
fn find_minimum_distance_point(map: &Map, points: &mut Vec<Point>) -> Option<Point> {
    let mut minimum = (0, Distance::Infinite);
    for (n, (i, j)) in points.iter().enumerate() {
        let p = map[*i][*j];
        if p.1.is_value() && p.1 < minimum.1 {
            minimum = (n, p.1);
        }
    }

    if minimum.1.is_infinite() {
        None
    } else {
        Some(points.remove(minimum.0))
    }
}

fn adjacent_to(points: &Map, p: Point) -> Vec<Point> {
    let (i, j) = (p.0 as isize, p.1 as isize);
    if points.is_empty() {
        return Vec::new()
    }

    let mut p = Vec::new();
    let rows = points.len() as isize;
    let columns = points[0].len() as isize;

    if (i-1) >= 0 && (i-1) < rows && j >= 0 && j < columns {
        p.push((i as usize - 1, j as usize));
    }

    if i >= 0 && i < rows && (j-1) >= 0 && (j-1) < columns {
        p.push((i as usize, j as usize - 1));
    }

    if i >= 0 && i < rows && (j+1) >= 0 && (j+1) < columns {
        p.push((i as usize, j as usize + 1));
    }

    if (i+1) >= 0 && (i+1) < rows && j >= 0 && j < columns {
        p.push((i as usize + 1, j as usize));
    }

    p
}

type Map = Vec<Vec<(usize, Distance)>>;
type Point = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Distance {
    Value(usize),
    Out(usize),
    Infinite,
}

impl Distance {
    fn is_out(&self) -> bool {
        match self {
            Self::Out(_) => true,
            _ => false,
        }
    }

    fn is_value(&self) -> bool {
        match self {
            Self::Value(_) => true,
            _ => false,
        }
    }

    fn is_infinite(&self) -> bool {
        match self {
            Self::Infinite => true,
            _ => false,
        }
    }

    fn to_out(&mut self) {
        match self {
            Self::Value(x) => *self = Self::Out(*x),
            _ => (),
        }
    }

    fn inner(&self) -> Option<usize> {
        match self {
            Self::Infinite => None,
            Self::Value(x) => Some(*x),
            Self::Out(x) => Some(*x),
        }
    }
}

use std::cmp::Ordering;
impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Infinite, Self::Infinite) => Some(Ordering::Equal),
            (Self::Infinite, _) => Some(Ordering::Greater),
            (_, Self::Infinite) => Some(Ordering::Less),
            (Self::Value(x), Self::Value(y)) => Some(x.cmp(y)),
            (Self::Out(x), Self::Out(y)) => Some(x.cmp(y)),
            (Self::Value(x), Self::Out(y)) => Some(x.cmp(y)),
            (Self::Out(x), Self::Value(y)) => Some(x.cmp(y)),
        }
    }
}