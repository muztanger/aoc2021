use std::fmt;
use std::fs;
use std::ops;
use std::cmp::Ordering;
use regex::Regex;

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

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
     (self.x * self.x + self.y * self.y).cmp(&(other.x * other.x + other.y * other.y)) //TODO Is this good enough right now??
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::ops::Div<i32> for Pos {
    type Output = Pos;
    fn div(self, n: i32) -> <Self as std::ops::Div<i32>>::Output {
         Pos{
             x: self.x / n,
             y: self.y / n,
         }
    }
}

struct Area {
    upper_left: Pos,
    lower_right: Pos,
}

impl Area {
    fn is_inside(&self, pos: Pos) -> bool {
        pos.x >= self.upper_left.x 
            && pos.x <= self.lower_right.x
            && pos.y >= self.lower_right.y 
            && pos.y <= self.upper_left.y
    }

    fn cmp(&self, pos: Pos) -> Pos {
        let mut x = 0;
        if pos.x < self.upper_left.x {
            x = -1;
        } else if pos.x > self.lower_right.x {
            x = 1;
        }

        let mut y = 0;
        if pos.y > self.upper_left.y {
            y = 1;
        } else if pos.y < self.lower_right.y {
            y = -1;
        }

        Pos {x: x, y: y}
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.upper_left, self.lower_right)
    }
}

struct Probe {
    pos: Pos,
    velocity: Pos,
}

impl Probe {
    fn step(&mut self) {
        self.pos += self.velocity;
        self.velocity.x = self.velocity.signum().x * (self.velocity.x.abs() - 1);
        self.velocity.y -= 1;
    }
}


pub fn problem(file: String, n: usize) -> i128 {
    let lines: Vec<String> = fs::read_to_string(file).expect("Could not read file").split('\n').filter_map(|s | s.parse::<String>().ok()).collect();

    //target area: x=150..193, y=-136..-86
    let re = Regex::new(r"target area:\s+x=(-?\d+)\.\.(-?\d+),\s+y=(-?\d+)\.\.(-?\d+)").expect("Failed to create regular expression");
    let captures = re.captures(lines.first().expect("Failed to get first line")).expect("Failed to get captures");
    assert_eq!(4 + 1, captures.len());
    let upper_left = Pos {
        x: captures.get(1).unwrap().as_str().parse::<i32>().expect("Failed to parse"),
        y: captures.get(4).unwrap().as_str().parse::<i32>().expect("Failed to parse"),
    };
    let lower_right = Pos {
        x: captures.get(2).unwrap().as_str().parse::<i32>().expect("Failed to parse"),
        y: captures.get(3).unwrap().as_str().parse::<i32>().expect("Failed to parse"),
    };
    let target_area = Area{upper_left: upper_left, lower_right: lower_right};
    println!("target_area: {}", target_area);

    let mut probe = Probe {pos: Pos{x:0, y:0}, velocity:Pos {x:6, y: 9}};
    println!("probe.pos: {}", probe.pos);
    println!("side: {}", target_area.cmp(probe.pos));
    println!("v: {}", probe.velocity);
    
    for _ in 0..30 {
        probe.step();
        println!("probe.pos: {}", probe.pos);
        println!("side: {}", target_area.cmp(probe.pos));
        println!("inside: {}", target_area.is_inside(probe.pos));
    }

    1
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use crate::day17::*;

    #[test]
    fn test_day17_part1_example() {
        let result = problem("data/day17.example".to_string(), 1);
        assert_eq!(40, result);
    }

    #[test]
    fn test_day17_part1_input() {
        let result = problem("data/day17.input".to_string(), 1);
        assert_eq!(40, result);
    }


    #[test]
    fn test_day17_part2_txt() {
        let result = problem("data/day17.txt".to_string(), 2);
        assert_eq!(539051801941, result);
    }
}