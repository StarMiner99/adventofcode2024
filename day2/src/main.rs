use std::fs;

use regex::Regex;

fn check_if_safe(report: &Vec<i32>) -> bool {
    let mut prev_num = -1;
    let mut direction = 0;
    let mut safe = true;

    for num in report {
        if prev_num == -1 { // check if we have a number to begin with
            prev_num = *num;
            continue;
        }
        // check if the difference is ok
        let diff = num.abs_diff(prev_num);
        if !(diff <= 3 && diff >= 1) {
            safe = false;
            break;
        }
        // check if we have a direction
        if direction == 0 {
            direction = (num - prev_num).signum();
        } else {
            if direction != (num - prev_num).signum() { // check if direction is the same
                safe = false;
                break;
            }
        }
        prev_num = *num;
    }

    safe
}


const INPUT_PATH: &str = "input.txt";
fn main() {
    let re = Regex::new(r"([0-9]+)").unwrap();

    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let input:Vec<&str> = input.split_terminator('\n').collect();

    let mut safe_reports = 0;
    let mut unsafe_reports = vec![];
    for line in input {
        let report: Vec<i32> = re.captures_iter(line).map(|c| {
            c.extract::<1>().1[0].parse::<i32>().unwrap()
        }).collect();
       
        if check_if_safe(&report) {
            safe_reports += 1;
        } else {
            unsafe_reports.push(report);
        }
    }

    let mut dampened_safe = safe_reports;

    for unsafe_report in unsafe_reports {
        let mut safe = false;
        for (index, _) in unsafe_report.iter().enumerate() {
            let mut unsafe_report = unsafe_report.clone();
            unsafe_report.remove(index);
            safe = safe || check_if_safe(&unsafe_report);
        }
        if safe {
            dampened_safe += 1;
        }
    }
    
    println!("Safe reports: {safe_reports}");
    println!("Dampened: {dampened_safe}");
}
