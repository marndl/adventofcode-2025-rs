struct Bank {
    batteries: Vec<u8>,
}

impl Bank {
    fn highest_jolt_with_2_batteries(&self) -> u8 {
        let (battery_index, highest_jolt) = self
            .batteries
            .iter()
            .copied()
            .enumerate()
            .rev()
            .skip(1)
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .unwrap();

        highest_jolt * 10 + self.batteries.iter().skip(battery_index + 1).max().unwrap()
    }
}

fn parse_input(input: &str) -> Vec<Bank> {
    input
        .lines()
        .map(|line| Bank {
            batteries: line
                .chars()
                .map(|char| {
                    char.to_digit(10)
                        .expect("battery input to only be digits")
                        .try_into()
                        .unwrap()
                })
                .collect(),
        })
        .collect()
}

pub fn part1(input: &str) -> String {
    parse_input(input)
        .iter()
        .map(|bank| u64::from(bank.highest_jolt_with_2_batteries()))
        .sum::<u64>()
        .to_string()
}

pub fn part2(_input: &str) -> String {
    "Solve part2".to_owned()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day03_part1() {
        let banks = indoc::indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "};
        assert_eq!(part1(banks), "357");
    }

    #[test]
    fn day03_part2() {
        assert_eq!(part2(""), "Solve part2");
    }
}
