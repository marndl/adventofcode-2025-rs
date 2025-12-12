fn parse_diagram(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.trim().as_bytes().to_vec())
        .collect()
}

fn find_reachable_rolls(diagram: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut reachable_rolls = vec![];

    for (y, row) in diagram.iter().enumerate() {
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
                        .and_then(|row| row.get(x))
                        .is_some_and(|&position| position == b'@')
                    {
                        roll_counter += 1;
                    }
                }
            }

            if roll_counter < 4 {
                reachable_rolls.push((y, x));
            }
        }
    }

    reachable_rolls
}

pub fn part1(input: &str) -> String {
    let diagram = parse_diagram(input);
    find_reachable_rolls(&diagram).len().to_string()
}

pub fn part2(input: &str) -> String {
    let mut diagram = parse_diagram(input);
    let mut total_removed_rolls = 0;

    loop {
        let reachable_rolls = find_reachable_rolls(&diagram);

        if reachable_rolls.is_empty() {
            break;
        }

        total_removed_rolls += reachable_rolls.len();

        reachable_rolls
            .into_iter()
            .for_each(|(y, x)| diagram[y][x] = b'.');
    }

    total_removed_rolls.to_string()
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
        assert_eq!(part2(roll_diagram), "43");
    }
}
