use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn first_part() -> i32 {
    // Get input file and open it
    let input = File::open(get_input()).unwrap();
    let reader = BufReader::new(input);

    // Create list vectors
    let mut reports: Vec<Vec<i32>> = Vec::new();

    // Read lines from file and add to list vectors
    for line in reader.lines() {
        let curline = line.unwrap();
        let report: Vec<i32> = curline
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        reports.push(report);
    };

    // Convert Vec<Vec<i32>> to Vec<&[i32]>
    let report_refs: Vec<&[i32]> = reports.iter().map(|v| v.as_slice()).collect();

    // Check report safety and return valids
    ReportSafety::check_safety(&report_refs)
}


// Struct for first_part
struct ReportSafety;
impl ReportSafety {
    pub fn check_safety(reports: &[&[i32]]) -> i32 {
        let mut valids = 0;
        // Check increment and decrease
        for report in reports {
            if ReportSafety::check_increment(report) {
                valids += 1;
                continue;
            }
            if ReportSafety::check_decrease(report) {
                valids += 1;
            }
        }
        valids
    }
    // Check if it increments
    fn check_increment(report: &[i32]) -> bool {
        for i in 0..(report.len() - 1) {
            if report[i] > report[i+1] {
                return false;
            }
            if report[i+1] - report[i] > 3 || report[i+1] - report[i] < 1 {
                return false;
            }
        }
        true
    }
    // Check if it decreases
    fn check_decrease(report: &[i32]) -> bool {
        for i in 0..(report.len() - 1) {
            if report[i] < report[i+1] {
                return false;
            }
            if report[i] - report[i+1] > 3 || report[i] - report[i+1] < 1 {
                return false;
            }
        }
        true
    }
}





fn second_part() -> i32 {
    // Get input file and open it
    let input = File::open(get_input()).unwrap();
    let reader = BufReader::new(input);

    // Create list vectors
    let mut reports: Vec<Vec<i32>> = Vec::new();

    // Read lines from file and add to list vectors
    for line in reader.lines() {
        let curline = line.unwrap();
        let report: Vec<i32> = curline
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        reports.push(report);
    };

    // Convert Vec<Vec<i32>> to Vec<&[i32]>
    let report_refs: Vec<&[i32]> = reports.iter().map(|v| v.as_slice()).collect();

    // Check report safety and return valids
    ReportSafetySecond::check_safety_second(&report_refs)
}

// Struct for second_part
struct ReportSafetySecond;
impl ReportSafetySecond {
    pub fn check_safety_second(reports: &[&[i32]]) -> i32 {
        let mut valids = 0;
        for report in reports {
            if ReportSafetySecond::check_increment_second(report) {
                valids += 1;
                continue;
            }
            if ReportSafetySecond::check_decrease_second(report) {
                valids += 1;
            }
        }
        valids
    }

    // Check if it increments
    fn check_increment_second(report: &[i32]) -> bool {
        fn is_valid_increase(report: &[i32]) -> bool {
            for i in 1..report.len() {
                let diff = report[i] - report[i - 1];
                if diff < 1 || diff > 3 || report[i] <= report[i - 1] {
                    return false;
                }
            }
            true
        }
    
        // Check if report is valid without removing any element
        if is_valid_increase(report) {
            return true;
        }
    
        // Check if report is valid after removing one element
        for i in 0..report.len() {
            let mut modified_report = report.to_vec();
            modified_report.remove(i);
            if is_valid_increase(&modified_report) {
                return true;
            }
        }
    
        // If none of the above conditions are met, return false
        false
    }

    // Check if it decreases
    fn check_decrease_second(report: &[i32]) -> bool {
        fn is_valid_decrease(report: &[i32]) -> bool {
            for i in 1..report.len() {
                let diff = report[i - 1] - report[i];
                if diff < 1 || diff > 3 || report[i] >= report[i - 1] {
                    return false;
                }
            }
            true
        }

        // Check if report is valid without removing any element
        if is_valid_decrease(report) {
            return true;
        }

        // Check if report is valid after removing one element
        for i in 0..report.len() {
            let mut modified_report = report.to_vec();
            modified_report.remove(i);
            if is_valid_decrease(&modified_report) {
                return true;
            }
        }

        // If none of the above conditions are met, return false
        false
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

