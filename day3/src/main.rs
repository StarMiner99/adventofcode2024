use std::fs;

use regex::Regex;

fn do_mul(input: &str) -> u32{
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut out_n = 0;

    for (_, [n1, n2]) in re.captures_iter(input).map(|c| c.extract()) {
        let n1:u32 = n1.parse().unwrap();
        let n2:u32 = n2.parse().unwrap();
        out_n += n1 * n2;
    }

    out_n
}

const INPUT_PATH: &str = "input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let out_n = do_mul(input.as_str());
    println!("out: {}", out_n);

    // remove text between don't and do
    let do_re = Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap();

    let input_without_dont = do_re.replace_all(&input, "").into_owned();

    let out_without_dont = do_mul(&input_without_dont);

    println!("out without dont blocks: {}", out_without_dont)
}
