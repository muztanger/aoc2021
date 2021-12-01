use std::fs;
// use itertools::Itertools;

pub fn day01part1() -> String {
    let values = read_data();
    
    let mut x = 0;
    for i in 1..values.len() {
        // println!("{}", values[i]);
        if values[i - 1] < values[i] {
            x += 1;
        }        
    }

    x.to_string()
}

pub fn day01part2() -> String {
    let values = read_data();

    let mut count = 0;
    for i in 0..values.len() - 3 {
        let x = values[i] + values[i + 1] + values[i + 2];
        let y = values[i + 1] + values[i + 2] + values[i + 3];

        if x < y {
            count += 1;
        }
    }

    count.to_string()
}

fn read_data() -> Vec<usize> {
    let values: String = fs::read_to_string("data/day01.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<usize>().ok()).collect()
}

fn _read_example() -> Vec<usize> {
    let values: String = fs::read_to_string("data/day01_example.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<usize>().ok()).collect()
}