mod days;

macro_rules! days {
    ($($day:literal = $mod:ident),+ $(,)?) => {
        pub const ALL_DAYS: &[u8] = &[$( $day ),+];

        pub fn run(day: u8, part: Option<&str>) {
            if !ALL_DAYS.contains(&day) {
                panic!("Unknown day try one of {ALL_DAYS:?}");
            }

            let input_path = format!("{}/input/day{day:02}.txt", env!("CARGO_MANIFEST_DIR"));
            let Ok(input) = std::fs::read_to_string(&input_path) else {
                panic!("Could not read input for day{day:02} ({input_path}). Does the file exist?");
            };

            match (day, part) {
                $(
                    ($day, Some("part1")) => {
                        println!("{}", days::$mod::part1(&input));
                    }
                    ($day, Some("part2")) => {
                        println!("{}", days::$mod::part2(&input));
                    }
                    ($day, None) => {
                        println!("{}", days::$mod::part1(&input));
                        println!("{}", days::$mod::part2(&input));
                    }
                ),+
                _ => {
                    panic!("Unknown part try one of [part1|part2|<omit>]");
                }
            }
        }
    };
}

days! {
    1 = day01,
    2 = day02,
    3 = day03,
    4 = day04,
    5 = day05,
    6 = day06,
    7 = day07,
}
