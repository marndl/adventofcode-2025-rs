pub fn part1(input: &str) -> String {
    let (fresh_id_ranges, ingredient_ids) = input
        .split_once("\n\n")
        .expect("fresh ingredient ranges and ingredient ids to be separated by an empty line");

    let fresh_id_ranges = fresh_id_ranges
        .lines()
        .map(|line| {
            let (start, end) = line
                .split_once('-')
                .expect("id range bounds to be separated by an dash");

            start
                .parse::<u64>()
                .expect("id range bounds to fit within an u64")
                ..=end
                    .parse::<u64>()
                    .expect("id range bounds to fit within an u64")
        })
        .collect::<Vec<_>>();

    ingredient_ids
        .lines()
        .map(|ingredient_id| ingredient_id.parse::<u64>().unwrap())
        .filter(|ingredient_id| {
            fresh_id_ranges
                .iter()
                .any(|id_range| id_range.contains(ingredient_id))
        })
        .count()
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
    fn day05_part1() {
        let database = indoc! {"
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "};
        assert_eq!(part1(database), "3");
    }

    #[test]
    fn day05_part2() {
        assert_eq!(part2(""), "Solve part2");
    }
}
