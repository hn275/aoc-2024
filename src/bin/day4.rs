#![allow(unused_variables, unused_imports)]

use std::{
    env, fs,
    io::{self, BufRead, Read},
};

type Matrix = Vec<Vec<char>>;

fn main() -> io::Result<()> {
    let a = [1, 2, 3];
    let args = env::args().collect::<Vec<String>>();
    let input = parse_input(&args[1]);
    part_1(&input);
    part_2(&input);
    Ok(())
}

fn part_1(input: &Matrix) {
    type Dir = (i64, i64);

    const XMAS: &'static str = "XMAS";
    const DIRECTION: [Dir; 8] = [
        (-1, -1), // up left
        (-1, 0),  // up
        (-1, 1),  // up right
        (0, -1),  // left
        (0, 1),   // right
        (1, -1),  // down left
        (1, 0),   // down
        (1, 1),   // down
    ];

    let row_len = input.len();
    let col_len = input[0].len();

    let mut r: u64 = 0;
    for i in 0..row_len {
        for j in 0..col_len {
            if input[i][j] != 'X' {
                continue;
            }

            for (v, h) in DIRECTION {
                let mut row = i as i64;
                let mut col = j as i64;
                let mut ok = true;

                for c in XMAS.chars() {
                    // out of bound
                    let mut oob = row < 0 || row >= (row_len as i64);
                    oob = oob || col < 0 || col >= (col_len as i64);
                    if oob {
                        ok = false;
                        break;
                    }

                    let p = input[row as usize][col as usize];
                    if input[row as usize][col as usize] != c {
                        ok = false;
                        break;
                    }

                    row += v;
                    col += h;
                }

                if ok {
                    r += 1;
                }
            }
        }
    }

    println!("Part 1: {r}");
}

fn part_2(input: &Matrix) {
    let row_len = input.len();
    let col_len = input[0].len();

    let mut r: u64 = 0;

    let ul = (-1, -1); // up left
    let ur = (-1, 1); // up right
    let dl = (1, -1); // down left
    let dr = (1, 1); // down right

    for i in 1..row_len - 1 {
        for j in 1..col_len - 1 {
            let root = input[i][j];
            if input[i][j] != 'A' {
                continue;
            }

            let ulc = input[((i as i64) + ul.0) as usize][((j as i64) + ul.1) as usize];
            let urc = input[((i as i64) + ur.0) as usize][((j as i64) + ur.1) as usize];
            let dlc = input[((i as i64) + dl.0) as usize][((j as i64) + dl.1) as usize];
            let drc = input[((i as i64) + dr.0) as usize][((j as i64) + dr.1) as usize];

            if !(ulc == 'M' && drc == 'S') && !(ulc == 'S' && drc == 'M') {
                continue;
            }

            if !(urc == 'M' && dlc == 'S') && !(urc == 'S' && dlc == 'M') {
                continue;
            }

            r += 1;
        }
    }

    println!("Part 2: {r}");
}

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
