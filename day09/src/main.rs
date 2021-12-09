use utils::{files, parse_field};

type Point = (usize, usize);
type Map = Vec<Vec<Option<u32>>>;
type Values = Vec<Vec<u32>>;

fn main() {
    let filename = "src/input.txt";
    let values = files::read_in_matrix_as::<u32>(filename);

    println!("----- PART 1 -----");
    let mut answer = 0;
    for i in 0..values.len() {
        for j in 0..values[0].len() {
            if is_low_point(&values, i, j) {
                // println!("({},{}) -> {}", i, j, values[i][j]);
                answer += values[i][j] + 1;
            }
        }
    }

    println!("Part 1 Answer: {}", answer);

    println!("\n\n----- PART 2 -----");
    // Set all as Some(v) or None - None represent 9s (i.e. walls)
    let mut map: Map = values.iter()
        .map(
            |r|
            r.iter()
                .map(|v| if *v == 9 { None } else { Some(*v) })
                .collect())
        .collect();

    // for row in map.iter() {
    //     println!("{:?}", row);
    // }
    let mut basins = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j].is_some() {
                let size = find_basin_size(&mut map, (i, j));
                // println!("Found basin at ({:02}, {:02}) -> SIZE: {}",i, j, size);
                basins.push(size);
            }
        }
    }
    basins.sort();
    basins.reverse();

    println!("\nLargest basins are: {}, {}, {}", basins[0], basins[1], basins[2]);
    let answer = basins[0] * basins[1] * basins[2];
    println!("Part 2 Answer: {}", answer);

}

fn find_basin_size(map: &mut Map, start_point: Point) -> u32 {
    let (i, j) = start_point;
    // Already checked or not part of a basin
    if map[i][j].is_none() {
        return 0;
    } else {
        // Set to NONE
        map[i][j] = None;
    }
    // println!("FINDING BASIN FOR ({}, {})", i, j);

    let surrounding_in_basin = |map: &Map, (i, j): Point| -> Option<Vec<Point>> {
        let mut points = Vec::new();
        if i != 0 {
            if map[i-1][j].is_some() {
                points.push((i-1, j));
            }
        }

        if j != 0 {
            if map[i][j-1].is_some() {
                points.push((i, j-1));
            }
        }

        if let Some(v) = map[i].get(j+1) {
            if v.is_some() {
                points.push((i, j+1));
            }
        }

        if let Some(r) = map.get(i+1) {
            if r[j].is_some() {
                points.push((i+1, j));
            }
        }

        if points.len() == 0 {
            None
        } else {
            Some(points)
        }
    };

    let mut size = 1;
    let mut to_check = vec!(start_point);
    while let Some(point) = to_check.pop() {
        // print!("Point ({}, {}) in basin ", point.0, point.1);
        if let Some(points) = surrounding_in_basin(map, point) {
            for p in points {
                // print!("({}, {})", p.0, p.1);
                size += 1;
                map[p.0][p.1] = None;  // Prevent double counting and infinite loops
                to_check.push(p);
            }
        }
        // println!("");
    }

    size
}

fn is_low_point(values: &Vec<Vec<u32>>, i: usize, j:usize) -> bool {
    // Get surrounding points
    let mut result = true;
    if i != 0 {
        if let Some(r) = values.get(i-1) {
            if let Some(v) = r.get(j) {
                result = result && (*v > values[i][j]);
            }
        }
    }

    if j != 0 {
        if let Some(r) = values.get(i) {
            if let Some(v) = r.get(j-1) {
                result = result && (*v > values[i][j]);
            }
        }
    }

    if let Some(r) = values.get(i) {
        if let Some(v) = r.get(j+1) {
            result = result && (*v > values[i][j]);
        }
    }

    if let Some(r) = values.get(i+1) {
        if let Some(v) = r.get(j) {
            result = result && (*v > values[i][j]);
        }
    }

    result
}
