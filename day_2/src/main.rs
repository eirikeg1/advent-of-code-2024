use std::fs;

fn main() {
    let input_data = fs::read_to_string("inputs/input_1.txt").expect("Could not read input file");

    let reports: Vec<Vec<u32>> = input_data
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|value| value.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let safe_reports = reports.into_iter().map(|report| {
        check_safety(&report)
    }).filter(|&is_safe| is_safe).count();

    println!("Number of safe reports: {safe_reports}");
}

fn check_safety(report: &Vec<u32>) -> bool {
    if report.len() < 2 {
        println!("ERROR, report shorter than 2");
        return false;
    }

    let increasing = report[0] < report[1];
    let mut prev = report[0];

    for &level in report.iter().skip(1) {
        let increase_check = increasing && (prev < level);
        let decrease_check = !increasing && (prev > level);
        let diff = prev.abs_diff(level);

        if !increase_check && !decrease_check || (diff < 1) || (diff > 3) {
            return false;
        }

        prev = level;
    }
    true
}
