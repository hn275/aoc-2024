use std::{
    env, fs,
    io::{self, BufRead, Read},
};

use regex::Regex;

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let mut file = fs::File::open(&args[1])?;

    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut c = 0;
    part1_string_match(&buf).iter().for_each(|e| {
        let e = &e[4..e.len() - 1];
        let e: Vec<&str> = e.split(",").into_iter().collect();
        let n1 = e[0].parse::<u64>().unwrap();
        let n2 = e[1].parse::<u64>().unwrap();
        c += n1 * n2;
    });

    println!("Part 1: {c}");

    let result = part2_string_match(&buf);
    let mut c = 0;
    result.iter().for_each(|mat| {
        let mat = &mat.as_str()[4..mat.len() - 1];
        let mat: Vec<&str> = mat.split(",").into_iter().collect();
        let num1 = mat[0].parse::<u64>().unwrap();
        let num2 = mat[1].parse::<u64>().unwrap();
        c += num1 * num2;
    });

    println!("Part 2: {c}");

    Ok(())
}

fn part2_string_match(input: &str) -> Vec<String> {
    let mul_regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    let mut enabled = true; // mul instructions are enabled
    let mut buf = Vec::new();

    let mut matches: Vec<_> = mul_regex
        .find_iter(input)
        .chain(do_regex.find_iter(input))
        .chain(dont_regex.find_iter(input))
        .collect();

    matches.sort_by_key(|m| m.start());

    for mat in matches {
        let matched_str = &input[mat.start()..mat.end()];
        if do_regex.is_match(matched_str) {
            enabled = true;
        } else if dont_regex.is_match(matched_str) {
            enabled = false;
        } else if mul_regex.is_match(matched_str) {
            if enabled {
                buf.push(matched_str.to_string());
            }
        }
    }
    buf
}

fn part1_string_match(input: &String) -> Vec<String> {
    regex::Regex::new(r"mul\(\d+,\d+\)")
        .unwrap()
        .find_iter(input)
        .into_iter()
        .map(|el| el.as_str().to_owned())
        .collect::<Vec<String>>()
}
