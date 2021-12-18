
// i.e. (value, nested level)
pub type SnailDigit = (u32, usize);

#[derive(Debug, Clone)]
pub struct SnailNumber {
    number: Vec<SnailDigit>,
}

impl SnailNumber {
    pub fn new(number: Vec<SnailDigit>) -> Self {
        SnailNumber {
            number
        }
    }

    pub fn print(&self) {
        let mut last_level = 0;
        print!("[");
        for (d, level) in self.number.iter() {
            while last_level < *level {
                print!("[");
                last_level += 1;
            }
            while last_level > *level {
                print!("]");
                last_level -= 1;
            }
            print!("{},", d);
        }
        println!();
    }

    pub fn reduce(&mut self) {
        loop {
            if self.explode() {
                // println!("AFTER EXPLODE: {:?}", self);
                continue;
            }
            if !self.split() {
                break;
            }
            // println!("AFTER SPLIT: {:?}", self);
        }
    }

    fn explode(&mut self) -> bool {
        // Find first explodable pair
        let mut last_level = 0;
        let mut index = None;
        for (i, d) in self.number.iter().enumerate() {
            if last_level == d.1 && last_level >= 4 {
                // Explode candidate found
                index = Some((i-1, i));
                break;
            }
            last_level = d.1;
        }

        if let Some((l, r)) = index {
            // Do explosion
            // println!("EXPLODING: [{:?}, {:?}] @ {}", self.number[l], self.number[r], last_level);
            if l > 0 {
                self.number[l-1].0 += self.number[l].0;
            }
            if r < self.number.len() - 1 {
                self.number[r+1].0 += self.number[r].0;
            }
            // Destroy in place
            self.number[l] = (0, last_level - 1);
            self.number.remove(r);
            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        // Find first splittable number
        let mut index = None;
        for (i, d) in self.number.iter().enumerate() {
            if d.0 > 9 {
                // Explode candidate found
                index = Some(i);
                break;
            }
        }

        if let Some(i) = index {
            // Do split
            let l = self.number[i].0 / 2;
            let r = (self.number[i].0 + 1) / 2;
            let level = self.number[i].1 + 1;
            self.number[i] = (l, level);
            self.number.insert(i+1, (r, level));
            true
        } else {
            false
        }
    }

    pub fn magnitude(&self) -> u32 {
        let mut copy = self.clone();
        while copy.number.len() > 1 {
            let mut last_level = 0;
            let mut i_l = 0;
            let mut i_r = 0;
            for (i, d) in copy.number.iter().enumerate() {
                if last_level == d.1 && i != 0 {
                    // println!("Found pair at {} - ... <=> {:?}", i, d);
                    // Found pair to 'reduce' for magnitude (3l + 2r)
                    i_l = i - 1;
                    i_r = i;
                    break;
                }
                last_level = d.1;
            }
            if last_level == 0 {
                copy.number[i_l] = (copy.number[i_l].0 * 3 + copy.number[i_r].0 * 2, last_level);
                copy.number.remove(i_r);
            } else {
                copy.number[i_l] = (copy.number[i_l].0 * 3 + copy.number[i_r].0 * 2, last_level - 1);
                copy.number.remove(i_r);
            }
        }
        copy.number[0].0
    }
}

impl std::ops::AddAssign for SnailNumber {

    fn add_assign(&mut self, other: Self) {
        for (v, l) in self.number.iter_mut() {
            *l += 1;
        }
        for (v, l) in other.number.iter() {
            self.number.push((*v, l+1));
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    ParseSnailNumberError
}

impl std::str::FromStr for SnailNumber {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nest_level: usize = 0;
        let mut number = Vec::new();
        for c in s.chars() {
            match c {
                '[' => nest_level += 1,
                ']' => nest_level -= 1,
                ',' => (),
                c => {
                    if let Some(num) = c.to_digit(10) {
                        number.push((num, nest_level - 1));
                    } else {
                        return Err(Self::Err::ParseSnailNumberError);
                    }
                }
            }
        }
        Ok(SnailNumber::new(number))
    }
}


pub fn parse_snail_number(s: &str) -> Vec<SnailDigit> {
    let mut nest_level: usize = 0;
    let mut number = Vec::new();
    for c in s.chars() {
        match c {
            '[' => nest_level += 1,
            ']' => nest_level -= 1,
            ',' => (),
            c => {
                let num = c.to_digit(10).unwrap();
                number.push((num, nest_level - 1));
            }
        }
    }
    number
}