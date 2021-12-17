use itertools::Itertools;
use regex::Regex;
//use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::ops;

pub fn part1() -> i128 {
    let input = read_data();
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
    let input = read_data();
    let fish_input: Vec<i128> = input.first().unwrap().split(",").filter_map(|x| x.parse::<i128>().ok()).collect();

    let mut fish: HashMap<i128, i128> = HashMap::new();
    for x in 0..10 {
        fish.insert(x, 0);
    }
    for f in fish_input {
        *fish.entry(f).or_insert(0) += 1;
    }

    // println!("Initial state: {}", fish.keys().sorted().map(|k| format!("{}:{}", k, fish.get(k).unwrap())).collect::<Vec<String>>().join(", "));

    let mut n = 256;
    while n != 0 {
        let old_count = fish.get(&0).unwrap().clone();
        let new_count = fish.entry(9).or_insert(0);
        *new_count += old_count;

        for key in 0..9 {
            fish.insert(key, fish.get(&(key + 1)).unwrap().clone());
        }

        *fish.get_mut(&6).unwrap() += old_count;
        *fish.get_mut(&9).unwrap() = 0;

        // print!("After {} day: ", 19 - n);
        // println!("{}", fish.keys().sorted().map(|k| format!("{}:{}", k, fish.get(k).unwrap())).collect::<Vec<String>>().join(", "));

        n -= 1;
    }
    
    fish.values().sum()
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
        assert_eq!(352872, part1());
    }

    #[test]
    fn test_day6part2() {
        assert_eq!(1604361182149, part2());
    }
}