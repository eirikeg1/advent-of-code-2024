use std::fs;

fn main() {
    println!("Note: I refuse to learn regex for this"); // <--- IMPORTANT!!!!

    let input_data = fs::read_to_string("inputs/input_1.txt").expect("Could not read input file");

    let mul_sum: u128 = parse_memory(input_data.as_str());

    println!("The answer to task 1 is {}", mul_sum);

    let conditional_str = remove_conditionals(&input_data);

    let mul_sum_2: u128 = parse_memory(conditional_str.as_str());

    println!("The answer to task 2 is {}", mul_sum_2);
}

fn check_character(character: char, expected: char, counter: &mut u32) {
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
            0 => check_character(character, 'm', &mut counter),
            1 => check_character(character, 'u', &mut counter),
            2 => check_character(character, 'l', &mut counter),
            3 => check_character(character, '(', &mut counter),
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

fn remove_conditionals(input: &str) -> String {
    let mut result = String::new();
    let mut multiplication_active = true;
    let mut iter = input.char_indices().peekable();

    while let Some(&(i, _)) = iter.peek() {
        if multiplication_active {
            if input[i..].starts_with("don't()") {
                multiplication_active = false;
            } else {
                let (_, ch) = iter.next().unwrap();
                result.push(ch);
            }
        } else {
            if input[i..].starts_with("do()") {
                multiplication_active = true;
            } else {
                iter.next();
            }
        }
    }
    result
}



