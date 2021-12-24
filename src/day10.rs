use itertools::Itertools;
use std::fs;
use std::collections::HashMap;


pub fn part1() -> i128 {
    let lines = read_data();

    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('[', ']');
    map.insert('{', '}');
    map.insert('<', '>');

    let mut points = HashMap::new();
    points.insert(')', 3);
    points.insert(']', 57);
    points.insert('}', 1197);
    points.insert('>', 25137);

    let mut score = 0;

    for line in lines {
        // println!("Line: {}", line);
        let mut stack = Vec::new();
        for (i, c) in line.chars().enumerate() {
            match c {
                '(' | '[' | '{' | '<' => {stack.push(c);},
                ')' | ']' | '}' | '>' => {
                    if let Some(d) = stack.pop() {
                        let e = *map.get(&d).unwrap();
                        if c != e {
                            // println!("No match at i={}. Expected {} got {}", i, e, c);
                            score += points.get(&c).unwrap();
                        }
                    } else {
                        // println!("No match at i={}", i);
                    }
                },
                _ => {

                }
            }
        }
    }

    score
}

pub fn part2() -> i128 {
    1
}

fn read_data() -> Vec<String> {
    let values: String = fs::read_to_string("data/day10.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

fn _read_example() -> Vec<String> {
    let values: String = fs::read_to_string("data/day10.example").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_day10part1() {
        let result = part1();
        assert_eq!(442131, result);
    }

    #[test]
    fn test_day10part2() {
        let result = part2();
        assert_eq!(1168440, result);
    }
}