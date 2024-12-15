use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;


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


        let puzzelvector = convert_string_to_guardfield(combined.as_str(), linebreak);
        GuardPatrol::guard_path(puzzelvector, linebreak)
}

// Struct for first_part
struct GuardPatrol;
impl GuardPatrol {
    pub fn guard_path(puzzle: Vec<Vec<char>>, linebreak: i32) -> i32 {
        let mut guardposition: Vec<usize> = Vec::new();
        let mut direction: char = ' ';

        //Search For Guard and Direction
        for (index, vec) in puzzle.iter().enumerate() {
            let mut row = String::new();
            for (i, char) in vec.iter().enumerate() {
                (guardposition, direction) = match char {
                    '^' => (vec!(index as usize, i as usize), '^'),
                    'v' => (vec!(index as usize, i as usize), 'v'),
                    '>' => (vec!(index as usize, i as usize), '>'),
                    '<' => (vec!(index as usize, i as usize), '<'),
                    _ => continue,
                };
                row.push(*char);
            }

        }

        //Move Guard
        let guard_moved = GuardPatrol::move_guard(guardposition, direction, puzzle, linebreak);
        guard_moved


    }

    fn move_guard(mut guardposition: Vec<usize>, mut direction: char, mut puzzle: Vec<Vec<char>>, linebreak: i32) -> i32 {
        let mut tiles_visited = 0;
        loop {
             // Check if guard is at the edge of the field
            if guardposition[0] == 0 ||
               guardposition[0] == (linebreak - 1).try_into().unwrap() ||
               guardposition[1] == 0 ||
               guardposition[1] == (linebreak - 1).try_into().unwrap() {
                break;
            }
            match direction {
                // Move Guard Upwards
                '^' => {
                    if puzzle[guardposition[0] - 1][guardposition[1]] == '#' {
                        direction = '>';
                        continue;
                    }
                    if puzzle[guardposition[0]][guardposition[1]] != 'X'{
                        tiles_visited += 1;
                    }
                    puzzle[guardposition[0]][guardposition[1]] = 'X';
                    guardposition[0] -= 1;
                }
                // Move Guard Right
                '>' => {
                    if puzzle[guardposition[0]][guardposition[1] + 1] == '#' {
                        direction = 'v';
                        continue;
                    }
                    if puzzle[guardposition[0]][guardposition[1]] != 'X'{
                        tiles_visited += 1;
                    }
                    puzzle[guardposition[0]][guardposition[1]] = 'X';
                    guardposition[1] += 1;
                }
                // Move Guard Down
                'v' => {
                    if puzzle[guardposition[0] + 1][guardposition[1]] == '#' {
                        direction = '<';
                        continue;
                    }
                    if puzzle[guardposition[0]][guardposition[1]] != 'X'{
                        tiles_visited += 1;
                    }
                    puzzle[guardposition[0]][guardposition[1]] = 'X';
                    guardposition[0] += 1;
                }
                // Move Guard Left
                '<' => {
                    if puzzle[guardposition[0]][guardposition[1] - 1] == '#' {
                        direction = '^';
                        continue;
                    }
                    if puzzle[guardposition[0]][guardposition[1]] != 'X'{
                        tiles_visited += 1;
                    }
                    puzzle[guardposition[0]][guardposition[1]] = 'X';
                    guardposition[1] -= 1;
                }
                _ => continue,
            }
        }
        // Add 1 since the guard left the field
        tiles_visited += 1;
        tiles_visited
    }

}





fn second_part() -> i32 {
    // Get input file and open it
    let input = File::open(get_input()).unwrap();
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();

    let mut puzzelvector: Vec<Vec<char>> = Vec::with_capacity(lines.len());

    lines.iter().for_each(|l| {
        let curline: Vec<char> = Vec::from_iter(l.chars());
        puzzelvector.push(curline);
    });

    GuardPatrolSecond::guard_path_second(puzzelvector)
}

struct GuardPatrolSecond;
impl GuardPatrolSecond {
    pub fn guard_path_second(puzzle: Vec<Vec<char>>) -> i32 {
        let mut guardposition: Vec<usize> = Vec::new();
        let mut direction: char = ' ';


        //Search For Guard and Direction
        for (index, vec) in puzzle.iter().enumerate() {
            let mut row = String::new();
            for (i, char) in vec.iter().enumerate() {
                (guardposition, direction) = match char {
                    '^' => (vec!(index as usize, i as usize), '^'),
                    'v' => (vec!(index as usize, i as usize), 'v'),
                    '>' => (vec!(index as usize, i as usize), '>'),
                    '<' => (vec!(index as usize, i as usize), '<'),
                    _ => continue,
                };
                row.push(*char);
            }
        }

        //Move Guard
        let guard_stuck = GuardPatrolSecond::move_guard_second(guardposition, direction, puzzle);
        guard_stuck.try_into().unwrap()
    }

    // Find out if the guard is stuck with brute force approach
    fn move_guard_second(guardposition: Vec<usize>, direction: char, mut puzzle: Vec<Vec<char>>) -> usize {
        let mut counter: usize = 0;

        for i in 0..puzzle[0].len() {
            //println!("Went into row {:?}", i);
            for j in 0..puzzle[0].len() {

                let original_puzzlestate = puzzle[i][j];
                puzzle[i][j] = '#';
                //println!("# Placed at: {}, {}", i, j);
        
                let mut seen_before: HashMap<Vec<usize>, char> = HashMap::new();
                let mut stuck = false;
                let mut safety_stop = 0;
    
                // Reset guard position and direction for each iteration
                let mut current_guardposition = guardposition.clone();
                let mut current_direction = direction.clone();
    


                loop  {
                    // Check if guard is at the edge of the field
                    if current_guardposition[0] == 0 ||
                    current_guardposition[0] == (puzzle.len() - 1).try_into().unwrap() ||
                    current_guardposition[1] == 0 ||
                    current_guardposition[1] == (puzzle[0].len() - 1).try_into().unwrap() {
                        break;
                    }

                    if seen_before.contains_key(&current_guardposition) {

                        if seen_before.get(&current_guardposition) == Some(&current_direction) {
                            stuck = true;
                            break;
                        }
                    }

                    seen_before.insert(current_guardposition.clone(), current_direction);

                    if safety_stop > 200 {
                        stuck = true;
                        break;
                    }

                    match current_direction {
                        // Move Guard Upwards
                        '^' => {
                            if puzzle[current_guardposition[0] - 1][current_guardposition[1]] == '#' {
                                current_direction = '>';
                                safety_stop += 1;
                                continue;
                            }
                            current_guardposition[0] -= 1;
                        }
                        // Move Guard Right
                        '>' => {
                            if puzzle[current_guardposition[0]][current_guardposition[1] + 1] == '#' {
                                current_direction = 'v';
                                safety_stop += 1;
                                continue;
                            }
                            current_guardposition[1] += 1;
                        }
                        // Move Guard Down
                        'v' => {
                            if puzzle[current_guardposition[0] + 1][current_guardposition[1]] == '#' {
                                current_direction = '<';
                                safety_stop += 1;
                                continue;
                            }
                            current_guardposition[0] += 1;
                        }
                        // Move Guard Left
                        '<' => {
                            if puzzle[current_guardposition[0]][current_guardposition[1] - 1] == '#' {
                                current_direction = '^';
                                safety_stop += 1;
                                continue;
                            }
                            current_guardposition[1] -= 1;
                        }
                        _ => continue,
                    }
                }
                // Reset the puzzle to its original state
                puzzle[i][j] = original_puzzlestate;

                // Increase counter if stuck
                if stuck {
                    counter += 1;
                }

        }
    }
    counter.try_into().unwrap()
    }
}


// Convert string to 2d vector, used in both parts
fn convert_string_to_guardfield(input: &str, linebreak: i32) -> Vec<Vec<char>> {
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
                130
            }
        }
        Err(_) => 130,
    }
}