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

    fn highest_jolt_with_12_batteries(&self) -> u64 {
        let mut result = 0;
        let mut start = 0;

        for rest in (0..=11).rev() {
            let (index, digit) = self.batteries[start..(self.batteries.len() - rest)]
                .iter()
                .copied()
                .enumerate()
                .max_by(|(index_x, x), (index_y, y)| x.cmp(y).then(index_x.cmp(index_y).reverse()))
                .expect("at least one digit to remain in current search space");
            start += index + 1;
            result += u64::from(digit) * 10u64.pow(u32::try_from(rest).unwrap());
        }

        result
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

pub fn part2(input: &str) -> String {
    parse_input(input)
        .iter()
        .map(Bank::highest_jolt_with_12_batteries)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn day03_part1() {
        assert_eq!(part1(INPUT), "357");
    }

    #[test]
    fn day03_part2() {
        assert_eq!(part2(INPUT), "3121910778619");
    }
}
