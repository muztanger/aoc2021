use std::fs;
// use itertools::Itertools;

pub fn day01part1() -> String {
    return calculate(2);
}

pub fn day01part2() -> String{
    return calculate(3);
}

#[test]
fn day01part2_test() {
    assert_eq!("203481432".to_string(), day01part2());
}

#[test]
fn day01part1_test() {
    assert_eq!("877971".to_string(), day01part1());
}

fn calculate(x: usize) -> String {
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

fn read_data() -> Vec<usize> {
    let values: String = fs::read_to_string("data/day01.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<usize>().ok()).collect()
}