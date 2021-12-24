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
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => {stack.push(c);},
                ')' | ']' | '}' | '>' => {
                    if let Some(d) = stack.pop() {
                        let e = *map.get(&d).unwrap();
                        if c != e {
                            score += points.get(&c).unwrap();
                        }
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
    let lines = read_data();

    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('[', ']');
    map.insert('{', '}');
    map.insert('<', '>');

    let mut points = HashMap::new();
    points.insert('(', 1);
    points.insert('[', 2);
    points.insert('{', 3);
    points.insert('<', 4);

    let mut scores = Vec::new();

    'line: for line in lines {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => {stack.push(c);},
                ')' | ']' | '}' | '>' => {
                    if let Some(d) = stack.pop() {
                        let e = *map.get(&d).unwrap();
                        if c != e {
                            continue 'line;
                        }
                    } else {
                        continue 'line;
                    }
                },
                _ => {

                }
            }
        }

        if stack.len() > 0 {
            let mut stack_score = 0;
            while let Some(c) = stack.pop() {
                stack_score *= 5;
                stack_score += points.get(&c).unwrap();
            }
            // println!("stack_score={}", stack_score);
            scores.push(stack_score);
        }
    }
    scores.sort();
    scores[scores.len() / 2]
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
        assert_eq!(3646451424, result);
    }
}