#![allow(unused_variables, unused_imports)]

use std::{
    env, fs,
    io::{self, BufRead, Read},
};

type Matrix = Vec<Vec<char>>;

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let input = parse_input(&args[1]);
    part_1(&input);
    part_2(&input);
    Ok(())
}

fn part_1(input: &Matrix) {
    let row_len = input.len();
    let col_len = input[0].len();
    dbg!(input, row_len, col_len);

    for row_index in 0..row_len {
        for col_index in 0..col_len {}
    }
}

fn part_2(input: &Matrix) {}

fn parse_input(filename: &str) -> Matrix {
    let mut buf = String::new();
    let _ = fs::File::open(filename).unwrap().read_to_string(&mut buf);
    let mut matrix: Vec<Vec<_>> = Vec::new();
    buf.split("\n").into_iter().for_each(|line| {
        let mut b = Vec::new();
        for c in line.chars() {
            b.push(c);
        }
        if b.len() == 0 {
            return;
        }
        matrix.push(b);
    });
    matrix
}
