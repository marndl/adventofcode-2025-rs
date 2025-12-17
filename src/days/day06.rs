pub fn part1(input: &str) -> String {
    let mut input = input.lines().rev();

    let operations: Vec<fn(u64, u64) -> u64> = input
        .next()
        .expect("at least two lines of input")
        .split_ascii_whitespace()
        .map(|operation| match operation {
            "*" => |x, y| x * y,
            "+" => |x, y| x + y,
            operation => panic!("unkown operation '{operation}'"),
        })
        .collect();

    let mut numbers: Vec<Vec<u64>> = input
        .map(|row| {
            row.split_ascii_whitespace()
                .map(|number| number.parse().expect("input numbers to fit into u64"))
                .collect()
        })
        .collect();

    let column_count = numbers[0].len();
    assert!(
        numbers.iter().all(|column| column.len() == column_count),
        "all input lines to have the same amount of columns"
    );
    assert!(
        column_count == operations.len(),
        "all input lines to have the same amount of columns"
    );

    let mut results = numbers.pop().expect("at least two lines of input");

    for row in numbers {
        for (index, value) in row.into_iter().enumerate() {
            results[index] = operations[index](results[index], value);
        }
    }

    results.into_iter().sum::<u64>().to_string()
}

pub fn part2(_input: &str) -> String {
    "Solve part2".to_owned()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn day06_part1() {
        let input = indoc! {"
            123 328 51 64 
            45 64 387 23 
            6 98 215 314
            * + * +
        "};
        assert_eq!(part1(input), "4277556");
    }

    #[test]
    fn day06_part2() {
        assert_eq!(part2(""), "Solve part2");
    }
}
