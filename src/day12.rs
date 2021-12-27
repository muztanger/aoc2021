use std::fmt;
use std::fs;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

struct Cave {
    name: String,
    neighbours: Vec<String>
}

impl Cave {
    pub fn new(name: String) -> Cave {
        Cave {name: name, neighbours: Vec::new()}
    }

    pub fn add_neighbour(&mut self, other: String) {
        if !self.neighbours.contains(&other) {
            self.neighbours.push(other);
            self.neighbours.sort();
        }
    }
}

impl PartialEq for Cave {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Cave {}

impl Hash for Cave {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Ord for Cave {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Cave {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, Neighbours: [{}]", self.name, self.neighbours.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ") )
    }
}

struct Caves {
    caves: Vec<Cave>
}

impl fmt::Display for Caves {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Caves:\n   {}", self.caves.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n   "))
    }
}

impl Caves {
    fn new() -> Caves{
        Caves{caves: Vec::new()}
    }

    pub fn create_caves(lines: Vec<String>) -> Caves {
        let mut caves : Caves = Caves::new();
        for line in lines {
            let split: Vec<&str> = line.split('-').collect();
            assert_eq!(2, split.len());
            
            caves.add_neighbours(&split[0].to_string(), &split[1].to_string());
        }
        caves
    }

    fn add(&mut self, cave: &String) {
        if !self.caves.contains(&Cave::new(cave.clone())) {
            self.caves.push(Cave::new(cave.clone()));
            self.caves.sort();
        }
    }

    fn add_neighbours(&mut self, cave1: &String, cave2: &String) {
        self.add(cave1);
        self.add(cave2);
        if let Some(i) = self.caves.iter().position(|x| x.name == *cave1) {
            self.caves[i].add_neighbour(cave2.clone());
        }
        if let Some(i) = self.caves.iter().position(|x| x.name == *cave2) {
            self.caves[i].add_neighbour(cave1.clone());
        }
    }
}

pub fn part1(lines: Vec<String>) -> i128 {
    let caves = Caves::create_caves(lines);
    println!("{}", caves);
    1
}

pub fn part2() -> i128 {
    1
}

fn read_data() -> Vec<String> {
    let values: String = fs::read_to_string("data/day11.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).filter(|x| x.len() > 0).collect()
}

fn _read_example(num: i32) -> Vec<String> {
    let values: String = fs::read_to_string(format!("data/day12.example{}", num)).expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).filter(|x| x.len() > 0).collect()
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_day12part1_example1() {
        let result = part1(_read_example(1));
        assert_eq!(1585, result);
    }

    #[test]
    fn test_day12part1_example2() {
        let result = part1(_read_example(2));
        assert_eq!(1585, result);
    }

    #[test]
    fn test_day12part1_example3() {
        let result = part1(_read_example(3));
        assert_eq!(1585, result);
    }


    #[test]
    fn test_day12part2() {
        let result = part2();
        assert_eq!(382, result);
    }
}