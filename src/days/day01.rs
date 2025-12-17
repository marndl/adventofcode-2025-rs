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

        Ok(Self {
            direction: match direction {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => return Err("Expected L or R at the start of an instruction".to_owned()),
            },
            distance: distance
                .parse()
                .map_err(|err| format!("Expect dial value after direction: {err}"))?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Dial {
    value: u32,
    zero_counter: u32,
}

impl Dial {
    fn new() -> Self {
        Self::default()
    }

    fn is_zero(self) -> bool {
        self.value == 0
    }

    fn zero_counter(self) -> u32 {
        self.zero_counter
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            value: 50,
            zero_counter: 0,
        }
    }
}

impl AddAssign<u32> for Dial {
    fn add_assign(&mut self, rhs: u32) {
        self.zero_counter = rhs / 100;
        let mut new_value = self.value + rhs % 100;

        if self.value != 0 && (new_value > 99 || new_value == 0) {
            self.zero_counter += 1;
        }

        new_value = new_value.rem_euclid(100);
        self.value = new_value;
    }
}

impl SubAssign<u32> for Dial {
    fn sub_assign(&mut self, rhs: u32) {
        self.zero_counter = rhs / 100;
        let mut new_value = i32::try_from(self.value).unwrap() - i32::try_from(rhs % 100).unwrap();

        if self.value != 0 && new_value <= 0 {
            self.zero_counter += 1;
        }

        new_value = new_value.rem_euclid(100);
        self.value = u32::try_from(new_value).unwrap();
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
        password += dial.zero_counter();
    }

    password.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
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

    #[test]
    fn day01_part1() {
        assert_eq!(part1(INPUT), "3");
    }

    #[test]
    fn day01_part2() {
        assert_eq!(part2(INPUT), "6");
    }
}
