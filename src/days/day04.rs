pub fn part1(input: &str) -> String {
    let diagram = input
        .lines()
        .map(|line| line.trim().as_bytes())
        .collect::<Vec<_>>();

    let mut reachable_rolls = 0;

    for (y, &row) in diagram.iter().enumerate() {
        for (x, &position) in row.iter().enumerate() {
            if position == b'.' {
                continue;
            }

            let mut roll_counter = 0;

            for y_offset in -1..=1 {
                for x_offset in -1..=1 {
                    if y_offset == 0 && x_offset == 0 {
                        continue;
                    }

                    let y = y.wrapping_add_signed(y_offset);
                    let x = x.wrapping_add_signed(x_offset);

                    if diagram
                        .get(y)
                        .and_then(|&row| row.get(x))
                        .is_some_and(|&position| position == b'@')
                    {
                        roll_counter += 1;
                    }
                }
            }

            if roll_counter < 4 {
                reachable_rolls += 1;
            }
        }
    }

    reachable_rolls.to_string()
}

pub fn part2(_input: &str) -> String {
    "Solve part2".to_owned()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn day04_part1() {
        let roll_diagram = indoc! {"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "};
        assert_eq!(part1(roll_diagram), "13");
    }

    #[test]
    fn day04_part2() {
        assert_eq!(part2(""), "Solve part2");
    }
}
