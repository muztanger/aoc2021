use std::fs;
use itertools::Itertools;

pub fn day01a() -> String{
    let values = read_data();
    
    let result = values.iter().combinations(2)
        .find(|v| v[0] + v[1] == 2020)
        .map(|v | v.into_iter().product::<usize>());

    match result {
        Some(v) => v.to_string(),
        None => "No result".to_string()
    }
}

fn read_data() -> Vec<usize> {
    let values: String = fs::read_to_string("data/day01.txt").expect("Could not read file");
    values
        .split('\n')
        .filter_map(|s | s.parse::<usize>().ok())
        .collect()
}