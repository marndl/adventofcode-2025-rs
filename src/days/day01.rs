use std::{
    ops::{AddAssign, SubAssign},
    str::FromStr,
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: u32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_at(1);

        Ok(Instruction {
            direction: match direction {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => return Err("Expected L or R at the start of an instruction".to_owned()),
            },
            distance: distance
                .parse()
                .map_err(|err| format!("Expect dial value after direction: {}", err))?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Dial(u32);

impl Dial {
    fn new() -> Self {
        Self::default()
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self(50)
    }
}

impl AddAssign<u32> for Dial {
    fn add_assign(&mut self, rhs: u32) {
        self.0 = (self.0 + rhs).rem_euclid(100);
    }
}

impl SubAssign<u32> for Dial {
    fn sub_assign(&mut self, rhs: u32) {
        self.0 = (i32::try_from(self.0).expect("dial value to be in range 0-99")
            - i32::try_from(rhs).expect("distance value to fit in i32"))
        .rem_euclid(100)
        .try_into()
        .unwrap();
    }
}

impl AddAssign<Instruction> for Dial {
    fn add_assign(&mut self, rhs: Instruction) {
        match rhs.direction {
            Direction::Left => *self -= rhs.distance,
            Direction::Right => *self += rhs.distance,
        }
    }
}

pub fn part1(input: &str) -> String {
    let instructions = input
        .trim()
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>, _>>()
        .expect("puzzle input to be valid");

    let mut password = 0;
    let mut dial = Dial::new();

    for instruction in instructions {
        dial += instruction;
        if dial.is_zero() {
            password += 1;
        }
    }

    password.to_string()
}

pub fn part2(input: &str) -> String {
    "Solve part2".to_owned()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn day01_part1() {
        let instructions = indoc! {"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "};
        assert_eq!(part1(instructions), "3");
    }

    #[test]
    fn day01_part2() {
        assert_eq!(part2(""), "Solve part2");
    }
}
