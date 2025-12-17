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

pub fn part2(input: &str) -> String {
    let mut input = input.lines().rev();
    let operations = input.next().unwrap().as_bytes();
    let numbers = input.rev().map(str::as_bytes).collect::<Vec<_>>();

    let mut total = 0;
    let mut current_operation: fn(u64, u64) -> u64 = |_, _| 0;
    let mut current_result = 0;
    let mut raw_number_buffer = Vec::with_capacity(numbers.len());

    for column in 0..operations.len() {
        raw_number_buffer.clear();

        if operations[column] != b' ' {
            total += current_result;
            current_result = 0;
            current_operation = match operations[column] {
                b'*' => |x, y| x * y,
                b'+' => |x, y| x + y,
                operation => panic!("Unknown operation: {operation}"),
            };
        }

        for row in &numbers {
            raw_number_buffer.push(row[column]);
        }

        let number_str = str::from_utf8(&raw_number_buffer)
            .expect("input to only contain ascii digits and spaces")
            .trim();

        if number_str.is_empty() {
            continue;
        }

        let number = number_str
            .parse::<u64>()
            .expect("buffer to only contain digits");

        if current_result == 0 {
            current_result = number;
        } else {
            current_result = current_operation(current_result, number);
        }
    }

    total += current_result;

    total.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        123 328  51 64 
         45 64  387 23 
          6 98  215 314
        *   +   *   +  
    "};

    #[test]
    fn day06_part1() {
        assert_eq!(part1(INPUT), "4277556");
    }

    #[test]
    fn day06_part2() {
        assert_eq!(part2(INPUT), "3263827");
    }
}
