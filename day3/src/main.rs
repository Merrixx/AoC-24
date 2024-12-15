use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn first_part() -> i32 {
    // Get input file and open it
    let input = File::open(get_input()).unwrap();
    let reader = BufReader::new(input);
    let curline: String = reader.lines().next().unwrap().unwrap();

    // Filter all mul() and return them
    let filtered = filter_mul_one(curline.as_str());
    ReportSafety::get_anwser(filtered)
}

fn filter_mul_one(input: &str) -> Vec<String> {
    let re = Regex::new(r"mul\([^a-zA-Z\s\-\+)]*,[^a-zA-Z\s\-\+)]*\)").unwrap();
    re.find_iter(&input).map(|mat| mat.as_str().to_string()).collect()
}

struct ReportSafety;
impl ReportSafety {
    pub fn get_anwser(input_vector: Vec<String>) -> i32 {
        let mut sum = 0;
        // Get the two numbers from the string and multiply them
        for input in input_vector {
            let temp: Vec<i32> = input[4..input.len()-1] 
            .split(',') 
            .filter_map(|x| x.parse::<i32>().ok()) 
            .collect(); 

        if temp.len() == 2 {
            sum += temp[0] * temp[1];
        }
        }
        sum
    }
}



fn second_part() -> i32 {
    // Get input file and open it
    let input = File::open(get_input()).unwrap();
    let reader = BufReader::new(input);
    let curline: String = reader.lines().next().unwrap().unwrap();
    let filtered = filter_mul_second(curline.as_str());
    ReportSafetySecond::get_anwser_second(filtered)
}

fn filter_mul_second(input: &str) -> Vec<String> {
    // Remove everything between don't() and do()
    let re_remove = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let cleaned_input = re_remove.replace_all(input, "").to_string();

    // Find all mul() and return them
    let re = Regex::new(r"mul\([^a-zA-Z\s\-\+)]*,[^a-zA-Z\s\-\+)]*\)").unwrap();
    re.find_iter(&cleaned_input).map(|mat| mat.as_str().to_string()).collect()
}

struct ReportSafetySecond;
impl ReportSafetySecond {
    pub fn get_anwser_second(input_vector: Vec<String>) -> i32 {
        let mut sum = 0;
        // Get the two numbers from the string and multiply them
        for input in input_vector {
            let temp: Vec<i32> = input[4..input.len()-1]
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            sum += temp[0] * temp[1];
        }
        return sum;
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