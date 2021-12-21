use utils::{files, parse_field};

fn main() {
    let filename = "src/input.txt";
    // let matrix = files::read_in_matrix_as::<u32>(filename);
    let mut s1 = (4, 0);
    let mut s2 = (1, 0);
    println!("----- PART 1 -----");
    let mut die = 0;
    let mut count = 0;

    loop {
        let mut roll1 = 0;
        for _ in 0..3 {
            die += 1;
            if die > 100 {
                die = 1;
            }
            roll1 += die;
        }
        count += 3;
        s1.0 += roll1;
        while s1.0 > 10 {
            s1.0 -= 10;
        }
        s1.1 += s1.0;

        // println!("P1 Rolled {} -> Moved to {} -> Score {}", roll1, s1.0, s1.1);

        if s1.1 >= 1000 {
            println!("{}", s2.1 * count);
            break;
        }

        let mut roll2 = 0;
        for _ in 0..3 {
            die += 1;
            if die > 100 {
                die = 1;
            }
            roll2 += die;
        }
        count += 3;
        s2.0 += roll2;
        while s2.0 > 10 {
            s2.0 -= 10;
        }
        s2.1 += s2.0;

        // println!("P2 Rolled {} -> Moved to {} -> Score {}", roll2, s2.0, s2.1);

        if s2.1 >= 1000 {
            println!("{}", s1.1 * count);
            break;
        }
    }

    println!("Part 1 Answer: {}", 0);

    println!("\n\n----- PART 2 -----");

    // DIRAC DICE QUANTUM SPLIT!!!
    // Possible rolls:
    // 1 1 1        -> 3
    // 2 1 1 x 3    -> 4 x 3
    // 2 2 1 x 3    -> 5 x 3
    // 2 2 2        -> 6
    // 3 1 1 x 3    -> 5 x 3
    // 3 3 1 x 3    -> 7 x 3
    // 3 3 3        -> 9
    // 3 2 2 x 3    -> 7 x 3
    // 3 3 2 x 3    -> 8 x 3
    // 1 2 3 x 6    -> 6 x 6
    // TOTAL OF 27 possible outcomes
    // Distribution:
    // 3
    // 4 4 4
    // 5 5 5 5 5 5
    // 6 6 6 6 6 6 6
    // 7 7 7 7 7 7
    // 8 8 8
    // 9
    let rolls = [(3, 1),
                 (4, 3),
                 (5, 6),
                 (6, 7),
                 (7, 6),
                 (8, 3),
                 (9, 1)];

    // [(p1), (p2)]
    let mut games = vec!((Player::new(4), Player::new(1), 1));
    let mut p1_wins: u128 = 0;
    let mut p2_wins: u128 = 0;
    loop {
        let mut next_games_p1 = Vec::new();
        for game in games.iter() {
            let p1 = game.0;
            let p2 = game.1;
            let n = game.2;
            for roll in rolls {
                let p1_new = p1.take_turn(roll.0);
                if p1_new.score >= 21 {
                    // println!("P1 won {}", n * roll.1);
                    p1_wins += n * roll.1;
                } else {
                    next_games_p1.push((p1_new, p2, n * roll.1));
                }
            }
        }

        if next_games_p1.len() == 0 {
            break;
        }

        let mut next_games_p2 = Vec::new();
        for game in next_games_p1.iter() {
            let p1 = game.0;
            let p2 = game.1;
            let n = game.2;
            for roll in rolls {
                let p2_new = p2.take_turn(roll.0);
                if p2_new.score >= 21 {
                    // println!("P2 won {}", n * roll.1);
                    p2_wins += n * roll.1;
                } else {
                    next_games_p2.push((p1, p2_new, n * roll.1));
                }
            }
        }

        if next_games_p2.len() == 0 {
            break;
        }

        games = next_games_p2;
    }

    println!("Part 2 Answer: P1-{} P2-{}", p1_wins, p2_wins);

}

#[derive(Debug, Clone, Copy)]
struct Player {
    pub position: usize,
    pub score: usize,
}

impl Player {
    fn new(start: usize) -> Self {
        Self {
            position: start,
            score: 0
        }
    }

    /// Take turn returning the current score
    pub fn take_turn(mut self, roll: usize) -> Self {
        self.position += roll;
        if self.position > 10 {
            self.position -= 10;
        }

        self.score += self.position;
        self
    }
}
