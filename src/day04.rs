use std::fs;
use itertools::Itertools;
use regex::Regex;

// pub struct Mark

pub struct Board {
    pub grid: Vec<Vec<i32>>,
    pub mark: Vec<Vec<bool>>
}

impl Board {
    pub fn add_row(&mut self, row: Vec<i32>) {
        assert_eq!(5, row.len());
        self.grid.push(row);
    }

    pub fn len(&self) -> usize {
        self.grid.len()
    }

    pub fn draw(&mut self, draw: i32) {
        for (j, row) in self.grid.iter().enumerate() {
            for (i, cell) in row.iter().enumerate() {
                if draw == *cell {
                    self.mark[j][i] = true;
                }
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        // Check rows
        for row in &self.mark {
            if row.iter().all(|x| *x) {
                return true;
            }
        }
        
        // Check columns
        for col in 0..5 {
            let mut result = true;
            for row in 0..5 {
                let v: &Vec<bool> = &self.mark[row];
                if v[col] == false{
                    result = false;
                    break;
                }
            }
            if result {
                return true;
            }
        }
        false
    }
}

pub fn part1() -> i128 {
    let lines = _read_example();
    let draws : Vec<i32> = lines.first().unwrap().split(',').filter_map(|x| x.parse::<i32>().ok()).collect();
    
    let mut boards: Vec<Board> = Vec::new();
    let mut board = Board{grid: Vec::new(), mark: vec![vec![false; 5]; 5]};
    let re = Regex::new(r"\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();
    for line in lines.iter().skip(1) {
        let caps = re.captures(line);
        match caps {
            Some(caps) => {
                let row: Vec<i32> = caps.iter().filter_map(|x| x.unwrap().as_str().parse::<i32>().ok()).collect();
                board.add_row(row);
            },
            None => {
                if board.len() == 5 {
                    boards.push(board);
                    board = Board{grid: Vec::new(), mark: vec![vec![false; 5]; 5]};
                }
            }
        }
    }

    for (i, draw) in draws.iter().enumerate() {
        println!("Draw {}: {}", i + 1, draw);

    }


    boards.len() as i128
}

pub fn part2() -> i128 {
    let lines = _read_example();
    1    
}

fn read_data() -> Vec<String> {
    let values: String = fs::read_to_string("data/day04.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

fn _read_example() -> Vec<String> {
    let values: String = fs::read_to_string("data/day04.example").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_day4part1() {
        assert_eq!(3429254, part1());
    }

    #[test]
    fn test_day4part2() {
        assert_eq!(230, part2());
    }
}