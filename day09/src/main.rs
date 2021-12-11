use utils::{files};

mod map;
use map::*;

fn main() {
    let filename = "src/input.txt";
    let map = Map{
        elements: files::read_in_matrix_as::<u32>(filename)
    };

    println!("----- PART 1 -----");
    let points = map.find_low_points();
    let answer = points.iter().fold(0, |acc, (i, j)| acc + map.elements[*i][*j] + 1);
    println!("Part 1 Answer: {}", answer);

    println!("\n\n----- PART 2 -----");
    let mut basins = map.find_basins();
    basins.sort();
    basins.reverse();

    // println!("\nLargest basins are: {}, {}, {}", basins[0].0, basins[1].0, basins[2].0);
    for i in 0..3 {
        println!("BASIN {} - SIZE {}", i+1, basins[i].size());
        println!("{:?}", basins[i]);
    }
    let answer = basins[0].size() * basins[1].size() * basins[2].size();
    println!("Part 2 Answer: {}", answer);

}
