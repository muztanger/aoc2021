use regex::Regex;
//use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::ops;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32
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

struct Line {
    p1: Pos,
    p2: Pos
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.p1, self.p2)
    }
}

impl Line {
    fn positions(&self) -> Vec<Pos> {
        let mut result = Vec::new();
        
        if !(self.p1.x == self.p2.x || self.p1.y == self.p2.y) {
            return result;
        }
        
        let step = (self.p2 - self.p1).signum();
        let mut pos = self.p1.clone();
        while pos != self.p2 {
            result.push(pos.clone());
            pos += step;
        }
        result.push(pos.clone());

        result
    }

    fn positions2(&self) -> Vec<Pos> {
        let mut result = Vec::new();

        let dt = self.p2 - self.p1;
        if !(dt.x == 0 || dt.y == 0 || dt.x.abs() == dt.y.abs())
        {
            return result;
        }

        let step = dt.signum();
        let mut pos = self.p1.clone();
        while pos != self.p2 {
            result.push(pos.clone());
            pos += step;
        }
        result.push(pos.clone());

        result
    }
}


fn parse_input(input: Vec<String>) -> Vec<Line> {
    let re = Regex::new(r"\s*(\d+),(\d+)[^\d]+(\d+),(\d+)").unwrap();

    let mut lines: Vec<Line> = Vec::new();
    for row in input {
        let caps = re.captures(&row);
        match caps {
            Some(caps) => {
                let p1 = Pos {
                    x: caps[1].parse::<i32>().unwrap(),
                    y: caps[2].parse::<i32>().unwrap()
                };
                let p2 = Pos {
                    x: caps[3].parse::<i32>().unwrap(),
                    y: caps[4].parse::<i32>().unwrap()
                };
                lines.push(Line{p1, p2});
            },
            None => {}
        }
    }

    lines
}

// fn find_min_max(lines: Vec<Line>) -> (Pos, Pos) {
//     let mut min = Pos{x:i32::MAX, y:i32::MAX};
//     let mut max = Pos{x:i32::MIN, y:i32::MIN};
//     for line in lines {
//         min.x = cmp::min(min.x, cmp::min(line.p1.x, line.p2.x));
//         min.y = cmp::min(min.y, cmp::min(line.p1.y, line.p2.y)); 
//         max.x = cmp::max(max.x, cmp::max(line.p1.x, line.p2.x));
//         max.y = cmp::max(max.y, cmp::max(line.p1.y, line.p2.y)); 
//     }
//     (min, max)
// }

pub fn part1() -> i128 {
    let input = read_data();
    let lines = parse_input(input);

    let mut count: HashMap<Pos, i32> = HashMap::new();
    for line in &lines {
        // println!("Line: {}", line);
        // println!("  Positions: [{}]", line.positions().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "));
        for p in line.positions() {
            let x = count.entry(p).or_insert(0);
            *x += 1;
        }
    }

    // let (min, max) = find_min_max(lines);
    // for y in min.y .. max.y + 1 {
    //     for x in min.x .. max.x + 1 {
    //         let pos = Pos {x, y};
            
    //         if count.contains_key(&pos) {
    //             print!("{}", count.get(&pos).unwrap().to_string());
    //         } else {
    //             print!(".");
    //         }

    //     }
    //     println!("");
    // }

    // println!("count: {}", count.iter().filter(|(_, v)| **v > 1).map(|(k, v)| format!("{}:{}", k.to_string(), v.to_string())).collect::<Vec<String>>().join(" "));

    count.iter().filter(|(_, value)| **value > 1).count() as i128
}

pub fn part2() -> i128 {
    let input = read_data();
    let lines = parse_input(input);

    let mut count: HashMap<Pos, i32> = HashMap::new();
    for line in &lines {
        // println!("Line: {}", line);
        // println!("  Positions: [{}]", line.positions().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "));
        for p in line.positions2() {
            let x = count.entry(p).or_insert(0);
            *x += 1;
        }
    }

    // let (min, max) = find_min_max(lines);
    // for y in min.y .. max.y + 1 {
    //     for x in min.x .. max.x + 1 {
    //         let pos = Pos {x, y};
            
    //         if count.contains_key(&pos) {
    //             print!("{}", count.get(&pos).unwrap().to_string());
    //         } else {
    //             print!(".");
    //         }

    //     }
    //     println!("");
    // }

    // println!("count: {}", count.iter().filter(|(_, v)| **v > 1).map(|(k, v)| format!("{}:{}", k.to_string(), v.to_string())).collect::<Vec<String>>().join(" "));

    count.iter().filter(|(_, value)| **value > 1).count() as i128
}

fn read_data() -> Vec<String> {
    let values: String = fs::read_to_string("data/day05.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

fn _read_example() -> Vec<String> {
    let values: String = fs::read_to_string("data/day05.example").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_day5part1() {
        let result = part1();
        assert_ne!(678, result);
        assert_eq!(6687, result);
    }

    #[test]
    fn test_day5part2() {
        assert_eq!(19851, part2());
    }
}