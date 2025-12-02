pub use aoc_2025::run;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let day = args
        .first()
        .expect("The [day] argument is required")
        .parse::<u8>()
        .expect("First argument should be the day");
    let part = args.get(1).map(|part| part.as_str());
    run(day, part);
}
