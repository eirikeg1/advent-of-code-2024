use std::fs;

fn main() {
    let input_data = fs::read_to_string("inputs/input_1.txt").expect("Could not read input file");

    let mul_sum: u128 = input_data.lines().map(parse_memory).sum();

    println!("The answer to task 1 is {}", mul_sum);
}

fn check_charater(character: char, expected: char, counter: &mut u32) {
    if character == expected {
        *counter += 1;
    } else {
        *counter = 0;
    }
}

fn parse_memory(input: &str) -> u128 {
    let mut counter = 0;
    let mut num_builder = String::new();
    let mut num_1 = 0;
    let mut num_sum = 0;

    for character in input.chars() {
        match counter {
            0 => check_charater(character, 'm', &mut counter),
            1 => check_charater(character, 'u', &mut counter),
            2 => check_charater(character, 'l', &mut counter),
            3 => check_charater(character, '(', &mut counter),
            4 => {
                if character == ',' && num_builder.len() > 0 {
                    counter += 1;
                    num_1 = num_builder.parse::<u128>().unwrap();
                    num_builder = String::new();
                    continue;
                }

                if character.is_digit(10) {
                    num_builder.push(character);
                } else {
                    counter = 0;
                    num_builder = String::new();
                }
            },
            5 => {
                if character == ')' && num_builder.len() > 0 {
                    counter = 0;
                    let num_2 = num_builder.parse::<u128>().unwrap();
                    num_sum += num_1 * num_2;
                    num_builder = String::new();
                    continue;
                }
                if character.is_digit(10) {
                    num_builder.push(character);
                } else {
                    counter = 0;
                    num_builder = String::new();
                }
            },
            _ => counter = 0
        }
    }
    num_sum
}
