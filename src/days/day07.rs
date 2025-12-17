pub fn part1(input: &str) -> String {
    let mut diagram = input
        .lines()
        .map(|line| line.trim().as_bytes().to_owned())
        .collect::<Vec<_>>();

    let mut total_splits = 0;

    let initial_beam = diagram[0]
        .iter()
        .position(|&location| location == b'S')
        .expect("first line to start the beam");
    diagram[1][initial_beam] = b'|';

    for row in 2..diagram.len() {
        for position in 0..diagram[row].len() {
            if diagram[row - 1][position] != b'|' {
                continue;
            }

            if diagram[row][position] == b'.' {
                diagram[row][position] = b'|';
            }

            if diagram[row][position] == b'^' {
                let mut was_split = false;

                for position in [position.wrapping_sub(1), position + 1] {
                    if let Some(location) = diagram[row].get_mut(position)
                        && *location == b'.'
                    {
                        *location = b'|';
                        was_split = true;
                    }
                }

                if was_split {
                    total_splits += 1;
                }
            }
        }
    }

    total_splits.to_string()
}

pub fn part2(_input: &str) -> String {
    "Solve part2".to_owned()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn day07_part1() {
        let input = indoc! {"
            .......S.......
            ...............
            .......^.......
            ...............
            ......^.^......
            ...............
            .....^.^.^.....
            ...............
            ....^.^...^....
            ...............
            ...^.^...^.^...
            ...............
            ..^...^.....^..
            ...............
            .^.^.^.^.^...^.
            ...............
        "};
        assert_eq!(part1(input), "21");
    }

    #[test]
    fn day07_part2() {
        assert_eq!(part2(""), "Solve part2");
    }
}
