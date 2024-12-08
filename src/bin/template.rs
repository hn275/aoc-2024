#![allow(unused_variables, unused_imports)]

use std::{
    env, fs,
    io::{self, BufRead, Read},
};

type ParsedInput = String;

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let input = parse_input(&args[1]);
    part_1(&input);
    part_2(&input);
    Ok(())
}

fn part_1(input: &ParsedInput) {}

fn part_2(input: &ParsedInput) {}

fn parse_input(filename: &str) -> ParsedInput {
    let mut buf = String::new();
    let _ = fs::File::open(filename).unwrap().read_to_string(&mut buf);
    buf
}
