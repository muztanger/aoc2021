use std::fs;
use itertools::Itertools;

pub fn day2part1() -> i128 {
    let lines = read_data();
    
    let line_count = lines.len();
    
    let mut counting = Vec::new();
    for line in lines {
        for (i, col) in line.chars().enumerate() {
            if counting.len() <= i {
                counting.push(0);
            }
            if col == '1' {
                counting[i] += 1;
            }
        }
    }
    let mut gamma_rate = Vec::new();
    let mut epsilon_rate = Vec::new();
    println!("counting: {}", counting.iter().join(""));
    for x in counting {
        if x > line_count / 2{
            gamma_rate.push(1);
            epsilon_rate.push(0);
        } else {
            gamma_rate.push(0);
            epsilon_rate.push(1);
        }
    }
    let gamma_str = gamma_rate.iter().join("");
    let epsilon_str = epsilon_rate.iter().join("");
    println!("gamma rate: {}\nepsilon rate: {}", gamma_str, epsilon_str);
    let gamma = i128::from_str_radix(&gamma_str.to_string(), 2).unwrap();
    let epsilon = i128::from_str_radix(&epsilon_str.to_string(), 2).unwrap();
    println!("gamma={} epsilon={}", gamma, epsilon);
    gamma * epsilon
}

pub fn day2part2() -> i128 {
    let lines = read_data();
    
    let mut oxygen : Vec<String> = lines.clone().into_iter().filter(|x| x.len() > 0).collect();
    let mut index = 0;
    while oxygen.len() > 1 && index < oxygen.first().unwrap().len() {
        let zeros = oxygen.clone().into_iter().filter(|o| o.chars().nth(index).unwrap() == '0').count();
        let ones = oxygen.clone().into_iter().filter(|o| o.chars().nth(index).unwrap() == '1').count();
        
        // println!("zeros={} ones={}", zeros, ones);
        oxygen = oxygen.into_iter().filter(|o|
            (ones >= zeros && o.chars().nth(index).unwrap() == '1')
            || (ones < zeros && o.chars().nth(index).unwrap() == '0') ).collect();

        // println!("oxygen index={}:", index);
        // for x in oxygen.clone() {
        //     println!("   {}", x);
        // }
        index += 1;
    }

    let mut co2 : Vec<String> = lines.clone().into_iter().filter(|x| x.len() > 0).collect();
    let mut index = 0;
    while co2.len() > 1 && index < co2.first().unwrap().len() {
        let zeros = co2.clone().into_iter().filter(|o| o.chars().nth(index).unwrap() == '0').count();
        let ones = co2.clone().into_iter().filter(|o| o.chars().nth(index).unwrap() == '1').count();

        // println!("zeros={} ones={}", zeros, ones);
        co2 = co2.into_iter().filter(|o|
            (ones < zeros && o.chars().nth(index).unwrap() == '1')
            || (ones >= zeros && o.chars().nth(index).unwrap() == '0') ).collect();


        // println!("co2 index={}:", index);
        // for x in co2.clone() {
        //     println!("   {}", x);
        // }
        index += 1;
    }

    let oxygen = i128::from_str_radix(oxygen.first().unwrap(), 2).unwrap();
    let co2 = i128::from_str_radix(co2.first().unwrap(), 2).unwrap();

    oxygen * co2
}

fn read_data() -> Vec<String> {
    let values: String = fs::read_to_string("data/day03.txt").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

fn _read_example() -> Vec<String> {
    let values: String = fs::read_to_string("data/day03.example").expect("Could not read file");
    values.split('\n').filter_map(|s | s.parse::<String>().ok()).collect()
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_day2part1() {
        assert_eq!(3429254, day2part1());
    }

    #[test]
    fn test_day2part2() {
        assert_eq!(5410338, day2part2());
    }
}