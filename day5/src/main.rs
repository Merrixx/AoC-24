use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn first_part() -> i32 {
        // Get input file and open it
        let input = File::open(get_input()).unwrap();
        let reader = BufReader::new(input);

        // Create list vectors
        let mut conditions: Vec<Vec<i32>> = Vec::new();
        let mut print_orders: Vec<Vec<i32>> = Vec::new();

        // Read lines from file and add to list vectors
        for line in reader.lines() {
            let curline = line.unwrap();

            // Add conditions to conditions list with '|' separator
            if curline.contains('|') {
                let condition: Vec<i32> = curline
                    .split('|')
                    .map(|num| num.parse().unwrap())
                    .collect();
                conditions.push(condition)
            }

            // Add print orders to print_orders list with ',' separator
            else if curline.contains(',') {
                let print_order: Vec<i32> = curline
                    .split(',')
                    .map(|num| num.parse().unwrap())
                    .collect();
                print_orders.push(print_order)
            }
        }

        // Call check_print_order function from PrintOrder struct
        PrintOrder::check_print_order(conditions, print_orders)
}

// Struct for first_part
struct PrintOrder;
impl PrintOrder {
    pub fn check_print_order(conditions: Vec<Vec<i32>>, print_orders: Vec<Vec<i32>>) -> i32 {
        // Create a map from conditions for print orders
        let conditons_map = PrintOrder::set_print_conditions(conditions);
        
        // Only get correct print orders
        let correct_prints = PrintOrder::check_prints(conditons_map, print_orders);

        // Sum correct print orders
        let ordersum = PrintOrder::sum_print_orders(correct_prints);
        ordersum
    }


    fn set_print_conditions(conditions: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        // Add values for keys in map
        for condition in conditions {
            let mut vec = Vec::new();
            if map.contains_key(&condition[0]) {
                vec = map.get(&condition[0]).unwrap().to_vec();
            }
            vec.push(condition[1]);
            map.insert(condition[0], vec);
        }
        map
    }

    fn check_prints(map: HashMap<i32, Vec<i32>>, print_orders: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut correct_prints: Vec<Vec<i32>> = Vec::new();

        // Check if print orders are correct
        for print_order in print_orders {
            let mut wrong_order: bool = false;
            let mut reversed_print_order = print_order.clone();
            reversed_print_order.reverse();
            
            // Itterate through print orders
            for i in 0..reversed_print_order.len() {

                // Get map values for current print number
                if map.contains_key(&reversed_print_order[i]) {
                    let values = map.get(&reversed_print_order[i]).unwrap();

                    // Check if values match with print order
                    for value in values {
                        if reversed_print_order[i..].contains(value) {
                            wrong_order = true;
                        }
                    }
                }
            }
            // Add correct print orders to correct_prints list
            if !wrong_order {
                correct_prints.push(print_order);
            }
        }
        correct_prints
    }

    fn sum_print_orders(correct_prints: Vec<Vec<i32>>) -> i32 {
        let mut sum: i32 = 0;

        // Get middle part of print order and sum it (Based on task)
        for print_order in correct_prints {
            let middle_part: u32 = ((print_order.len() - 1) / 2).try_into().unwrap();
            sum += print_order[middle_part as usize];
        }
        sum
    }
}


fn second_part() -> i32 {
    // Get input file and open it
    let input = File::open(get_input()).unwrap();
    let reader = BufReader::new(input);

    // Create list vectors
    let mut conditions: Vec<Vec<i32>> = Vec::new();
    let mut print_orders: Vec<Vec<i32>> = Vec::new();

    // Read lines from file and add to list vectors
    for line in reader.lines() {
        let curline = line.unwrap();

        // Add conditions to conditions list with '|' separator
        if curline.contains('|') {
            let condition: Vec<i32> = curline
                .split('|')
                .map(|num| num.parse().unwrap())
                .collect();
            conditions.push(condition)
        }

        // Add print orders to print_orders list with ',' separator
        else if curline.contains(',') {
            let print_order: Vec<i32> = curline
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect();
            print_orders.push(print_order)
        }
    }

    // Call check_print_order function from PrintOrder struct
    PrintOrderIncorrects::check_print_order_second(conditions, print_orders)
}




// Struct for second_part
struct PrintOrderIncorrects;
impl PrintOrderIncorrects {
    pub fn check_print_order_second(conditions: Vec<Vec<i32>>, print_orders: Vec<Vec<i32>>) -> i32 {
        // Create a map from conditions for print orders
        let conditons_map = PrintOrderIncorrects::set_print_conditions_second(conditions);
        //println!("{:?}", conditons_map);

        // Only get incorrect print orders
        let incorrect_prints = PrintOrderIncorrects::check_prints_second(conditons_map.clone(), print_orders);
        //println!("{:?}", incorrect_prints);

        let corrected_prints = PrintOrderIncorrects::correct_prints_second(conditons_map, incorrect_prints);
        //println!("{:?}", corrected_prints);
        
        // Sum correct print orders
        let ordersum = PrintOrderIncorrects::sum_print_orders_second(corrected_prints);
        ordersum
    }

    fn set_print_conditions_second(conditions: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        // Add values for keys in map
        for condition in conditions {
            let mut vec = Vec::new();
            if map.contains_key(&condition[0]) {
                vec = map.get(&condition[0]).unwrap().to_vec();
            }
            vec.push(condition[1]);
            map.insert(condition[0], vec);
        }
        map
    }

    fn check_prints_second(map: HashMap<i32, Vec<i32>>, print_orders: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut correct_prints: Vec<Vec<i32>> = Vec::new();

        // Check if print orders are correct
        for print_order in print_orders {
            let mut wrong_order: bool = false;
            let mut reversed_print_order = print_order.clone();
            reversed_print_order.reverse();
            
            // Itterate through print orders
            for i in 0..reversed_print_order.len() {

                // Get map values for current print number
                if map.contains_key(&reversed_print_order[i]) {
                    let values = map.get(&reversed_print_order[i]).unwrap();

                    // Check if values match with print order
                    for value in values {
                        if reversed_print_order[i..].contains(value) {
                            wrong_order = true;
                        }
                    }
                }
            }
            // Add correct print orders to correct_prints list
            if wrong_order {
                correct_prints.push(print_order);
            }
        }
        correct_prints
    }

    // Adjust order to correct print
    fn correct_prints_second(map: HashMap<i32, Vec<i32>>, incorrect_prints: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut correct_prints: Vec<Vec<i32>> = Vec::new();

        for print_order in incorrect_prints {
            let mut reversed_print_order = print_order.clone();
            reversed_print_order.reverse();
            let mut wrong_order: bool = true;

            while wrong_order {
                wrong_order = true;
                let mut changed: bool = false;
                for i in 0..reversed_print_order.len() {
                    if map.contains_key(&reversed_print_order[i]) {
                        let values = map.get(&reversed_print_order[i]).unwrap();

                        // Check if values match with print order
                        for value in values {
                            if reversed_print_order[i..].contains(value) {
                                changed = true;

                                // Remove and add value to correct position
                                let temp = reversed_print_order[i];
                                reversed_print_order.remove(i);
                                reversed_print_order.push(temp);

                                // Break if value was moved
                                break;
                            }
                        }
                    }
                }
                if !changed {
                    wrong_order = false;
                }
            }

            reversed_print_order.reverse();
            correct_prints.push(reversed_print_order);
        }
        correct_prints
    }

    fn sum_print_orders_second(correct_prints: Vec<Vec<i32>>) -> i32 {
        let mut sum: i32 = 0;

        // Get middle part of print order and sum it (Based on task)
        for print_order in correct_prints {
            let middle_part: u32 = ((print_order.len() - 1) / 2).try_into().unwrap();
            sum += print_order[middle_part as usize];
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
