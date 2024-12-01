use std::env;
use std::iter::Sum;
use std::fs;

fn main() {
    let input_data = fs::read_to_string("inputs/input_1.txt").expect("Could not read input file");
    
    
    let mut left_values: Vec<u32> = vec![];
    let mut right_values: Vec<u32> = vec![];

    for line in input_data.lines() {
        let mut values = line.split("   ");
        left_values.push(values.next().unwrap().parse::<u32>().unwrap());
        right_values.push(values.next().unwrap().parse::<u32>().unwrap());
    }

    left_values.sort();
    right_values.sort();

    let differences = left_values
        .into_iter()
        .zip(right_values.into_iter())
        .map(|(left, right)| {
            if left >= right {
                left - right
            } else {
                right - left
            }
        });

    let final_sum: u32 = differences.sum();

    println!("The final value for task 1 is {final_sum}");

}
