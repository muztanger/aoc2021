use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::fs;
use std::fmt;
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


struct Cavern {
    cavern: Vec<Vec<i32>>,
    start: Pos,
    end: Pos,
    left: Pos,
    right: Pos,
    up: Pos,
    down: Pos,
    mem: HashMap<Pos, i32>
}

impl Cavern {
    pub fn new(lines: Vec<String>) -> Self {
        let cavern: Vec<Vec<i32>> = lines.iter().filter(|line| line.len() > 0).map(|line| line.chars().map(|x| x.to_digit(10).unwrap() as i32).collect::<Vec<i32>>()).collect();
        let width = cavern[0].len() as i32 - 1;
        let height = cavern.len() as i32 - 1;


        let mut mem = HashMap::new();
        let end = Pos{x: width, y: height};
        mem.insert(end, cavern[height as usize][width as usize]);

        Cavern {
            cavern: cavern, 
            start: Pos{x: 0, y:0 },
            end: end,
            left: Pos{x: -1, y: 0},
            right: Pos{x: 1, y: 0},
            up: Pos{x: 0, y: -1},
            down: Pos{x: 0, y: 1},
            mem: HashMap::new(),
        }
    }

    pub fn print(&self) {
        for j in 0..self.end.y as usize {
            for i in 0..self.end.x as usize {
                print!("{} ", self.cavern[j][i]);
            }
            println!();
        }
    }

    fn in_range(&self, pos: Pos) -> bool {
        !(pos.x < 0 || pos.y < 0 || pos.x > self.end.x || pos.y > self.end.y)
    }

    fn value_of(&self, pos: Pos) -> Option<i32> {
        if !self.in_range(pos) {
            None
        } else {
            Some(self.cavern[pos.y as usize][pos.x as usize])
        }
    }


    fn dijsktra(&self) -> i32 {
        let mut dist = vec![vec![i32::MAX; self.cavern[0].len()]; self.cavern.len()];
        let mut processed = vec![vec![false; self.cavern[0].len()]; self.cavern.len()];
        dist[0][0] = 0;

        let mut queue: BinaryHeap::<(i32, Pos)> = BinaryHeap::new();
        queue.push((0, Pos{x:0, y:0}));
        while !queue.is_empty() {
            let pos = queue.pop().unwrap().1;
            if processed[pos.y as usize][pos.x as usize] {
                continue;
            }
            processed[pos.y as usize][pos.x as usize] = true;
            for dp in vec![Pos{x:1, y:0}, Pos{x:0, y:1}, Pos{x:-1, y:0}, Pos{x:0, y:-1}] {
                let q = pos + dp;
                if self.in_range(q) {
                    if dist[pos.y as usize][pos.x as usize] + self.cavern[q.y as usize][q.x as usize] < dist[q.y as usize][q.x as usize] {
                        dist[q.y as usize][q.x as usize] = dist[pos.y as usize][pos.x as usize] + self.cavern[q.y as usize][q.x as usize];
                        queue.push((-dist[q.y as usize][q.x as usize], q));
                    }
                }
            }
        }

        for j in 0..dist.len() {
            for i in 0..dist[0].len() {
                print!("{:3?}", dist[j][i]);
            }
            println!();
        }

        dist[self.end.y as usize][self.end.x as usize]
    }

    fn search_min(&self, pos: Pos, mut cost: i32, min_cost: &mut i32) {
        //println!("search_min pos={} cost={} min_cost={}", pos, cost, min_cost);
        if let Some(v) = self.value_of(pos) {
            cost += v;
        } else {
            return;
        }

        // relax
        if cost > 9 * (pos.x + 1) * (pos.y + 1) {
            return;
        }

        if pos == self.end {
            //println!("Found end with cost: {}", cost);
            if cost < *min_cost {
                *min_cost = cost;
            }
            return;
        }

        for d in vec![Pos{x:1, y:0}, Pos{x:0, y:1}] { //Is it possible that I would need to go round?
            let next = pos + d;
            if self.in_range(next) {
                self.search_min(next, cost, min_cost);
            }
        }
    }

}

pub fn problem(file: String, n: usize) -> i128 {
    let lines: Vec<String> = fs::read_to_string(file).expect("Could not read file").split('\n').filter_map(|s | s.parse::<String>().ok()).collect();

    let cavern = Cavern::new(lines);
    cavern.print();

    // let min_cost = &mut i32::MAX.clone();
    // cavern.search_min(cavern.start, 0, min_cost);

    // *min_cost as i128 - cavern.cavern[0][0] as i128
    cavern.dijsktra().into()
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_part1_example() {
        let result = problem("data/day15.example".to_string(), 10);
        assert_eq!(40, result);
    }

    #[test]
    fn test_part1() {
        let result = problem("data/day15.txt".to_string(), 10);
        assert_eq!(2915, result);
    }

    #[test]
    fn test_part2() {
        assert_eq!(3353146900153, problem("data/day15.txt".to_string(), 40));
    }
}