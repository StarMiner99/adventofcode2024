use std::fs;

use regex::Regex;


const INPUT_PATH: &str = "input.txt";
fn main() {
    let xmas_re = Regex::new(r"XMAS").unwrap();
    let samx_re = Regex::new(r"SAMX").unwrap();
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let input_split: Vec<&str> = input.split_whitespace().collect();
    let input_ln_len = input_split[0].len();
    let input_len = input_split.len();
    
    // count horizontal occurrences
    let horizontal_count = xmas_re.find_iter(input.as_str()).count() + samx_re.find_iter(input.as_str()).count();
    println!("{}", horizontal_count);

    // count vertical occurrences
    let mut input_vertical: Vec<String> = vec!["".to_string(); input_ln_len];
    for line in &input_split {
        for (i, c) in line.chars().enumerate() {
            input_vertical[i].push(c);
        }
    }
    let input_vertical = input_vertical.join("\n");

    let vertical_count = xmas_re.find_iter(input_vertical.as_str()).count() + samx_re.find_iter(input_vertical.as_str()).count();
    println!("{}", vertical_count);

    // count occurrences in diagonal one (right top to bottom left)
    let mut input_dia1 = vec!["".to_string(); input_ln_len + input_len];

    for (y, line) in input_split.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            input_dia1[x+y].push(c);
        }
    }
    let input_dia1_str = input_dia1.join("\n");
    
    let dia1_count = xmas_re.find_iter(input_dia1_str.as_str()).count() + samx_re.find_iter(input_dia1_str.as_str()).count();
    println!("{}", dia1_count);

    // count occurrences in diagonal two (left top to bottom right)
    let mut input_dia2 = vec!["".to_string(); input_ln_len + input_len];

    for (y, line) in input_split.iter().rev().enumerate() {
        for (x, c) in line.chars().enumerate() {
            input_dia2[x+y].push(c);
        }
    }
    let input_dia2_str = input_dia2.join("\n");

    let dia2_count = xmas_re.find_iter(input_dia2_str.as_str()).count() + samx_re.find_iter(input_dia2_str.as_str()).count();
    println!("{}", dia2_count);
    let total_count = horizontal_count + vertical_count + dia1_count + dia2_count;
    println!("TOTAL XMAS COUNT: {}",total_count);

    // ***** PART 2 *****
    // find all As and check if they are part of a X mas
    let mut mas_count = 0;
    for (y, line) in input_split.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != 'A' || y < 1 || y > input_split.len() - 2 || x < 1 || x > line.len() - 2 {
                continue;
            }

            let char1: char = input_split[y - 1].chars().nth(x - 1).unwrap();
            let char2: char = input_split[y - 1].chars().nth(x + 1).unwrap();
            let char3: char = input_split[y + 1].chars().nth(x - 1).unwrap();
            let char4: char = input_split[y + 1].chars().nth(x + 1).unwrap();

            if  ((char1 == 'M' || char1 == 'S') && (char4 == 'M' || char4 == 'S') && char1 != char4) &&
                ((char2 == 'M' || char2 == 'S') && (char3 == 'M' || char3 == 'S') && char2 != char3) {
                mas_count += 1;
            }
            
        }
    }

    println!("X-MAS count: {}", mas_count)
}
