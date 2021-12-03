#[derive(Debug)]
pub enum Direction {
    Forward,
    Up,
    Down,
}

impl std::str::FromStr for Direction {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(Self::Err::ParseError),
        }
    }
}

pub enum ParseInstructionError {
    ParseError,
}

#[derive(Debug)]
pub struct Position1 {
    pub horizontal: i32,
    pub depth: i32,
}

impl Position1 {
    pub fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }

    pub fn execute_instruction(&mut self, dir: Direction, amount: i32) {
        match dir {
            Direction::Forward => self.horizontal += amount,
            Direction::Down => self.depth += amount,
            Direction::Up => self.depth -= amount,
        };
    }

}

#[derive(Debug)]
pub struct Position2 {
    pub horizontal: i32,
    pub depth: i32,
    pub aim: i32,
}

impl Position2 {
    pub fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn execute_instruction(&mut self, dir: Direction, amount: i32) {
        match dir {
            Direction::Forward => {
                self.horizontal += amount;
                self.depth += amount * self.aim;
            }
            Direction::Down => self.aim += amount,
            Direction::Up => self.aim -= amount,
        };
    }
}