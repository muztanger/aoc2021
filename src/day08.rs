use std::fs;

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
    1
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
        assert_eq!(86397080, part2());
    }
}