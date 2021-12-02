use std::fs;
use std::str::FromStr;
use std::fmt;

struct Command {
    command: String,
    value: i64
}

struct CommandParseError;

impl fmt::Display for CommandParseError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { todo!() }
}

impl fmt::Debug for CommandParseError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { todo!() }
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        if split.len() != 2 {
            return Err(CommandParseError{});
        }
        let command_fromstr = split[0].parse::<String>().unwrap();
        let value_fromstr = split[1].parse::<i64>().unwrap();
        return Ok(Command {command: command_fromstr, value: value_fromstr})
    }
}

pub fn day2part1() -> i64 {
    let commands = read_data();
    
    let mut horizontal = 0;
    let mut depth = 0;

    for command in commands {
        match command.command.as_str() {
            "forward" => {horizontal += command.value},
            "down" => {depth += command.value},
            "up" => {depth -= command.value},
            _ => {}
        };
    }

    println!("{} {}", horizontal, depth);
    horizontal * depth
}

pub fn day2part2() -> i64 {
    let commands = read_data();
    
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match command.command.as_str() {
            "forward" => {horizontal += command.value; depth += aim * command.value},
            "down" => {aim += command.value},
            "up" => {aim -= command.value},
            _ => {}
        };
    }

    println!("{} {}", horizontal, depth);
    horizontal * depth
}

fn read_data() -> Vec<Command> {
    let values: String = fs::read_to_string("data/day02.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<Command>().ok()).collect()
}

fn _read_example() -> Vec<Command> {
    let values: String = fs::read_to_string("data/day02.example").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<Command>().ok()).collect()
}

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_day2part1() {
        assert_eq!(1660158, day2part1());
    }

    #[test]
    fn test_day2part2() {
        assert_eq!(1604592846, day2part2());
    }
}