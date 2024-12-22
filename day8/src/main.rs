use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;

fn first_part() -> i32 {
    let input = File::open(get_input()).unwrap();
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();

    let mut antennavector: Vec<Vec<char>> = Vec::with_capacity(lines.len());

    lines.iter().for_each(|l| {
        let curline: Vec<char> = Vec::from_iter(l.chars());
        antennavector.push(curline);
    });
    AntenaRange::get_anti_antenas(antennavector)
}


// Struct for first_part
struct AntenaRange;
impl AntenaRange {
    pub fn get_anti_antenas(puzzle: Vec<Vec<char>>) -> i32 {
        let mut antiantennasum: HashSet<Vec<i32>> = HashSet::new();
        let mut antennalocations: HashMap<char, Vec<Vec<i32>>> = HashMap::new();
        let mut puzzlecopy = puzzle.clone();


        // Get Antena Locations and save them in a hashmap
        for (y, line) in puzzle.iter().enumerate() {
            for (x, _row) in line.iter().enumerate() {

                if puzzle[y][x] != '.' {

                    // If character is inside map check for the other similar antenas
                    if antennalocations.contains_key(&puzzle[y][x]) {
                        // Get the location of the existing antennas
                        let characterantennas :Vec<Vec<i32>> = antennalocations.get(&puzzle[y][x]).unwrap().to_vec();
                        let curx: i32 = x as i32;
                        let cury: i32 = y as i32;

                        for mapantenna in &characterantennas {
                            // Get the x and y difference between the antenas
                            let ydiff: i32 = cury - mapantenna[1];
                            let xdiff: i32;
                            let xdirection: i32;

                            if curx < mapantenna[0] {
                                xdiff = mapantenna[0] - curx;
                                xdirection = 1;
                            } else {
                                xdiff = curx - mapantenna[0];
                                xdirection = 2;
                            }


                            if cury + ydiff < puzzle.len() as i32 {
                                if xdirection == 1 && curx - xdiff >= 0 {
                                    if puzzle[cury as usize + ydiff as usize][curx as usize - xdiff as usize] == '.' {
                                        puzzlecopy[cury as usize + ydiff as usize][curx as usize - xdiff as usize] = '#';
                                    }
                                    antiantennasum.insert([cury + ydiff, curx - xdiff].to_vec());
                                }   else if xdirection == 2 && curx + xdiff < puzzle.len() as i32 {
                                    if puzzle[cury as usize + ydiff as usize][curx as usize + xdiff as usize] == '.' {
                                        puzzlecopy[cury as usize + ydiff as usize][curx as usize + xdiff as usize] = '#';
                                    }
                                    antiantennasum.insert([cury + ydiff, curx + xdiff].to_vec());
                                }
                            }

                            if mapantenna[1] - ydiff >= 0 {
                                if xdirection == 1 && mapantenna[0] + xdiff < puzzle.len() as i32 {
                                    if puzzle[mapantenna[1] as usize - ydiff as usize][mapantenna[0] as usize + xdiff as usize] == '.' {
                                        puzzlecopy[mapantenna[1] as usize - ydiff as usize][mapantenna[0] as usize + xdiff as usize] = '#';
                                    }
                                    antiantennasum.insert([mapantenna[1] - ydiff, mapantenna[0] + xdiff].to_vec());
                                } else if xdirection == 2 && mapantenna[0] - xdiff >= 0 {
                                    if puzzle[mapantenna[1] as usize - ydiff as usize][mapantenna[0] as usize - xdiff as usize] == '.' {
                                        puzzlecopy[mapantenna[1] as usize - ydiff as usize][mapantenna[0] as usize - xdiff as usize] = '#';
                                    }
                                    antiantennasum.insert([mapantenna[1] - ydiff, mapantenna[0] - xdiff].to_vec());
                                }
                            }
                        }
                        // Add the antena to the map
                        if let Some(location) = antennalocations.get_mut(&puzzle[y][x]) {
                            location.push([curx, cury].to_vec());
                        }
                    } else {
                        // Extend the map vec with new antenna
                        antennalocations.insert(puzzle[y][x], vec![[x as i32, y as i32].to_vec()]);
                       
                    }
                }
                }
            }
        antiantennasum.len() as i32
    }
}



fn second_part() -> i32 {
    let input = File::open(get_input()).unwrap();
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();

    let mut antennavector: Vec<Vec<char>> = Vec::with_capacity(lines.len());

    lines.iter().for_each(|l| {
        let curline: Vec<char> = Vec::from_iter(l.chars());
        antennavector.push(curline);
    });
    AntenaRangeSecond::get_anti_antenas_second(antennavector)
}

struct AntenaRangeSecond;
impl AntenaRangeSecond {
    pub fn get_anti_antenas_second(puzzle: Vec<Vec<char>>) -> i32 {
        let mut antiantennasum: HashSet<Vec<i32>> = HashSet::new();
        let mut antennalocations: HashMap<char, Vec<Vec<i32>>> = HashMap::new();
        let mut puzzlecopy = puzzle.clone();


        // Get Antena Locations and save them in a hashmap
        for (y, line) in puzzle.iter().enumerate() {
            for (x, _row) in line.iter().enumerate() {

                if puzzle[y][x] != '.' {

                    // If character is inside map check for the other similar antenas
                    if antennalocations.contains_key(&puzzle[y][x]) {
                        // Get the location of the existing antennas
                        let characterantennas :Vec<Vec<i32>> = antennalocations.get(&puzzle[y][x]).unwrap().to_vec();
                        let curx: i32 = x as i32;
                        let cury: i32 = y as i32;

                        // Take current antenna as anti antenna
                        antiantennasum.insert([cury, curx].to_vec());

                        for mapantenna in &characterantennas {
                            // Get the x and y difference between the antenas
                            let ydiff: i32 = cury - mapantenna[1];
                            let xdiff: i32;
                            let xdirection: i32;
                            if curx < mapantenna[0] {
                                xdiff = mapantenna[0] - curx;
                                xdirection = 1;
                            } else {
                                xdiff = curx - mapantenna[0];
                                xdirection = 2;
                            }

                            // Take map antenna as anti antenna
                            antiantennasum.insert([mapantenna[1], mapantenna[0]].to_vec());

                            // Create the anti antenas for the current antena
                            let mut antisignal: bool = true;
                            let mut ymove: i32 = cury.clone();
                            let mut xmove: i32 = curx.clone();

                            while antisignal == true {
                                if ymove + ydiff < puzzle.len() as i32 {
                                    if xdirection == 1 && xmove - xdiff >= 0 {
                                        if puzzle[ymove as usize + ydiff as usize][xmove as usize - xdiff as usize] == '.' {
                                            puzzlecopy[ymove as usize + ydiff as usize][xmove as usize - xdiff as usize] = '#';
                                        }
                                        antiantennasum.insert([ymove + ydiff, xmove - xdiff].to_vec());
                                        xmove -= xdiff;
                                        

                                    }   else if xdirection == 2 && xmove + xdiff < puzzle.len() as i32 {
                                        if puzzle[ymove as usize + ydiff as usize][xmove as usize + xdiff as usize] == '.' {
                                            puzzlecopy[ymove as usize + ydiff as usize][xmove as usize + xdiff as usize] = '#';
                                        }
                                        antiantennasum.insert([ymove + ydiff, xmove + xdiff].to_vec());
                                        xmove += xdiff;

                                    }
                                }   else { antisignal = false; }
                                ymove += ydiff;
                            }


                            // Create the anti antenas for the other antenas
                            let mut antisignal: bool = true;
                            let mut ymove: i32 = mapantenna[1].clone();
                            let mut xmove: i32 = mapantenna[0].clone();

                            while antisignal == true {
                                if ymove - ydiff >= 0 {
                                    if xdirection == 1 && xmove + xdiff < puzzle.len() as i32 {
                                        if puzzle[ymove as usize - ydiff as usize][mapantenna[0] as usize + xdiff as usize] == '.' {
                                            puzzlecopy[ymove as usize - ydiff as usize][xmove as usize + xdiff as usize] = '#';
                                        }
                                        antiantennasum.insert([ymove - ydiff, xmove + xdiff].to_vec());
                                        xmove += xdiff;

                                    } else if xdirection == 2 && xmove - xdiff >= 0 {
                                        if puzzle[ymove as usize - ydiff as usize][xmove as usize - xdiff as usize] == '.' {
                                            puzzlecopy[ymove as usize - ydiff as usize][xmove as usize - xdiff as usize] = '#';
                                        }
                                        antiantennasum.insert([ymove - ydiff, xmove - xdiff].to_vec());
                                        xmove -= xdiff;
                                    }
                                }   else { antisignal = false; }
                                ymove -= ydiff;
                            }
                    }


                        // Add the antena to the map
                        if let Some(location) = antennalocations.get_mut(&puzzle[y][x]) {
                            location.push([curx, cury].to_vec());
                        }
                    } else {
                        // Extend the map vec with new antenna
                        antennalocations.insert(puzzle[y][x], vec![[x as i32, y as i32].to_vec()]);
                       
                    }
                }
                }
            }
        antiantennasum.len() as i32
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
