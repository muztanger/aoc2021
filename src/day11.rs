use std::fs;
use std::collections::HashMap;

struct Octopus {
    energy: i32,
    flash: bool
}

struct Octopuses {
    octopuses: Vec<Vec<Octopus>>
}

impl Octopuses {
    pub fn size(&mut self) -> i32 {
        (self.octopuses.len() * self.octopuses[0].len()) as i32
    }

    pub fn print(&mut self) {
        for j in 0..self.octopuses.len() {
            for i in 0..self.octopuses[0].len() {
                print!("{} ", self.octopuses[j][i].energy)
            }
            println!();
        }
    }

    pub fn step(&mut self) {
        for j in 0..self.octopuses.len() {
            for i in 0..self.octopuses[0].len() {
                self.octopuses[j][i].energy += 1;
            }
        }
    }

    pub fn flash(&mut self) {
        let mut flashes = 0;
        loop {
            for j in 0..self.octopuses.len() {
                for i in 0..self.octopuses[0].len() {
                    let mut octopus = &mut self.octopuses[j][i];
                    if octopus.flash == false && octopus.energy > 9 {
                        octopus.flash = true;
                        flashes += 1;

                        // up
                        if j > 0 { self.octopuses[j - 1][i].energy += 1;}
    
                        // up right
                        if j > 0 && i < self.octopuses[j].len() - 1 { self.octopuses[j - 1][i + 1].energy += 1;}
                        
                        // right
                        if i < self.octopuses[0].len() - 1 { self.octopuses[j][i + 1].energy += 1;}
    
                        // down right
                        if j < self.octopuses.len() - 1 && i < self.octopuses[j].len() - 1 { self.octopuses[j + 1][i + 1].energy += 1;}

                        // down
                        if j < self.octopuses.len() - 1 { self.octopuses[j + 1][i].energy += 1;}

                        // down left
                        if j < self.octopuses.len() - 1 && i > 0 { self.octopuses[j + 1][i - 1].energy += 1;}

                        // left
                        if i > 0 { self.octopuses[j][i - 1].energy += 1;}

                        // up left
                        if i > 0 && j > 0 { self.octopuses[j - 1][i - 1].energy += 1;}
                    }
                } 
            }

            if flashes == 0 {break;}
            flashes = 0;
        }
    }

    pub fn reset(&mut self) -> i32 {
        let mut flashes = 0;
        for j in 0..self.octopuses.len() {
            for i in 0..self.octopuses[0].len() {
                if self.octopuses[j][i].flash {
                    flashes += 1;
                } 
                self.octopuses[j][i].flash = false;

                if self.octopuses[j][i].energy > 9 {
                    self.octopuses[j][i].energy = 0;
                }
            }
        }
        flashes
    }
}

fn create_octopuses(lines: Vec<String>) -> Octopuses {
    Octopuses{octopuses: lines
        .iter()
        .map(|line| line
            .chars()
            .map(|x| Octopus{energy: x.to_digit(10).unwrap() as i32, flash:false})
            .collect())
        .collect()}
}

pub fn part1() -> i128 {
    let lines = read_data();
    
    let mut octopuses = create_octopuses(lines);
    octopuses.print();
    println!();

    let mut flashes = 0;
    for step in 1..101 {
        octopuses.step();
        octopuses.flash();
        flashes += octopuses.reset();

        // println!("After step {}", step);
        // octopuses.print();
    }
    flashes as i128
}

pub fn part2() -> i128 {
    let lines = read_data();
    
    let mut octopuses = create_octopuses(lines);
    octopuses.print();
    println!();

    let mut step = 1;
    loop {
        octopuses.step();
        octopuses.flash();
        if octopuses.reset() == octopuses.size() {
            break;
        }

        step += 1;
        // println!("After step {}", step);
        // octopuses.print();
    }
    step as i128
}

fn read_data() -> Vec<String> {
    let values: String = fs::read_to_string("data/day11.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).filter(|x| x.len() > 0).collect()
}

fn _read_example() -> Vec<String> {
    let values: String = fs::read_to_string("data/day11.example").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).filter(|x| x.len() > 0).collect()
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_day11part1() {
        let result = part1();
        assert_eq!(1585, result);
    }

    #[test]
    fn test_day11part2() {
        let result = part2();
        assert_eq!(382, result);
    }
}