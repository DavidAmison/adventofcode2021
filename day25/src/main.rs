use utils::{files, parse_field};


type Map = Vec<Vec<Point>>;

fn main() {
    let filename = "input";
    let matrix = files::read_in_matrix(filename);

    let map = |c: &char| -> Point {
        match c {
            '>' => Point::SCEast,
            'v' => Point::SCSouth,
            _ => Point::Empty,
        }
    };

    let mut map: Map = matrix.iter().map(|row| row.iter().map(map).collect()).collect();

    println!("----- PART 1 -----");

    let mut step = 1;
    while move_east_herd(&mut map) + move_south_herd(&mut map) != 0 {
        step += 1;
        // print_map(&map);
    }

    println!("Part 1 Answer: {}", step);

    println!("\n\n----- PART 2 -----");

    println!("Part 2 Answer: {}", 0);

}

#[derive(Debug, Clone)]
enum Point {
    Empty,
    SCEast,
    SCSouth,
}

fn move_east_herd(map: &mut Map) -> usize {
    let mut count = 0;
    let mut to_move = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            match map[i][j] {
                Point::SCEast => {
                    let target_i = i;
                    let mut target_j = j + 1;
                    if target_j >= map[0].len() {
                        target_j = 0;
                    }
                    match map[target_i][target_j] {
                        Point::Empty => {
                            to_move.push((i, j, target_j));
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    }
    while let Some(p) = to_move.pop() {
        // println!("Moving ({},{})", p.0, p.1);
        map[p.0][p.1] = Point::Empty;
        map[p.0][p.2] = Point::SCEast;
        count += 1;
    }
    count
}

fn move_south_herd(map: &mut Map) -> usize {
    let mut count = 0;
    let mut to_move = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            match map[i][j] {
                Point::SCSouth => {
                    let mut target_i = i + 1;
                    let target_j = j;
                    if target_i >= map.len() {
                        target_i = 0;
                    }
                    match map[target_i][target_j] {
                        Point::Empty => {
                            to_move.push((i, j, target_i));
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    }
    while let Some(p) = to_move.pop() {
        // println!("Moving ({},{})", p.0, p.1);
        map[p.0][p.1] = Point::Empty;
        map[p.2][p.1] = Point::SCSouth;
        count += 1;
    }
    count
}

fn print_map(map: &Map) {
    for row in map.iter() {
        for p in row.iter() {
            match p {
                Point::Empty => print!("."),
                Point::SCSouth => print!("v"),
                Point::SCEast => print!(">"),
            }
        }
        println!();
    }
    println!();
    println!();
}