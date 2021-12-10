use utils::{files, parse_field_unwrap};

type Fields = (String,);

fn open_to_close(c: char) -> char {
    match c {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => 'X',
    }
}

fn score_char_p1(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score_char_p2(c: char) -> u128 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}


fn main() {
    let filename = "input";
    let input = files::read_in_matrix(filename);

    println!("----- PART 1 -----");
    // Closure for filter
    let filter = |f: &Fields| -> bool {
        // DO SOMETHING
        true
    };

    // Closure for fold
    let fold = |acc: (u32, Vec<char>), c: &char| -> (u32, Vec<char>) {
        // DO SOMETHING
        let mut acc = acc.clone();
        if "({[<".contains(*c) {
            acc.1.push(*c);
        } else if ")}]>".contains(*c) {
            if let Some(open) = acc.1.last() {
                if open_to_close(*open) != *c {
                    println!("Found illegal {}, expected {}", c, open_to_close(*open));
                    if acc.0 == 0 {
                        acc.0 = score_char_p1(*c);
                    }
                }
                acc.1.pop();
            }
        } else {
            println!("char {} not recognized", c);
        }
        acc
    };

    let output: Vec<(u32, Vec<char>)> = input.iter()
        .map(|chars| chars.iter().fold((0, Vec::new()), fold))
        .collect();

    let answer = output.iter().fold(0, |acc, (score, _)| acc + score);
    println!("Part 1 Answer: {:?}", answer);

    println!("\n\n----- PART 2 -----");
    let mut scores: Vec<u128> = output.iter()
        .filter(|l| l.0 == 0)
        .map(|(_, chars)| chars.iter().rev().fold(0 as u128, |acc, c| acc * 5 + score_char_p2(*c)))
        .collect();
        // .fold(0, |acc, score| acc + score);

    scores.sort();
    let answer = scores[scores.len()/2];

    // for i in answer {
    //     println!("{:?}", i);
    // }
    println!("Part 2 Answer: {}", answer);
}
