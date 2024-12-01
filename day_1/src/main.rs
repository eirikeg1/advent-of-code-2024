use std::collections::HashMap;
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

    let differences = left_values.clone()
        .into_iter()
        .zip(right_values.clone().into_iter())
        .map(|(left, right)| {
            if left >= right {
                left - right
            } else {
                right - left
            }
        });

    let diff_sum: u32 = differences.sum();

    println!("The final value for task 1 is {diff_sum}");
    
    let mut right_counts = HashMap::<u32, u32>::new();

    right_values.into_iter()
        .for_each(|left_value| {
            *right_counts.entry(left_value).or_insert(0) += 1;
        });
    
    let similarity_score: u32 = left_values.into_iter()
        .map(|left_value| {
            left_value * right_counts.get(&left_value).unwrap_or(&0)
        })
        .sum();
    

    println!("The final similarity score for task 2 is {similarity_score}");
    
}
