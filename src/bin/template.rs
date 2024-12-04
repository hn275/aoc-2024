#![allow(unused_variables, unused_imports)]

use std::{
    env, fs,
    io::{self, BufRead, Read},
};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let input = parse_input(&args[1]);
    part_1(&input);
    part_2(&input);
    Ok(())
}

fn part_1(input: &String) {}

fn part_2(input: &String) {}

fn parse_input(filename: &str) -> String {
    let mut buf = String::new();
    let _ = fs::File::open(filename).unwrap().read_to_string(&mut buf);
    buf
}
