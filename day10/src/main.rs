use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn first_part() -> i32 {
    let input = File::open(get_input()).unwrap();
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    let mut mountainvector: Vec<Vec<char>> = Vec::with_capacity(lines.len());

    lines.iter().for_each(|l| {
        let curline: Vec<char> = Vec::from_iter(l.chars());
        mountainvector.push(curline);
    });

    // debug print for grid
    //for line in &mountainvector {
    //    println!("{:?}", line);
    //}
    
    MountainRange::get_paths(mountainvector)
}

struct MountainRange;
impl MountainRange {
    pub fn get_paths(mountain_puzzle: Vec<Vec<char>>) -> i32 {

        // Get the peak location "9"
        let mut peak_locations: Vec<Vec<i32>> = Vec::new();
        let mut true_paths: i32 = 0;        

        for (y, line) in mountain_puzzle.iter().enumerate() {
            for (x, _row) in line.iter().enumerate() {
                if mountain_puzzle[y][x] == '9' {
                    peak_locations.push(vec![x as i32, y as i32]);
                }
            }
        }


        for peak in peak_locations {
            println!("{:?}", peak); // [x, y]
            // make vector with 
            let current_true_paths = true_paths.clone();
            MountainRange::get_path(&mountain_puzzle, peak[0], peak[1], 9, &mut true_paths);
            println!("True Paths: {}", true_paths - current_true_paths);
        }

        true_paths
    }


    pub fn get_path(mountain_puzzle: &Vec<Vec<char>>, x: i32, y: i32, i: i32,  true_paths: &mut i32) -> i32 {
        let grid_length = mountain_puzzle.len() as i32;

        for j in (0..i).rev() {
            let mut paths: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
            let mut found_path = false;

            // Going left
            if x - 1 >= 0  && mountain_puzzle[y as usize][x as usize - 1] == j.to_string().chars().next().unwrap() {
                if j == 0 {
                    *true_paths += 1;
                    println!("Found 0: ({}, {})", x - 1, y);
                }   else {
                    paths.entry(j).or_insert_with(Vec::new).push(vec![x - 1, y]);
                }
                found_path = true;
            }

            // Going right
            if x + 1 < grid_length && mountain_puzzle[y as usize][x as usize + 1] == j.to_string().chars().next().unwrap() {
                if j == 0 {
                    *true_paths += 1;
                    println!("Found 0: ({}, {})", x + 1, y);
                }   else {
                    paths.entry(j).or_insert_with(Vec::new).push(vec![x + 1, y]);
                }
                found_path = true;
            }

            // Going down
            if y + 1 < grid_length && mountain_puzzle[y as usize + 1][x as usize] == j.to_string().chars().next().unwrap() {
                if j == 0 {
                    *true_paths += 1;
                    println!("Found 0: ({}, {})", x, y + 1);
                }   else {
                    paths.entry(j).or_insert_with(Vec::new).push(vec![x, y + 1]);
                }
                found_path = true;
            }

            // Going up
            if y - 1 >= 0 && mountain_puzzle[y as usize - 1][x as usize] == j.to_string().chars().next().unwrap() {
                if j == 0 {
                    *true_paths += 1;
                    println!("Found 0: ({}, {})", x, y - 1);
                }   else {
                    paths.entry(j).or_insert_with(Vec::new).push(vec![x, y - 1]);
                }
                found_path = true;
            }


            if j == 0 {
                break;
            }

            // j will be next i
            if let Some(locations_clone) = paths.get(&j) {
                if locations_clone.len() >= 1 {
                    for location in locations_clone {
                        MountainRange::get_path(mountain_puzzle, location[0], location[1], j, true_paths);
                    }
                }
            }

            if !found_path {
                break;
            }


        }
        0
    }
}



fn main() {
    println!("First Part Answer: {}", first_part());
    //println!("Second Part Answer: {}", second_part());
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
