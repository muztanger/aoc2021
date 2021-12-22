use itertools::Itertools;
use std::fs;
use std::collections::HashMap;

pub fn part1() -> i128 {
    let lines = read_data();
    let mut count = 0;
    for line in lines.iter().filter(|x| x.len() > 0) {
        let output = line.split("|").skip(1).collect::<Vec<&str>>().first().unwrap().trim();
        let output: Vec<&str> = output.split(" ").collect();
        //1, 4, 7, and 8
        //2, 4, 3, 7
        count += output.iter().filter(|x| x.len() == 2 || x.len() == 4 || x.len() == 3 || x.len() == 7).count();
    }
    count as i128
}


pub fn part2() -> i128 {
    let lines = read_data();
    let mut count = 0;
    for line in lines.iter().filter(|x| x.len() > 0) {
        //println!("Line: {}", &line);
        let line: Vec<&str> = line.split("|").collect::<Vec<&str>>();
        assert_eq!(2, line.len());
        let input: Vec<&str> = line[0].split_whitespace().collect();

        let mut candidates: Vec<Vec<char>> = Vec::new();

        let letters = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

        let mut lights: HashMap<char, Vec<usize>> = HashMap::new();
        lights.insert('0', vec![0, 1, 2, 4, 5, 6]);
        lights.insert('1', vec![2, 5]);
        lights.insert('2', vec![0, 2, 3, 4, 6]);
        lights.insert('3', vec![0, 2, 3, 5, 6]);
        lights.insert('4', vec![1, 2, 3, 5]);
        lights.insert('5', vec![0, 1, 3, 5, 6]);
        lights.insert('6', vec![0, 1, 3, 4, 5, 6]);
        lights.insert('7', vec![0, 2, 5]);
        lights.insert('8', vec![0, 1, 2, 3, 4, 5, 6]);
        lights.insert('9', vec![0, 1, 2, 3, 5]);

        for perm in letters.iter().permutations(letters.len()).unique() {
            let mut is_valid = true;
            'word: for word in &input {
                match word.len() {
                    // Digit 1
                    2 => {
                        for x in lights.get(&'1').unwrap() {
                            if word.chars().all(|c| *perm[*x] != c) {
                                is_valid = false;
                                break 'word;
                            }
                        }
                    },
                    // Digit 7
                    3 => {
                        for x in lights.get(&'7').unwrap() {
                            if word.chars().all(|c| *perm[*x] != c) {
                                is_valid = false;
                                break 'word;
                            }
                        }
                    },

                    // Digit 4
                    4 => {
                        for x in lights.get(&'4').unwrap() {
                            if word.chars().all(|c| *perm[*x] != c) {
                                is_valid = false;
                                break 'word;
                            }
                        }
                    },

                    // Digit 2, 3 or 5
                    5 => {
                        let mut found_valid = true;

                        // 2
                        for x in lights.get(&'2').unwrap() {
                            if word.chars().all(|c| *perm[*x] != c) {
                                found_valid = false;
                                break;
                            }
                        }

                        if !found_valid {
                            // 3
                            found_valid = true;
                            for x in lights.get(&'3').unwrap() {
                                if word.chars().all(|c| *perm[*x] != c) {
                                    found_valid = false;
                                    break;
                                }
                            }
                        }
                    
                        if !found_valid {
                            // 5
                            found_valid = true;
                            for x in lights.get(&'5').unwrap() {
                                if word.chars().all(|c| *perm[*x] != c) {
                                    found_valid = false;
                                    break;
                                }
                            }
                        }

                        if !found_valid {
                            is_valid = false;
                            break 'word;
                        }
                    }
    
    
                    _ => {}
                };
            }

            if is_valid {
                candidates.push(perm.iter().copied().map(|x| *x).collect())
            }
        }


        // for candidate in candidates {
        //     println!("{} -> {}",
        //     letters.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", "),
        //     candidate.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", "));
        // }
        assert_eq!(1, candidates.len());
        let code = candidates.first().unwrap();

        let output: Vec<&str> = line[1].split_whitespace().collect();
        let mut x: Vec<char> = Vec::new();
        for number in output {
                //  0000    aaaa
                // 1    2  b    c 
                // 1    2  b    c
                //  3333    dddd
                // 4    5  e    f
                // 4    5  e    f
                //  6666    gggg
                match number.len() {
                2 => {x.push('1');},
                3 => {x.push('7');},
                4 => {x.push('4');},
                5 => {
                    //2, 3 or 5
                    if number.chars().contains(&code[1]) {
                        x.push('5');
                    } else if number.chars().contains(&code[4]) {
                        x.push('2');
                    }
                    else {
                        x.push('3');
                    }
                },
                6 => {
                    //0, 6 or 9
                    if number.chars().contains(&code[3]) {
                        if number.chars().contains(&code[4]) {
                            x.push('6');
                        } else {
                            x.push('9');
                        }
                    } else {
                        x.push('0');
                    }
                },
                7 => {x.push('8');}
                _ => {
                    //fail!
                }
            }
        }
        let x: i128 = x.iter().map(|c| c.to_string()).collect::<Vec<String>>().concat().parse::<i128>().unwrap();
        // println!("x={}", x);

        //1, 4, 7, and 8
        //2, 4, 3, 7
        count += x;
    }
    count as i128
}

fn read_data() -> Vec<String> {
    let values: String = fs::read_to_string("data/day08.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

fn _read_example() -> Vec<String> {
    let values: String = fs::read_to_string("data/day08.example").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_day8part1() {
        assert_eq!(239, part1());
    }

    #[test]
    fn test_day8part2() {
        let result = part2();
        assert_ne!(944669, result); // is too low
        assert!(result > 944669);
        assert_eq!(946346, result);
    }
}