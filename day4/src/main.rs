use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};


fn first_part() -> i32 {
    // Get input file and open it
    let input = File::open(get_input()).unwrap();
    let reader = BufReader::new(input);
    let linebreak = get_linebreak();

    // Read lines from file and add to list vectors
    let combined: String = reader.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("");

    let wordtofind = "XMAS";
    let puzzelvector = convert_string_to_2d_vector(combined.as_str(), linebreak);
    WordMatcher::find_word(puzzelvector, wordtofind, linebreak)
}

struct WordMatcher;
impl WordMatcher {
    pub fn find_word(puzzle: Vec<Vec<char>>, word: &str, linebreak: i32) -> i32 {
        let mut count = 0;
        let reversed_word = word.chars().rev().collect::<String>();
        let mut i = 0;
        
        //Horizontal
        for vec in &puzzle {
            let mut row = String::new();
            for char in vec {
                row.push(*char);
            }
            count += row.matches(word).count();
            count += row.matches(&reversed_word).count();
        }
    
        //Vertical
        while i != (linebreak) {
            let mut column = String::new();
            for vec in &puzzle {
                column.push(vec[i as usize]);
            }
            count += column.matches(word).count();
            count += column.matches(&reversed_word).count();
            i += 1;
        }

        //Diagonal
        // Left top to right bottom
        for start in 0..linebreak {
            let mut diag = String::new();
            let mut row = 0;
            let mut col = start;
            while row < linebreak && col < linebreak {
                diag.push(puzzle[row as usize][col as usize]);
                row += 1;
                col += 1;
            }
            count += diag.matches(word).count();
            count += diag.matches(&reversed_word).count();
        }
        for start in 1..linebreak {
            let mut diag = String::new();
            let mut row = start;
            let mut col = 0;
            while row < linebreak && col < linebreak {
                diag.push(puzzle[row as usize][col as usize]);
                row += 1;
                col += 1;
            }
            count += diag.matches(word).count();
            count += diag.matches(&reversed_word).count();
        }

        // Right top to left bottom
        for start in 0..linebreak {
            let mut diag = String::new();
            let mut row = start;
            let mut col = 0;
            while row >= 0 && col < linebreak {
                diag.push(puzzle[row as usize][col as usize]);
                if row == 0 {
                    break;
                }
                row -= 1;
                col += 1;
            }
            count += diag.matches(word).count();
            count += diag.matches(&reversed_word).count();
        }
        for start in 1..linebreak {
            let mut diag = String::new();
            let mut row = linebreak - 1;
            let mut col = start;
            while row >= 0 && col < linebreak {
                diag.push(puzzle[row as usize][col as usize]);
                if row == 0 {
                    break;
                }
                row -= 1;
                col += 1;
            }
            count += diag.matches(word).count();
            count += diag.matches(&reversed_word).count();
        }

        count.try_into().unwrap()
    }
}



fn second_part() -> i32 {
    // Get input file and open it
    let input = File::open(get_input()).unwrap();
    let reader = BufReader::new(input);
    let linebreak = get_linebreak();

    // Read lines from file and add to list vectors
    let combined: String = reader.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("");

    let puzzelvector = convert_string_to_2d_vector(combined.as_str(), linebreak);
    FindMas::find_mark(puzzelvector, linebreak)
}

struct FindMas;
impl FindMas {
    pub fn find_mark(puzzle: Vec<Vec<char>>, linebreak: i32) -> i32 {
        let mut count = 0;
        let mut curvec = 0;
        for vec in &puzzle {
            let mut curchar = 0;
            for char in vec {

                // Limit the search to the inner part of the puzzle
                if curvec == 0 || curvec == (linebreak - 1) || curchar == 0 || curchar == (linebreak - 1){
                    curchar += 1;
                    continue;
                }
                
                // Create M and S counters 
                let mut mchars = 0;
                let mut schars = 0;

                // Check if the current char is A and if the surrounding chars are M or S with valid positons
                if char == &'A' {
                    if &puzzle[(curvec - 1) as usize][(curchar - 1) as usize] == &'M' {
                        mchars += 1;
                    } else if &puzzle[(curvec - 1) as usize][(curchar - 1) as usize] == &'S' {
                        schars += 1;
                    }
                    if &puzzle[(curvec + 1) as usize][(curchar + 1) as usize] == &'M' {
                        mchars += 1;
                    } else if &puzzle[(curvec + 1) as usize][(curchar + 1) as usize] == &'S' {
                        schars += 1;
                    }
                    if mchars != 2 && schars != 2 {

                        if &puzzle[(curvec - 1) as usize][(curchar + 1) as usize] == &'M' {
                            mchars += 1;
                        } else if &puzzle[(curvec - 1) as usize][(curchar + 1) as usize] == &'S' {
                            schars += 1;
                        }
                        if &puzzle[(curvec + 1) as usize][(curchar - 1) as usize] == &'M' {
                            mchars += 1;
                        } else if &puzzle[(curvec + 1) as usize][(curchar - 1) as usize] == &'S' {
                            schars += 1;
                        }

                        if mchars == 2 && schars == 2 {
                            count += 1;
                        }
                    }

                }
                curchar += 1;
            }
            curvec += 1;
        }
        count
    }
}



// Convert string to 2d vector, used in both parts
fn convert_string_to_2d_vector(input: &str, linebreak: i32) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let mut row = Vec::new();
    for c in input.chars() {
        row.push(c);
        if row.len() == linebreak.try_into().unwrap() {
            result.push(row);
            row = Vec::new();
        }
    }
    result
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

fn get_linebreak() -> i32 {
    match env::var("FILE") {
        Ok(value) => {
            if value == "test" || value == "true" {
                10
            } else {
                140
            }
        }
        Err(_) => 140,
    }
}