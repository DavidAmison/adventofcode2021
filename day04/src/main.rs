use utils::{files, parse_field};

#[derive(Debug, Clone)]
enum BoardNum {
    Unmarked(u32),
    Marked(u32),
}

impl BoardNum {
    fn from(x: u32) -> Self {
        Self::Unmarked(x)
    }

    fn mark(&mut self) {
        match self {
            Self::Unmarked(x) => *self = Self::Marked(*x),
            Self::Marked(_) => (),
        };
    }
}

impl std::str::FromStr for BoardNum {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s.parse::<u32>()?;
        Ok(Self::Unmarked(num))
    }
}

fn main() {
    let numbers = files::read_in_sv_as::<u32>("src/numbers.txt", ",");
    let mut boards = files::read_in_matrix_chunks_as::<BoardNum>("src/boards.txt");

    for x in numbers {
        for board in boards.iter_mut() {
            if check_board(board) {
                continue;
            }
            mark_matches(board, x);
            if check_board(board) {
                for row in board.iter() {
                    println!("{:?}", row);
                }
                println!("SCORE: {}", sum_unmarked(board)*x);
            }
        }
    }
}

fn mark_matches(board: &mut Vec<Vec<BoardNum>>, x: u32) {
    for row in board.iter_mut() {
        for num in row.iter_mut() {
            match num {
                BoardNum::Unmarked(n) if *n == x => num.mark(),
                _ => (),
            };
        }
    }
}

fn check_board(board: &Vec<Vec<BoardNum>>) -> bool {
    // Rows
    for row in board.iter() {
        let mut result = true;
        for num in row.iter() {
            result &= match num {
                BoardNum::Marked(_) => true,
                BoardNum::Unmarked(_) => false,
            }
        }
        if result {
            return true;
        }
    }
    // Columns
    let mut results = vec!(true; board.len());
    for row in board.iter() {
        for (i, num) in row.iter().enumerate() {
            results[i] &= match num {
                BoardNum::Marked(_) => true,
                BoardNum::Unmarked(_) => false,
            };
        }
    }
    for (i, result) in results.iter().enumerate() {
        if *result {
            return true
        }
    }
    false
}

fn sum_unmarked(board: &Vec<Vec<BoardNum>>) -> u32 {
    let mut sum = 0;
    for row in board.iter() {
        for num in row.iter() {
            sum += match num {
                BoardNum::Marked(_) => 0,
                BoardNum::Unmarked(x) => *x,
            }
        }
    }
    sum
}
