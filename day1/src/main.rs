use std::fs;

use regex::Regex;

const INPUT_PATH: &str = "day1/input.txt";

fn distance_sum(list1: &mut Vec<u32>, list2: &mut Vec<u32>) -> u32 {
    let mut distance_sum = 0;

    list1.sort();
    list2.sort();

    for (i, element) in list1.iter().enumerate() {
        let distance = element.abs_diff(list2[i]);
        distance_sum += distance;
    }

    distance_sum
}

fn similarity_score(list1: &Vec<u32>, list2: &Vec<u32>) -> u32 {
    let mut sim_score= 0;
    for element in list1 {
        let mut count = 0;
        for element2 in list2 {
            if element2 == element {
                count += 1;
            }
        }
        sim_score += element * count;
    }

    sim_score
}

fn main() {
    let re = Regex::new(r"([0-9]+)\s{3}([0-9]+)").unwrap();

    let mut list1 = vec![];
    let mut list2 = vec![];

    let input = fs::read_to_string(INPUT_PATH).unwrap();

    for (_, [n1, n2]) in re.captures_iter(input.as_str()).map(|c| c.extract()) {
        list1.push(n1.parse::<u32>().unwrap());
        list2.push(n2.parse::<u32>().unwrap());       
    }

    let distance_sum = distance_sum(&mut list1, &mut list2);
    println!("{distance_sum}");

    let similarity_score = similarity_score(&list1, &list2);
    println!("{similarity_score}")

}
