use std::fs;
use std::collections::{HashMap,HashSet};

pub fn part1() -> i128 {
    let lines = read_data();
    let mut count = 0;
    for line in lines.iter().filter(|x| x.len() > 0) {
        let output = line.split("|").skip(1).collect::<Vec<&str>>().first().unwrap().trim();
        let output: Vec<&str> = output.split(" ").collect();
        //1, 4, 7, and 8
        //2, 4, 3, 7
        count += output.iter().filter(|x| x.len() == 2 || x.len() == 4 || x.len() == 3 || x.len() == 7).count();
    }
    count as i128
}


pub fn part2() -> i128 {
    let lines = _read_example();
    let mut count = 0;
    for line in lines.iter().filter(|x| x.len() > 0) {
        println!("Line: {}", &line);
        let line: Vec<&str> = line.split("|").collect::<Vec<&str>>();
        assert_eq!(2, line.len());
        let input: Vec<&str> = line[0].split_whitespace().collect();

        let mut candidates: HashMap<char, HashSet<char>> = HashMap::new();
        let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        for c in letters {
            candidates.insert(c, letters.iter().cloned().collect());
        }

        //  aaaa
        // b    c
        // b    c
        //  dddd
        // e    f
        // e    f
        //  gggg
        for word in input {
            let remove: Vec<char> = match word.len() {
                // Digit 1
                2 => {vec!['a', 'b', 'd', 'e', 'g']},
                
                // Digit 4
                4 => {vec!['a', 'b', 'g']},

                // Digit 7
                3 => {vec!['b', 'd', 'e', 'g']},

                _ => {vec![]}
            };
            for letter in &remove {
                let cand = candidates.get_mut(letter).unwrap();
                for w in word.chars() {
                    cand.remove(&w);
                }
            }
        }
        let mut keys = candidates.keys().copied().collect::<Vec<_>>();
        keys.sort();
        for candidate in keys {
            let mut values = candidates.get(&candidate).unwrap().iter().map(|x| x.to_string()).collect::<Vec<String>>();
            values.sort();
            println!("{}: {}", candidate, values.join(","));
        }

        let output: Vec<&str> = line[1].split_whitespace().collect();

        //1, 4, 7, and 8
        //2, 4, 3, 7
        count += output.iter().filter(|x| x.len() == 2 || x.len() == 4 || x.len() == 3 || x.len() == 7).count();
    }
    count as i128
}

fn read_data() -> Vec<String> {
    let values: String = fs::read_to_string("data/day08.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

fn _read_example() -> Vec<String> {
    let values: String = fs::read_to_string("data/day08.example").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_day8part1() {
        assert_eq!(239, part1());
    }

    #[test]
    fn test_day8part2() {
        assert_eq!(61229, part2());
    }
}