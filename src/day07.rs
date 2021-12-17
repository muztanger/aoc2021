use std::fs;
use std::cmp;

pub fn part1() -> i128 {
    let input = read_data();
    let crabs: Vec<i128> = input.first().unwrap().split(",").filter_map(|x| x.parse::<i128>().ok()).collect();
    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    let mut min_fuel = i128::MAX;
    for x in *min .. max+1 {
        let fuel: i128 = crabs.iter().map(|y| (x-y).abs()).sum();
        min_fuel = cmp::min(min_fuel, fuel);
    }

    min_fuel
}

fn dist(x: i128, y: i128) -> i128 {
    if x == y {
        0
    } else {

        (0..((x-y).abs() + 1)).sum()
    }
}

pub fn part2() -> i128 {
    let input = read_data();
    let crabs: Vec<i128> = input.first().unwrap().split(",").filter_map(|x| x.parse::<i128>().ok()).collect();
    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    let mut min_fuel = i128::MAX;
    for x in *min .. max+1 {
        let fuel: i128 = crabs.iter().map(|y| dist(x, *y)  ).sum();
        min_fuel = cmp::min(min_fuel, fuel);
    }

    min_fuel
}

fn read_data() -> Vec<String> {
    let values: String = fs::read_to_string("data/day07.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

fn _read_example() -> Vec<String> {
    let values: String = fs::read_to_string("data/day07.example").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_day7part1() {
        assert_eq!(329389, part1());
    }

    #[test]
    fn test_day7part2() {
        assert_eq!(86397080, part2());
    }
}