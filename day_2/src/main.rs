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

    let safe_reports = reports.clone().into_iter().filter(|report| {
        check_safety(&report)
    });

    println!("Number of safe reports: {}", safe_reports.count());

    let safe_reports_with_problem_dampener = reports.into_iter().filter(|report| {
        check_safety_with_problem_dampener(&report)
    });

    println!("Number of safe reports with problem dampener: {}", safe_reports_with_problem_dampener.count());
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

fn check_safety_with_problem_dampener(report: &Vec<u32>) -> bool {
    if check_safety(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);
        if check_safety(&new_report) {
            return true;
        }
    }
    false
}

