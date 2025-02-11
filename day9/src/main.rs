use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn first_part() -> i64 {
    let input = File::open(get_input()).unwrap();
    let reader = BufReader::new(input);
    let memoryvar: String = reader.lines().next().unwrap().unwrap();

    //println!("reader: {:?}" , memoryvar);
    FileSystemChecksum::filesystem_checksum(memoryvar)
}

// Struct for first_part
struct FileSystemChecksum;
impl FileSystemChecksum {
    pub fn filesystem_checksum(diskmap: String) -> i64 {
        let mut adjusted_diskmap: Vec<(i32, String)> = Vec::new();
        let mut file_block: bool = true;
        let mut id_counter: i32 = 0;


        // Adjuste the diskmap for easier parsing
        for char in diskmap.chars() {
            if file_block {
                if let Some(file_number) = char.to_digit(10) {
                    for _ in 0..file_number {
                        adjusted_diskmap.push((id_counter ,id_counter.to_string()));
                    }
                    id_counter += 1;
                    file_block = false;
                }
            }   else {
                if let Some(file_number) = char.to_digit(10) {
                    for _ in 0..file_number {
                        adjusted_diskmap.push((0, ".".to_string()));
                    }
                    file_block = true;
                }
            }
        }

        // Change the order of the adjusted_diskmap (swap .'s with correct number)
        for i in 0..adjusted_diskmap.len() {
            if adjusted_diskmap[i].1 == "." {
                for j in (i..adjusted_diskmap.len()).rev() {
                    if adjusted_diskmap[j].0 != 0 {
                        adjusted_diskmap.swap(i, j);
                        break;
                    }
                }
            }
        }
        //Get the filesystem checksum
        let mut totalsum: i64 = 0;
        for i in 0..adjusted_diskmap.len() {
            if adjusted_diskmap[i].1 != "." {
                totalsum += adjusted_diskmap[i].0 as i64 * i as i64;
            }
        }

        totalsum
    }
}


fn main() {
    println!("First Part Answer: {}", first_part());
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