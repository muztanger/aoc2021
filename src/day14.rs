use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn get_next(template: &String, rules: &HashMap<String, String>) -> String {
    let mut next: Vec<String> = vec![template.chars().take(1).collect::<String>()];
    for r in template.chars().zip(template.chars().skip(1)).map(
        |(a, b)| format!("{}{}", rules.get(&format!("{}{}", a, b)).unwrap(), b)) {
            next.push(r);
    }
    next.concat()
}

struct PolymerCounter {
    rules: HashMap<String, String>,
    mem: HashMap<(String, i128), HashMap<char, i128>>
}

impl PolymerCounter {
    pub fn count(&mut self, template: &String, n: i128) -> HashMap<char, i128> {
        println!("Template: {} n: {}", template, n);
        if self.mem.contains_key(&(template.clone(), n)) {
            return self.mem.get(&(template.clone(), n)).unwrap().clone();
        }

        if n == 0 {
            let mut result: HashMap<char, i128> = HashMap::new();
            for c in template.chars() {
                if !result.contains_key(&c) {
                    result.insert(c, 1);
                } else {
                    if let Some(x) = result.get_mut(&c) {
                        *x += 1;
                    }
                }
            }
            self.mem.insert((template.clone(), n), result.clone());
            println!("Template: {} n: {} Result: {:?}", template, n, result);
            return result;
        }

        match template.len() {
            0 => HashMap::new(),
            1 => {
                let mut result = HashMap::new();
                result.insert(template.chars().next().unwrap(), 1);
                println!("Template: {} n: {} Result: {:?}", template, n, result);
                return result;
            },
            2 => {
                // if n == 0 {
                //     let mut result = HashMap::new();
                //     result.insert(template.chars().nth(0).unwrap(), 1);
                //     result.insert(template.chars().nth(1).unwrap(), 1);
                //     self.mem.insert((template.clone(), n), result.clone());
                //     println!("Template: {} n: {} Result: {:?}", template, n, result);
                //     return result;
                // } else {
                    let result = self.count(&format!("{}{}{}", template.chars().nth(0).unwrap(), self.rules.get(template).unwrap(), template.chars().nth(1).unwrap()), n - 1);
                    println!("Template: {} n: {} Result: {:?}", template, n, result);
                    return result;
                // }
            },
            _ => {
                let mut result = self.count(&template.chars().take(2).collect::<String>(), n);

                let rest = self.count(&template.chars().skip(1).collect::<String>(), n);
                for k in rest.keys() {
                    if result.contains_key(k) {
                        let x = result.get_mut(k).unwrap();
                        *x += rest.get(&k).unwrap();
                    } else {
                        result.insert(*k, *rest.get(&k).unwrap());
                    }
                }

                let c = template.chars().nth(1).unwrap();
                let x = result.get_mut(&c).unwrap();
                *x -= 1;

                self.mem.insert((template.clone(), n), result.clone());

                println!("Template: {} n: {} Result: {:?}", template, n, result);
                return result;
            }
        }
    }
}

pub fn problem(file: String, n: usize) -> i128 {
    let lines: Vec<String> = fs::read_to_string(file).expect("Could not read file").split('\n').filter_map(|s | s.parse::<String>().ok()).collect();
    let polymer_template = lines.first().unwrap();
    println!("Template: {}", polymer_template);

    let re = Regex::new(r"\s*(\w+)\s*->\s*(\w+)").unwrap();
    let mut rules: HashMap<String, String> = HashMap::new(); 
    for cap in lines.iter().skip(2).filter_map(|line| re.captures(line.as_str())) {
        rules.insert(
            cap.get(1).unwrap().as_str().to_string(),
            cap.get(2).unwrap().as_str().to_string());
    }

    println!("Rules: {:?}", rules);
    let mut counter = PolymerCounter{rules: rules, mem: HashMap::new()};
    let count: HashMap<char, i128> = counter.count(polymer_template, n as i128);

    // let mut next = polymer_template.clone();
    // for _ in 0..n {
    //     next = get_next(&next, &rules);
    //     //println!("Next: {:?}", next);
    // }

    // let mut count: HashMap<char, i128> = HashMap::new();
    // for c in next.chars() {
    //     if !count.contains_key(&c) {
    //         count.insert(c, 1);
    //     } else {
    //         if let Some(x) = count.get_mut(&c) {
    //             *x += 1;
    //         }
    //     }
    // }

    println!("{:?}", count);

    (count.values().max().unwrap() - count.values().min().unwrap()) as i128
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_day14_part1_example() {
        let result = problem("data/day14.example".to_string(), 10);
        assert_eq!(1588, result);
    }

    #[test]
    fn test_day14_part1() {
        let result = problem("data/day14.txt".to_string(), 10);
        assert_eq!(2915, result);
    }

    #[test]
    fn test_day14_part2() {
        assert_eq!(3353146900153, problem("data/day14.txt".to_string(), 40));
    }
}