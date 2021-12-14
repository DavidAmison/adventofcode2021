use utils::{files, parse_field_unwrap};

type Fields = (String,);

fn main() {
    let filename1 = "points";
    let filename2 = "instr";
    let input1 = files::read_in_lines(filename1);
    let input2 = files::read_in_lines(filename2);

    let mut points: Vec<(u32, u32)> = input1.iter().map(|l| parse_field_unwrap!(l => u32 ,"," | u32, "")).collect();
    points.sort();
    let instructions: Vec<(char, u32)> = input2.iter().map(|l| parse_field_unwrap!(l => String, "fold along " | char, "=" | u32, "")).map(|t| (t.1, t.2)).collect();

    // find the size of the paper
    let fold = |acc: (u32, u32), (x, y): &(u32, u32)| -> (u32, u32) {
        (
            if x > &acc.0 { *x } else { acc.0 },
            if y > &acc.0 { *y } else { acc.1 },
        )
    };
    let paper_size = points.iter().fold((0,0), fold);

    println!("----- PART 1 -----");

    // Perform first fold
    let mut folds = instructions.iter();
    let fold = folds.next().unwrap();
    fold_paper(&mut points, fold);

    println!("Part 1 Answer {}", points.len());

    println!("\n----- PART 2 -----");
    while let Some(fold) = folds.next() {
        fold_paper(&mut points, fold);
    }

    let mut current_row = 0;
    let mut current_column = 0;
    let mut flipped: Vec<(u32, u32)> = points.iter().map(|p| (p.1, p.0)).collect();
    flipped.sort();
    for p in flipped.iter() {
        while current_row < p.0 {
            println!("");
            current_row += 1;
            current_column = 0;
        }
        while current_column < p.1 {
            print!(" ");
            current_column += 1;
        }
        print!("#");
        current_column += 1;
    }
    println!();
    println!("Part 2 Answer: {}", points.len());

}

fn fold_paper(points: &mut Vec<(u32, u32)>, fold: &(char, u32)) {
    for p in points.iter_mut() {
        // Which axis
        match fold.0 {
            'x' => {
                if p.0 > fold.1 {
                    p.0 = 2*fold.1 - p.0;
                }
            }
            'y' => {
                if p.1 > fold.1 {
                    p.1 = 2*fold.1 - p.1;
                }
            }
            _ => (),
        }
    }
    points.sort();
    points.dedup();
}
