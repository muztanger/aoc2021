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

// impl Pos {
//     fn signum(&self) -> Pos {
//         Pos {
//             x: self.x.signum(), 
//             y: self.y.signum()
//         }
//     }
// }

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
    end: Pos,
}

impl Cavern {
    pub fn new(lines: Vec<String>, n: usize) -> Self {
        let cavern: Vec<Vec<i32>> = lines.iter().filter(|line| line.len() > 0).map(|line| line.chars().map(|x| x.to_digit(10).unwrap() as i32).collect::<Vec<i32>>()).collect();
        let width = (cavern[0].len() * n - 1) as i32;
        let height = (cavern.len() * n - 1) as i32;
        let end = Pos{x: width, y: height};

        let mut large_cavern: Vec<Vec<i32>> = Vec::new();
        for j in 0..cavern.len() * n {
            let mut line: Vec<i32> = Vec::new();
            for i in 0..n {
                let z = (i + j / cavern.len()) as i32;
                line.extend(cavern[j % cavern.len()].iter().map(|x| (x - 1 + z as i32) % 9 + 1).collect::<Vec<i32>>());
            }
            large_cavern.push(line);
        }

        Cavern {
            cavern: large_cavern, 
            end: end,
        }
    }

    pub fn _print(&self) {
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

        // for j in 0..dist.len() {
        //     for i in 0..dist[0].len() {
        //         if dist[j][i] < 10000 {
        //             print!("{:4?}", dist[j][i]);
        //         } else {
        //             print!("MOOP");
        //         }

        //     }
        //     println!();
        // }

        dist[self.end.y as usize][self.end.x as usize]
    }

}

pub fn problem(file: String, n: usize) -> i128 {
    let lines: Vec<String> = fs::read_to_string(file).expect("Could not read file").split('\n').filter_map(|s | s.parse::<String>().ok()).collect();

    let cavern = Cavern::new(lines, n);
    // cavern.print();
    cavern.dijsktra().into()
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_part1_example() {
        let result = problem("data/day15.example".to_string(), 1);
        assert_eq!(40, result);
    }

    #[test]
    fn test_part1() {
        let result = problem("data/day15.txt".to_string(), 1);
        assert_eq!(609, result);
    }

    #[test]
    fn test_part2_example() {
        let result = problem("data/day15.example".to_string(), 5);
        assert_eq!(315, result);
    }

    #[test]
    fn test_part2_txt() {
        let result = problem("data/day15.txt".to_string(), 5);
        assert_eq!(2925, result);
    }
}