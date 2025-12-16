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

pub fn part2(input: &str) -> String {
    let (fresh_id_ranges, _) = input
        .split_once("\n\n")
        .expect("fresh ingredient ranges and ingredient ids to be separated by an empty line");

    let mut fresh_id_ranges = fresh_id_ranges
        .lines()
        .map(|line| {
            let (start, end) = line
                .split_once('-')
                .expect("id range bounds to be separated by an dash");

            (
                start
                    .parse::<u64>()
                    .expect("id range bounds to fit within an u64"),
                end.parse::<u64>()
                    .expect("id range bounds to fit within an u64"),
            )
        })
        .collect::<Vec<_>>();

    fresh_id_ranges.sort_unstable_by(|(start_a, end_a), (start_b, end_b)| {
        start_a.cmp(start_b).then(end_a.cmp(end_b))
    });

    let mut combined_fresh_id_ranges = vec![];
    let mut latest_id_range = fresh_id_ranges[0];

    for id_range in fresh_id_ranges.into_iter().skip(1) {
        if id_range.0 <= latest_id_range.1 {
            if id_range.1 > latest_id_range.1 {
                latest_id_range.1 = id_range.1;
            }
            continue;
        }

        combined_fresh_id_ranges.push(latest_id_range);
        latest_id_range = id_range;
    }

    combined_fresh_id_ranges.push(latest_id_range);

    combined_fresh_id_ranges
        .iter()
        .map(|range| range.1 - (range.0 - 1))
        .sum::<u64>()
        .to_string()
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
        assert_eq!(part2(database), "14");
    }
}
