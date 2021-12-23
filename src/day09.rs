use itertools::Itertools;
use std::fs;
use std::collections::HashMap;

pub fn part1() -> i128 {
    let lines = read_data();

    let width = lines.first().unwrap().len();
    let height = lines.len();
    let mut curve = vec![vec![0 as i128; width]; height];
    for (j, row) in lines.iter().enumerate() {
        for (i, col) in row.chars().enumerate() {
            if let Some(x) = col.to_digit(10) {
                curve[j][i] = x as i128;
            }
        }
    }

    // let mut low_points: Vec<i128> = Vec::new();
    let mut result = 0;
    for j in 0..height {
        for i in 0..width {
            let x = curve[j][i];
            let mut is_low = true;
            if i > 0 && curve[j][i - 1] <= x { is_low = false; } // left
            if i + 1 < width && curve[j][i + 1] <= x { is_low = false; } // right
            if j > 0 && curve[j - 1][i] <= x { is_low = false; } // up
            if j + 1 < height && curve[j + 1][i] <= x { is_low = false; } // down
            
            if is_low {
                // println!("({},{})", i, j);
                // low_points.push(x + 1);
                result += x + 1;
            }
        }
    }

    // low_points.iter().sum::<i128>()
    result
}


pub fn part2() -> i128 {
    1 as i128
}

fn read_data() -> Vec<String> {
    let values: String = fs::read_to_string("data/day09.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

fn _read_example() -> Vec<String> {
    let values: String = fs::read_to_string("data/day09.example").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_day9part1() {
        let result = part1();
        assert_ne!(441, result);
        assert_eq!(444, result);
    }

    #[test]
    fn test_day9part2() {
        let result = part2();
        assert_ne!(944669, result); // is too low
        assert!(result > 944669);
        assert_eq!(946346, result);
    }
}