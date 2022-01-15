use std::fs;
use std::fmt;
use std::cmp;

struct Packet {
    version: i128, //3 bits
    type_id: i128, //3 bits: 4 = literal value, otherwise operator
    number: i128, //5 * n bits: 1 bit (is last) and then 4 bits part of the number. TODO does this belong in every packet?

    packets: Vec<Packet>,
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        let mut result = String::new();
        result += &format!("version={}, ", self.version);
        result += &format!("type_id={}, ", self.type_id);
        if self.type_id == 4 {
            result += &format!("number={}", self.number);
        } else {
            result += "packets=[";
            for (i, p) in self.packets.iter().enumerate() {
                if i != 0 {
                    result += ", ";
                }
                result += &format!("{}", p.to_string());
            }
            result += "]";
        }
        write!(f, "Packet({})", result)
    }
}

impl Packet {

    fn parse_packet(data: &String, index: &mut usize) -> Packet {
        let mut result = Packet{version:0, type_id:0, number:0, packets: Vec::new()};

        result.version = i128::from_str_radix(&data[*index..*index+3], 2).unwrap();
        *index += 3;
        // println!("version: {}", result.version);
        
        result.type_id = i128::from_str_radix(&data[*index..*index+3], 2).unwrap();
        *index += 3;
        // println!("type_id: {}", result.type_id);
        
        match result.type_id {
            4 => { // is literal value
                
                let mut value: i128 = 0;
                while &data[*index..*index+1] == "1" {
                    *index += 1;

                    value = value.checked_shl(4).unwrap();
                    value = value | i128::from_str_radix(&data[*index..*index+4], 2).unwrap();
                    *index += 4;
                }

                assert_eq!(&data[*index..*index+1], "0");
                *index += 1;
                
                value = value.checked_shl(4).unwrap();
                value = value | i128::from_str_radix(&data[*index..*index+4], 2).unwrap();
                *index += 4;

                result.number = value;
                // println!("Literal value: {}", result.number);
            },
            _ => {
                // is operator
                let length_type_id = &data[*index..*index+1];
                *index += 1;
                // println!("length_type_id={}", length_type_id);

                match length_type_id {
                    "0" => {
                        // the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
                        let mut total_length = usize::from_str_radix(&data[*index..*index+15], 2).unwrap();
                        *index += 15;
                        while total_length > 0 {
                            let start_index = *index;
                            let sub_packet = Self::parse_packet(data, index);
                            total_length -= *index - start_index;
                            result.packets.push(sub_packet);
                        }

                        // println!("total_length={}", total_length);
                    },
                    "1" => {
                        // the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet.
                        let num_sub_packets = i32::from_str_radix(&data[*index..*index+11], 2).unwrap();
                        *index += 11;

                        for _ in 0..num_sub_packets {
                            let sub_packet = Self::parse_packet(data, index);
                            result.packets.push(sub_packet);
                        }

                        println!("num_sub_packets={}", num_sub_packets);
                    }
                    _ => {}
                }

            }
        }

        result
    }

    fn parse(line: &String) -> Packet {
        let line_as_u8 = line.as_bytes().iter().filter_map(|b| {
            match b {
                b'0'..=b'9' => Some(b - b'0'),
                b'a'..=b'f' => Some(b - b'a' + 10),
                b'A'..=b'F' => Some(b - b'A' + 10),
                _ => None,
            }
        });

        let data = line_as_u8.fuse().map(|d| format!("{:04b}", d)).collect::<Vec<String>>().concat(); 
        let mut index = 0;
        let result = Self::parse_packet(&data, &mut index);
        println!("result={}", result.to_string());
        result
    }

    fn version_sum(&self) -> i128 {
        let mut result = self.version;
        for p in &self.packets {
            result += p.version_sum();
        }
        result
    }

    fn eval(&self) -> i128 {
        match self.type_id {
            0 => { // sum
                let mut result = 0;
                for p in &self.packets {
                    result += p.eval();
                }
                result
            },
            1 => { // product
                let mut result = 1;
                for p in &self.packets {
                    result *= p.eval();
                }
                result
            },
            2 => {
                let mut result = i128::MAX;
                for p in &self.packets {
                    result = cmp::min(result, p.eval());
                }
                result
            },
            3 => {
                let mut result = i128::MIN;
                for p in &self.packets {
                    result = cmp::max(result, p.eval());
                }
                result
            },
            4 => {
                self.number
            },
            5 => {
                assert_eq!(2, self.packets.len());
                if self.packets[0].eval() > self.packets[1].eval() {
                    1
                } else {
                    0
                }
            },
            6 => {
                assert_eq!(2, self.packets.len());
                if self.packets[0].eval() < self.packets[1].eval() {
                    1
                } else {
                    0
                }
            },
            7 => {
                assert_eq!(2, self.packets.len());
                if self.packets[0].eval() == self.packets[1].eval() {
                    1
                } else {
                    0
                }
            },
            _ => {return -1;}
        }
    }
}

pub fn problem(file: String, n: usize) -> i128 {
    let lines: Vec<String> = fs::read_to_string(file).expect("Could not read file").split('\n').filter_map(|s | s.parse::<String>().ok()).collect();
    let decoder = Packet::parse(lines.first().unwrap());
    if n == 1 {
        decoder.version_sum()
    } else {
        decoder.eval()
    }
}

#[cfg(debug_assertions)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = problem("data/day16.example1".to_string(), 1);
        assert_eq!(6, result);
    }

    #[test]
    fn test_part1_example2() {
        let result = problem("data/day16.example2".to_string(), 1);
        assert_eq!(40, result);
    }

    #[test]
    fn test_part1_example3() {
        let result = problem("data/day16.example3".to_string(), 1);
        assert_eq!(40, result);
    }

    #[test]
    fn test_part1_txt() {
        let result = problem("data/day16.txt".to_string(), 1);
        assert_eq!(879, result);
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(3, problem("data/day16_2.example1".to_string(), 2));
        assert_eq!(54, problem("data/day16_2.example2".to_string(), 2));
        assert_eq!(7, problem("data/day16_2.example3".to_string(), 2));
        assert_eq!(9, problem("data/day16_2.example4".to_string(), 2));
        assert_eq!(1, problem("data/day16_2.example5".to_string(), 2));
        assert_eq!(0, problem("data/day16_2.example6".to_string(), 2));
        assert_eq!(0, problem("data/day16_2.example7".to_string(), 2));
        assert_eq!(1, problem("data/day16_2.example8".to_string(), 2));
    }

    #[test]
    fn test_part2_txt() {
        let result = problem("data/day16.txt".to_string(), 2);
        assert_eq!(539051801941, result);
    }
}