use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use itertools::Itertools;


fn first_part() -> i64 {
        // Get input file and open it
        let input = File::open(get_input()).unwrap();
        let reader = BufReader::new(input);
        let mut calculator: HashMap<i64, Vec<i64>> = HashMap::new();

        for line in reader.lines() {
            let curline = line.unwrap();
            let parts: Vec<&str> = curline.split(":").collect();

            let key: i64 = parts[0].trim().parse().unwrap();
            let values: Vec<i64> = parts[1]
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            calculator.insert(key, values);
        }
        GetValueWithParameter::get_value_with_input(calculator).try_into().unwrap()
}

struct GetValueWithParameter;
impl GetValueWithParameter {
    pub fn get_value_with_input(calculator: HashMap<i64, Vec<i64>>) -> i64 {
        let mut sum: i64 = 0;

        for (key, values) in calculator.iter() {
            let cur_length = values.len() - 1;

            // Generate all possible combinations of operators
            let combinations: Vec<String> = (0..(1 << cur_length))
                .map(|i| {
                    (0..cur_length)
                        .map(|j| if i & (1 << (cur_length - j - 1)) != 0 { '+' } else { '*' }).collect()
                }).collect();

            // Calculate all possible results
            let results: Vec<i64> = combinations.iter().map(|possibility| {
                let mut result = values[0];

                for (i, operator) in possibility.chars().enumerate() {
                    match operator {
                        '+' => result += values[i + 1],
                        '*' => result *= values[i + 1],
                        _ => (),
                    }
                }
                result
            }).collect();

            // Check if the results contain the sumvalue needed
            if results.contains(&key) {
                sum += key;
            }
        }
        sum
    }
}



fn second_part() -> i64 {
        // Get input file and open it
        let input = File::open(get_input()).unwrap();
        let reader = BufReader::new(input);
        let mut calculator: HashMap<i64, Vec<i64>> = HashMap::new();

        for line in reader.lines() {
            let curline = line.unwrap();
            let parts: Vec<&str> = curline.split(":").collect();

            let key: i64 = parts[0].trim().parse().unwrap();
            let values: Vec<i64> = parts[1]
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            calculator.insert(key, values);
        }
        GetValueWithParameterSecond::get_value_with_input_second(calculator).try_into().unwrap()
}

struct GetValueWithParameterSecond;
impl GetValueWithParameterSecond {
    pub fn get_value_with_input_second(calculator: HashMap<i64, Vec<i64>>) -> i64 {
        let mut sum: i64 = 0;
        let mut combination_map: HashMap<i64, Vec<String>> = HashMap::new();

        for (key, values) in calculator.iter() {
            let cur_length = values.len() - 1;
            let mut combinations: Vec<String> = Vec::new();

            // Keep possible combinations in memory
            if combination_map.contains_key(&cur_length.try_into().unwrap()) {
                combinations = combination_map.get(&cur_length.try_into().unwrap()).unwrap().to_vec();
            }   else {

            // Generate all possible combinations of operators with Itertools (Cartesian Product)
                combinations = (0..cur_length)
                    .map(|_| ['+', '*', '|'].iter())
                    .multi_cartesian_product()
                    .map(|v| v.into_iter().collect())
                    .collect();
                combination_map.insert(cur_length.try_into().unwrap(), combinations.clone());
            }

            //println!("Combinations done");
            // Calculate all possible results
            let results: Vec<i64> = combinations.iter().map(|possibility| {
                let mut result = values[0];

                for (i, operator) in possibility.chars().enumerate() {
                    match operator {
                        '+' => result += values[i + 1],
                        '*' => result *= values[i + 1],
                        '|' => result = format!("{}{}", result, values[i+1]).parse().unwrap(),
                        _ => (),
                    }
                }
                result
            }).collect();

            // Check if the results contain the sumvalue needed
            if results.contains(&key) {
                sum += key;
            }
        }
        sum
    }
}

fn main() {
    println!("First Part Answer: {}", first_part());
    println!("Second Part Answer: {}", second_part());
}

// Get input file
fn get_input() -> &'static str {
    match env::var("FILE") {
        Ok(value) => {
            if value == "test" || value == "true" {
                "testcase.txt"
            } else {
                "input.txt"
            }
        }
        Err(_) => "input.txt",
    }

}