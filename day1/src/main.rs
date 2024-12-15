use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn first_part() -> i32 {
    // Get input file and open it
    let input = File::open(get_input()).unwrap();
    let reader = BufReader::new(input);
    
    // Create list vectors
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    // Read lines from file and add to list vectors
    for line in reader.lines() {
        let curline = line.unwrap();

        // Split line by space and add to lists
        let numbers: Vec<&str> = curline.split(" ").collect();
        let list1number: u32 = numbers.first().unwrap().parse().unwrap();
        let list2number: u32 = numbers.last().unwrap().parse().unwrap();

        // Add to lists as i32
        list1.push(list1number.try_into().unwrap());
        list2.push(list2number.try_into().unwrap());
    }
    // Sort lists
    list1.sort();
    list2.sort();

    // Return distance with Distancer struct
    Distancer::get_distance(list1, list2)
}

// Struct for first_part
struct Distancer;
impl Distancer {
    // Get distance between two lists
    pub fn get_distance(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
        let mut distance = 0;
        for (index, value) in list1.iter().enumerate() {
            distance += (value - list2[index]).abs();
        }
        distance
    }
}



fn second_part() -> i32 {
    // Get input file and open it
    let input = File::open(get_input()).unwrap();
    let reader = BufReader::new(input);
    
    // Create list vectors
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    // Read lines from file and add to list vectors
    for line in reader.lines() {
        let curline = line.unwrap();

        // Split line by space and add to lists
        let numbers: Vec<&str> = curline.split(" ").collect();
        let list1number: u32 = numbers.first().unwrap().parse().unwrap();
        let list2number: u32 = numbers.last().unwrap().parse().unwrap();

        // Add to lists as i32
        list1.push(list1number.try_into().unwrap());
        list2.push(list2number.try_into().unwrap());
    }

    // Return distance with Similarity struct
    Similarity::get_distance(list1, list2)
}

// Struct for second_part
struct Similarity;
impl Similarity {
    pub fn get_distance(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
        let mut similarity_value = 0;
        for value in list1 {
            for value2 in &list2 {
                if value == *value2 {
                    similarity_value += value;
                }
            }
        }
        similarity_value
    }
}


// Run with $env:FILE="test"/"input"; cargo run
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

