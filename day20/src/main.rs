use utils::{files, parse_field};

fn main() {
    let filename = "input";
    let filename2 = "key";
    let mut image = files::read_in_matrix(filename);
    let key = files::read_in_line(filename2).chars().collect::<Vec<char>>();

    println!("----- PART 1 -----");
    // print_image(&image);
    for n in 0..2 {
        image = process_image(image, &key, n);
        // print_image(&image);
    }

    let count = image.iter().flatten().fold(0, |acc, c| acc + if *c == '1' { 1 } else { 0 });

    println!("Part 1 Answer: {}", count);

    println!("\n\n----- PART 2 -----");
    // Already done 2 iterations
    for n in 2..50 {
        image = process_image(image, &key, n);
        // print_image(&image);
    }

    let count = image.iter().flatten().fold(0, |acc, c| acc + if *c == '1' { 1 } else { 0 });

    println!("Part 2 Answer: {}", count);

}

fn grow_image(image: &mut Vec<Vec<char>>, c: char, n: usize) {
    let rows = image.len();
    let cols = image[0].len();
    for _ in 0..2 {
        image.insert(0, vec!(c; cols));
        image.push(vec!(c; cols));
    }

    for row in image.iter_mut() {
        for _ in 0..n {
            row.insert(0, c);
            row.push(c);
        }
    }
}

fn print_image(image: &Vec<Vec<char>>) {
    for row in image {
        for c in row {
            match c {
                '0' => print!("."),
                '1' => print!("#"),
                _ => (),
            }
        }
        println!();
    }
    println!("\n\n");
}

fn process_image(mut image: Vec<Vec<char>>, key: &Vec<char>, step: isize) -> Vec<Vec<char>> {
    let rows = image.len();
    let cols = image[0].len();

    let c = if step == 0 {
        '0'
    } else if key[0] == '1' && step%2 == 0 {
        *key.last().unwrap()
    } else {
        key[0]
    };

    grow_image(&mut image, c, 2);

    let mut new_image = vec!(vec!('0'; cols+2); rows+2);
    for i in 0..rows+2 {
        for j in 0..cols+2 {
            let index_str = format!("{}{}{}{}{}{}{}{}{}",
                image[i+0][j], image[i+0][j+1], image[i+0][j+2],
                image[i+1][j], image[i+1][j+1], image[i+1][j+2],
                image[i+2][j], image[i+2][j+1], image[i+2][j+2]);
            let index = usize::from_str_radix(&index_str, 2).unwrap();
            // println!("({},{}) -> {} -> {} -> {}", i, j, index_str, index, key[index]);
            new_image[i][j] = key[index];
        }
    }

    new_image
}