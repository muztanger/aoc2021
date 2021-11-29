use aoc2021::day01::day01a;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");
    let result = match problem {
        "day01a" => day01a(),
        _ => "Solve this".to_string()
    };

    println!("{}", result);
}
