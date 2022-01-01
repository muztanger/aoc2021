use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::fmt;
use std::ops;

struct Rule {
    pair: String,
    replace: String 
}

pub fn get_next(template: &String, rules: &HashMap<String, String>) -> String {
    let mut next: Vec<String> = vec![template.chars().take(1).collect::<String>()];
    for r in template.chars().zip(template.chars().skip(1)).map(
        |(a, b)| format!("{}{}", rules.get(&format!("{}{}", a, b)).unwrap(), b)) {
            next.push(r);
    }
    next.concat()
}

pub fn calc(file: String, n: usize) -> i128 {
    let lines: Vec<String> = fs::read_to_string(file).expect("Could not read file").split('\n').filter_map(|s | s.parse::<String>().ok()).collect();
    let polymer_template = lines.first().unwrap();
    println!("Template: {}", polymer_template);

    let re = Regex::new(r"\s*(\w+)\s*->\s*(\w+)").unwrap();
    let mut rules: HashMap<String, String> = HashMap::new(); 
    for cap in lines.iter().skip(2).filter_map(|line| re.captures(line.as_str())) {
        rules.insert(
            cap.get(1).unwrap().as_str().to_string(),
            cap.get(2).unwrap().as_str().to_string());
    }

    println!("Rules: {:?}", rules);

    let mut next = polymer_template.clone();
    for _ in 0..10 {
        next = get_next(&next, &rules);
        //println!("Next: {:?}", next);
    }

    let mut count: HashMap<char, i128> = HashMap::new();
    for c in next.chars() {
        if !count.contains_key(&c) {
            count.insert(c, 1);
        } else {
            if let Some(x) = count.get_mut(&c) {
                *x += 1;
            }
        }
    }

    println!("{:?}", count);

    (count.values().max().unwrap() - count.values().min().unwrap()) as i128
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_day14_part1_example() {
        let result = calc("data/day14.example".to_string(), 1);
        assert_eq!(1588, result);
    }

    #[test]
    fn test_day14_part1() {
        let result = calc("data/day14.txt".to_string(), 1);
        assert_eq!(781, result);
    }


    #[test]
    fn test_day14_part2() {
        calc("data/day14.txt".to_string(), 12);
    }
}