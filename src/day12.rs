use std::fmt;
use std::fs;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

struct Cave {
    name: String,
    neighbours: Vec<usize>
}

impl Cave {
    pub fn new(name: String) -> Cave {
        Cave {name: name, neighbours: Vec::new()}
    }

    pub fn add_neighbour(&mut self, other: usize) {
        if !self.neighbours.contains(&other) {
            self.neighbours.push(other);
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
        }
    }

    fn add_neighbours(&mut self, cave1: &String, cave2: &String) {
        self.add(cave1);
        let i1 = self.caves.iter().position(|x| x.name == *cave1).unwrap();

        self.add(cave2);
        let i2 = self.caves.iter().position(|x| x.name == *cave2).unwrap();

        self.caves[i1].add_neighbour(i2);
        self.caves[i2].add_neighbour(i1);
    }

    fn find_path(caves: &Vec<Cave>, paths: &mut Vec<Vec<usize>>, path: &Vec<usize>, index: usize, version: i32) {
        // println!("caves: {}, paths: {}, path: {}, index: {}",
        //     caves.len(),
        //     paths.len(),
        //     format!("[{}]",
        //     path.iter().map(|&x| caves[x].name.clone()).collect::<Vec<String>>().join(",")),
        //     caves[index].name.clone());

        if version == 1
            || caves[index].name.as_str() == "start"
            || caves[index].name.as_str() == "end" {

            if path.iter().filter(|&&x| x == index).count() >= 1 && caves[index].name.chars().nth(0).unwrap().is_lowercase() {
                // println!("End due to duplicate");
                return;
            }
        }

        if version == 2 && path.len() > 0 {
            let mut count_lowercase : HashMap<usize, usize> = HashMap::new();
            for p in  path.iter().filter(|&&i| caves[i].name.chars().nth(0).unwrap().is_lowercase()) {
                if let Some(x) = count_lowercase.get_mut(p) {
                    *x += 1;
                } else {
                    count_lowercase.insert(*p, 1);
                }
            }
            // println!("{:?}", count_lowercase);
            if *count_lowercase.values().max().unwrap() > 2 || count_lowercase.values().filter(|&&x| x >= 2).count() >= 2 {
                return;
            }
        }

        if caves[index].name == "end" {
            // println!("End due to 'end'");
            let mut result = path.clone();
            result.push(index);
            paths.push(result);

            // println!("caves: {}, paths: {}, path: {}, index: {}",
            //     caves.len(),
            //     paths.len(),
            //     format!("[{}]",
            //     path.iter().map(|&x| caves[x].name.clone()).collect::<Vec<String>>().join(",")),
            //     caves[index].name.clone());
            return;
        }

        let mut path = path.clone();
        path.push(index.clone());
        for other in &caves[index].neighbours {
            Caves::find_path(caves, paths, &path, other.clone(), version);
        }
    }

    fn find_paths(self, version: i32) -> Vec<Vec<usize>> {
        let mut paths: Vec<Vec<usize>> = Vec::new();
        
        let i = self.caves.iter().position(|x| x.name == "start").unwrap();
        Caves::find_path(&self.caves, &mut paths, &vec![], i, version);

        paths
    }
}

pub fn part1(lines: Vec<String>) -> i128 {
    let caves = Caves::create_caves(lines);
    println!("{}", caves);

    let paths = caves.find_paths(1);
    paths.len() as i128
}

pub fn part2(lines: Vec<String>) -> i128 {
    let caves = Caves::create_caves(lines);
    println!("{}", caves);

    let paths = caves.find_paths(2);
    paths.len() as i128
}

fn read_data() -> Vec<String> {
    let values: String = fs::read_to_string("data/day12.txt").expect("Could not read file");
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
        assert_eq!(10, result);
    }

    #[test]
    fn test_day12part1_example2() {
        let result = part1(_read_example(2));
        assert_eq!(19, result);
    }

    #[test]
    fn test_day12part1_example3() {
        let result = part1(_read_example(3));
        assert_eq!(226, result);
    }

    #[test]
    fn test_day12part1() {
        let result = part1(read_data());
        assert_eq!(4573, result);
    }

    #[test]
    fn test_day12part2_example1() {
        let result = part2(_read_example(1));
        assert_eq!(36, result);
    }

    #[test]
    fn test_day12part2() {
        let result = part2(read_data());
        assert_eq!(117509, result);
    }
}