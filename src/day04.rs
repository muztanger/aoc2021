use std::fs;
use regex::Regex;

pub struct Cell {
    pub value: i32,
    pub mark: bool
}

pub struct Board {
    grid: Vec<Vec<Cell>>,
    has_won: bool
}

impl Board {
    pub fn add_row(&mut self, row: Vec<i32>) {
        assert_eq!(5, row.len());
        self.grid.push(row.iter().map(|r| Cell{value:*r, mark:false}).collect());
    }

    pub fn len(&self) -> usize {
        self.grid.len()
    }

    pub fn draw(&mut self, draw: i32) {
        if self.has_won {
            return;
        }

        for row in 0..5 {
            for col in 0..5 {
                if draw == self.grid[row][col].value {
                    self.grid[row][col].mark = true;
                }
            }
        }
    }

    pub fn is_winner(&mut self) -> bool {
        if self.has_won {
            return true;
        }

        // Check rows
        for row in &self.grid {
            if row.iter().all(|x| x.mark) {
                self.has_won = true;
                return true;
            }
        }
        
        // Check columns
        for col in 0..5 {
            let mut result = true;
            for row in 0..5 {
                if self.grid[row][col].mark == false{
                    result = false;
                    break;
                }
            }
            if result {
                self.has_won = true;
                return true;
            }
        }
        false
    }

    pub fn score(&self) -> i32 {
        let mut score = 0;
        for j in 0..5 {
            for i in 0..5 {
                if self.grid[j][i].mark == false {
                    score += self.grid[j][i].value;
                }
            }
        }
        score
    }
}

pub fn create_board() -> Board {
    Board{grid: Vec::new(), has_won: false}
}

pub fn create_boards(lines: Vec<String>) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    let mut board = create_board();
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
                    board = create_board();
                }
            }
        }
    }

    boards
}

pub fn part1() -> i128 {
    let lines = read_data();

    let draws : Vec<i32> = lines.first().unwrap().split(',').filter_map(|x| x.parse::<i32>().ok()).collect();
    let mut boards = create_boards(lines);

    let mut score: i32 = -1;
    let mut last_draw: i32 = -1;
    'outer: for (i, draw) in draws.iter().enumerate() {
        println!("Draw {}: {}", i + 1, draw);

        for b in 0..boards.len() {
            boards[b].draw(*draw);
            if boards[b].is_winner() {
                println!("Board {} is winner!", b);
                score = boards[b].score();
                last_draw = *draw;
                break 'outer;
            }
        }
    }

    println!("score={}, score*draw={}", score, last_draw * score);
    (score * last_draw) as i128
}

pub fn part2() -> i128 {
    let lines = read_data();

    let draws : Vec<i32> = lines.first().unwrap().split(',').filter_map(|x| x.parse::<i32>().ok()).collect();
    let mut boards = create_boards(lines);

    let mut score: i32 = -1;
    let mut last_draw: i32 = -1;
    for (i, draw) in draws.iter().enumerate() {
        println!("Draw {}: {}", i + 1, draw);

        for b in 0..boards.len() {
            if boards[b].is_winner() {
                continue;
            }
            boards[b].draw(*draw);
            if boards[b].is_winner() {
                println!("Board {} is winner!", b);
                score = boards[b].score();
                last_draw = *draw;
            }
        }
    }

    println!("score={}, score*draw={}", score, last_draw * score);
    (score * last_draw) as i128
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
        assert_eq!(21607, part1());
    }

    #[test]
    fn test_day4part2() {
        assert_eq!(19012, part2());
    }
}