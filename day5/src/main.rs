use std::{collections::HashMap, fs, vec};

use regex::Regex;

fn check_if_correct(in_check: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool{
    let mut prev_numbers = vec![];
    for n in in_check {
        let result = rules.get(&n).map_or(true, |checks|{
            for check in checks {
                if prev_numbers.contains(check) {
                    return false;
                }
            }
            return true;
        });
        if !result {
            return false;
        }

        prev_numbers.push(*n);
    }

    true
}

fn fix_order(in_check: &mut Vec<u32>, rules: &HashMap<u32, Vec<u32>>, start_index: usize) {

    let check_range = &in_check.clone()[start_index..];

    for n in check_range {
        if let Some(checks) = rules.get(n) {
            for check in checks {
                let index_del = in_check.iter().position(|x| x == n ).unwrap() + 1;
                if let Some(index) = in_check[..index_del].iter().position(|x| {x == check}) {
                    in_check.insert(index, *n);
                    in_check.remove(index_del);
                    fix_order(in_check, rules, index);
                    return;
                }
            }
        }
    }
}

const INPUT_PATH: &str = "input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let input_split: Vec<&str> = input.split_terminator("\n\n").collect();
    let input_checks = input_split[1];

    // create rules for every page number
    let rule_re = Regex::new(r"([0-9]+)\|([0-9]+)").unwrap();

    let mut rule_map = HashMap::new();

    for (_, [n1, n2]) in rule_re.captures_iter(input.as_str()).map(|c| c.extract()) {
        let n1: u32 = n1.parse().unwrap();
        let n2: u32 = n2.parse().unwrap();
        if !rule_map.contains_key(&n1) {
            rule_map.insert(n1, vec![n2]);
        } else {
            rule_map.get_mut(&n1).unwrap().push(n2);
        }
    }

    // get inputs and format them in vecs
    let check_re = Regex::new(r"([0-9]+)").unwrap();
    let input_checks_split:Vec<&str> = input_checks.split_terminator("\n").collect();

    let mut failed_input_checks = vec![];

    let mut sum_middle = 0;

    for input_check in input_checks_split {
        let check: Vec<u32> = check_re.captures_iter(input_check).map(|c| {
            c.extract::<1>().1[0].parse().unwrap()
        }).collect();

        let check_out = check_if_correct(&check, &rule_map);

        if check_out {
            sum_middle += check[check.len()/2];
        } else {
            failed_input_checks.push(check);
        }

    }

    println!("Sum only check: {}", sum_middle);

    let mut sum_fixed = 0;

    // as soon as we detect an error move the element that is currently beeing checked in front of the number that is in front of it but should be after it.
    for mut input in failed_input_checks {
        print!("{:?} -> ", input);

        fix_order(&mut input, &rule_map, 0);
        println!("{:?}", input);

        sum_fixed += input[input.len()/2];
    }

    println!("Sum after fix: {}", sum_fixed);



}