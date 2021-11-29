use aoc2021::day01::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");
    let result = match problem {
        "day01part1" => day01part1(),
        "day01part2" => day01part2(),
        _ => "Not added to command line".to_string()
    };

    println!("{}", result);
}
