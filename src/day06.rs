use regex::Regex;
//use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::ops;

pub fn part1() -> i128 {
    let input = _read_example();
    let mut fish: Vec<i128> = input.first().unwrap().split(",").filter_map(|x| x.parse::<i128>().ok()).collect();

    //println!("Initial state: {}", fish.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
    let mut n = 80;
    let mut count = 0;
    while n != 0 {
        fish.extend(vec![9; count]);
        count = 0;
        for i in 0..fish.len() {
            fish[i] -= 1;
            if fish[i] == 0 {
                count += 1;
            } else if fish[i] == -1 {
                fish[i] = 6;
            }
        }
        // print!("After {} day: ", 19 - n);
        // println!("{}", fish.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));

        n -= 1;
    }
    fish.len() as i128
}

pub fn part2() -> i128 {
   1
}

fn read_data() -> Vec<String> {
    let values: String = fs::read_to_string("data/day06.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

fn _read_example() -> Vec<String> {
    let values: String = fs::read_to_string("data/day06.example").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_day6part1() {
        assert_eq!(5934, part1());
    }

    #[test]
    fn test_day6part2() {
        assert_eq!(19851, part2());
    }
}