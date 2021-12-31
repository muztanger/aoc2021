use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::fmt;
use std::ops;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i128,
    y: i128
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign for Pos {
    fn add_assign(&mut self, other: Self) {
         *self = *self + other;
    }
}

impl ops::Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Neg for Pos {
    type Output = Self;
    
    fn neg(self) -> <Self as std::ops::Neg>::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}

impl Pos {
    fn signum(&self) -> Pos {
        Pos {
            x: self.x.signum(), 
            y: self.y.signum()
        }
    }
}

struct Poss {
    poss: Vec<Pos>,
    x_min: i128,
    y_min: i128,
    x_max: i128,
    y_max: i128,
}

impl Poss {
    pub fn new() -> Poss {
        Poss {
            poss: Vec::new(),
            x_min: i128::MAX,
            x_max: i128::MIN,
            y_min: i128::MAX,
            y_max: i128::MIN
        }
    }

    pub fn push(&mut self, pos: &Pos) {
        self.x_min = cmp::min(self.x_min, pos.x);
        self.x_max = cmp::max(self.x_max, pos.x);
        self.y_min = cmp::min(self.y_min, pos.y);
        self.y_max = cmp::max(self.y_max, pos.y);
        self.poss.push(*pos);
    }

    pub fn len(&self) -> usize {
        self.poss.len()
    }

    fn reset_min_max(&mut self) {
        let first = self.poss[0];
        self.x_min = first.x;
        self.y_min = first.y;
        self.x_max = first.x;
        self.y_max = first.y;
        for i in 1..self.poss.len() {
            self.x_min = cmp::min(self.x_min, self.poss[i].x);
            self.y_min = cmp::min(self.y_min, self.poss[i].y);
            self.x_max = cmp::max(self.x_max, self.poss[i].x);
            self.y_max = cmp::max(self.y_max, self.poss[i].y);
        }
    }

    pub fn fold(&mut self, fold: &Pos) {
        assert_eq!(0, fold.x * fold.y);
        let mut remove = vec![];
        let mut add = vec![];
        for p in &self.poss {
            if fold.x == 0 {
                if p.y > fold.y {
                    let q = Pos{x: p.x, y: fold.y - (p.y - fold.y)};
                    println!("{} -> {}", p, q);
                    remove.push(*p);
                    add.push(q)
                }
            } else {
                if p.x > fold.x {
                    let q = Pos{x: fold.x - (p.x - fold.x), y: p.y};
                    println!("{} -> {}", p, q);
                    remove.push(*p);
                    add.push(q)
                }
            }
        }
        for p in remove {
            self.poss.retain(|&x| x != p);
        }
        for p in add {
            if !self.poss.contains(&p) {
                self.poss.push(p);
            }
        }
        self.reset_min_max();
    }

    pub fn print(&self) {
        println!("min:({}, {}) max:({}, {})", self.x_min, self.y_min, self.x_max, self.y_max);
        println!("poss: {}", self.poss.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", "));
        for y in self.y_min .. self.y_max + 1 {
            for x in self.x_min .. self.x_max + 1 {
                if self.poss.contains(&Pos{x:x,y:y}) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

pub fn calc(file: String, n: usize) -> i128 {
    let lines: Vec<String> = fs::read_to_string(file).expect("Could not read file").split('\n').filter_map(|s | s.parse::<String>().ok()).collect();
    let re_points = Regex::new(r"\s*(\d+),(\d+)").unwrap();
    let re_folds = Regex::new(r"\s*fold along (.)=(\d+)").unwrap();

    let mut points: Poss = Poss::new();
    let mut folds: Vec<Pos> = Vec::new();
    for line in lines {
        if let Some(point_caps) = re_points.captures(line.as_str()) {
            points.push(&Pos {
                x: point_caps.get(1).unwrap().as_str().parse::<i128>().unwrap(),
                y: point_caps.get(2).unwrap().as_str().parse::<i128>().unwrap()
            });
        } else if let Some(fold_caps) = re_folds.captures(line.as_str()) {
            match fold_caps.get(1).unwrap().as_str() {
                "x" => {
                    folds.push(Pos {
                        x: fold_caps.get(2).unwrap().as_str().parse::<i128>().unwrap(),
                        y: 0
                    });
                },
                "y" => {
                    folds.push(Pos {
                        x: 0,
                        y: fold_caps.get(2).unwrap().as_str().parse::<i128>().unwrap()
                    });
                },
                _ => {}
            }
        }
    }

    // points.print();
    for i in 0..n {
        points.fold(&folds[i]);
    }
    // points.fold(&folds[1]);
    if n == folds.len() {
        points.print();
    }
    
    println!("points: {}\n folds: {}", points.len(), folds.len());

    points.len() as i128
}

pub fn part2() -> i128 {
    1
}

fn read_data(file: String) -> Vec<String> {
    let values: String = fs::read_to_string(file).expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

fn _read_example() -> Vec<String> {
    let values: String = fs::read_to_string("data/day13.example").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_day13_part1_example() {
        let result = calc("data/day13.example".to_string(), 1);
        assert_eq!(17, result);
    }

    #[test]
    fn test_day13_part1() {
        let result = calc("data/day13.txt".to_string(), 1);
        assert_eq!(781, result);
    }


    #[test]
    fn test_day10part2() {
        calc("data/day13.txt".to_string(), 12);
        //PERCGJPB
    }
}