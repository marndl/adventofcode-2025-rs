use std::str::FromStr;

#[derive(Debug)]
struct IdRange {
    start: u64,
    end: u64,
}

impl FromStr for IdRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once('-')
            .ok_or_else(|| "Expect range bounds to be separated by a dash".to_owned())?;
        Ok(Self {
            start: start
                .parse()
                .map_err(|err| format!("Failed to parse start of id range: {err}"))?,
            end: end
                .parse()
                .map_err(|err| format!("Failed to parse end of id range: {err}"))?,
        })
    }
}

impl IdRange {
    fn sum_invalid_ids(&self) -> u64 {
        let mut invalid_ids = 0;

        for id in self.start..=self.end {
            if !(id.ilog10() + 1).is_multiple_of(2) {
                continue;
            }

            let id_str = id.to_string();
            let id_bytes = id_str.as_bytes();
            let halfpoint = id_bytes.len() / 2;

            let mut valid = true;

            for i in 0..halfpoint {
                if id_bytes[i] != id_bytes[i + halfpoint] {
                    valid = false;
                    break;
                }
            }

            if valid {
                invalid_ids += id;
            }
        }

        invalid_ids
    }
}

pub fn part1(input: &str) -> String {
    input
        .split(',')
        .map(|raw_id_range| {
            raw_id_range
                .trim()
                .parse::<IdRange>()
                .expect("puzzle input to be valid")
                .sum_invalid_ids()
        })
        .sum::<u64>()
        .to_string()
}

pub fn part2(_input: &str) -> String {
    "Solve part2".to_owned()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn day02_part1() {
        let ids = indoc! {"
            11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
            1698522-1698528,446443-446449,38593856-38593862,565653-565659,
            824824821-824824827,2121212118-2121212124
        "};
        assert_eq!("1227775554", part1(ids));
    }

    #[test]
    fn day02_part2() {
        assert_eq!(part2(""), "Solve part2");
    }
}
