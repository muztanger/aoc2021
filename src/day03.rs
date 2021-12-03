use std::fs;
use itertools::Itertools;

pub fn day2part1() -> i128 {
    let lines = read_data();
    
    let line_count = lines.len();
    
    let mut counting = Vec::new();
    for line in lines {
        for (i, col) in line.chars().enumerate() {
            if counting.len() <= i {
                counting.push(0);
            }
            if col == '1' {
                counting[i] += 1;
            }
        }
    }
    let mut gamma_rate = Vec::new();
    let mut epsilon_rate = Vec::new();
    println!("counting: {}", counting.iter().join(""));
    for x in counting {
        if x > line_count / 2{
            gamma_rate.push(1);
            epsilon_rate.push(0);
        } else {
            gamma_rate.push(0);
            epsilon_rate.push(1);
        }
    }
    let gamma_str = gamma_rate.iter().join("");
    let epsilon_str = epsilon_rate.iter().join("");
    println!("gamma rate: {}\nepsilon rate: {}", gamma_str, epsilon_str);
    let gamma = i128::from_str_radix(&gamma_str.to_string(), 2).unwrap();
    let epsilon = i128::from_str_radix(&epsilon_str.to_string(), 2).unwrap();
    println!("gamma={} epsilon={}", gamma, epsilon);
    gamma * epsilon
}

pub fn day2part2() -> i64 {
    // let commands = read_data();
    
    // let mut horizontal = 0;
    // let mut depth = 0;
    // let mut aim = 0;

    // for command in commands {
    //     match command.command.as_str() {
    //         "forward" => {horizontal += command.value; depth += aim * command.value},
    //         "down" => {aim += command.value},
    //         "up" => {aim -= command.value},
    //         _ => {}
    //     };
    // }

    // println!("{} {}", horizontal, depth);
    // horizontal * depth
    0
}

fn read_data() -> Vec<String> {
    let values: String = fs::read_to_string("data/day03.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

fn _read_example() -> Vec<String> {
    let values: String = fs::read_to_string("data/day03.example").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_day2part1() {
        assert_eq!(3429254, day2part1());
    }

    #[test]
    fn test_day2part2() {
        assert_eq!(1604592846, day2part2());
    }
}