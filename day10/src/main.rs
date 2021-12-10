use utils::{files};

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
    // Search for syntax errors storing the syntax error score
    // and the unclosed  blocks
    let check_syntax = |acc: (u32, Vec<char>), c: &char| -> (u32, Vec<char>) {
        let mut acc = acc.clone();
        if "({[<".contains(*c) {
            acc.1.push(*c);
        } else if ")}]>".contains(*c) {
            if let Some(open) = acc.1.last() {
                if open_to_close(*open) != *c {
                    // Only score first illegal character
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

    let syntax_checked: Vec<(u32, Vec<char>)> = input.iter()
        .map(
            |chars| chars.iter().fold((0, Vec::new()), check_syntax)  // Check syntax
        ).collect();

    let answer = syntax_checked.iter()
        .fold(0, |acc, (score, _)| acc + score);  // Sum all syntax error scores
    println!("Part 1 Answer: {:?}", answer);

    println!("\n\n----- PART 2 -----");
    // Calculate score to autocomplete the line:
    //
    // The score is determined by considering the completion string
    // character-by-character. Start with a total score of 0. Then,
    // for each character, multiply the total score by 5 and then
    // increase the total score by the point value given for the
    // character;
    let autocomplete_score = |(_, chars): &(_, Vec<char>)| {
        chars.iter()
            .rev()
            .fold(0, |acc, c| acc * 5 + score_char_p2(*c))
    };

    let mut scores: Vec<u128> = syntax_checked.iter()
        .filter(|l| l.0 == 0)           // Only lines with 0 sytax error score
        .map(autocomplete_score)        // Calculate the score to autocomplete the line
        .collect();

    // Get the 'middle' score
    scores.sort();
    let answer = scores[scores.len()/2];

    println!("Part 2 Answer: {}", answer);
}
