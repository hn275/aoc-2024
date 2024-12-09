#![allow(unused_variables, unused_imports)]

use std::{
    env, fs,
    io::{self, BufRead, Read},
};

// horizontal, vertical
#[derive(Debug, Default, Clone)]
struct Direction(i8, i8);

#[derive(Default, Debug)]
struct ParsedInput {
    floor_map: Vec<Vec<char>>,
    guard_loc: (usize, usize),
    guard_dir: Direction,
    dimension: (usize, usize),
}

impl Direction {
    fn new(face: char) -> Direction {
        match face {
            '^' => Direction(0, -1),
            '>' => Direction(1, 0),
            'v' => Direction(0, 1),
            '<' => Direction(-1, 0),
            _ => unimplemented!(),
        }
    }

    fn turn(&mut self) {
        if self.0 == 0 && self.1 == -1 {
            // if ^
            self.0 = 1;
            self.1 = 0;
        } else if self.0 == 1 && self.1 == 0 {
            // if  >
            self.0 = 0;
            self.1 = 1;
        } else if self.0 == 0 && self.1 == 1 {
            // if v
            self.0 = -1;
            self.1 = 0;
        } else if self.0 == -1 && self.1 == 0 {
            // if <
            self.0 = 0;
            self.1 = -1;
        } else {
            panic!("unhandled case {self:?}");
        }
    }
}

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let input = parse_input(&args[1]);
    part_1(&input);
    part_2(&input);
    Ok(())
}

fn part_1(input: &ParsedInput) {
    let guard_loc = input.guard_loc;
    let mut floor_map = input.floor_map.clone();

    let mut dir = input.guard_dir.clone();
    let mut x = guard_loc.1 as i64;
    let mut y = guard_loc.0 as i64;

    let mut step: u64 = 0;
    loop {
        if x == 0
            || x == (input.dimension.0 as i64 - 1)
            || y == 0
            || y == (input.dimension.1 as i64 - 1)
        {
            step += 1;
            break;
        }

        floor_map[y as usize][x as usize] = 'X';

        let next_x = x + dir.0 as i64;
        let next_y = y + dir.1 as i64;

        let next_tile = floor_map[next_y as usize][next_x as usize];
        let seen = next_tile == 'X';

        if next_tile == '#' {
            dir.turn();
            continue;
        }

        if !seen {
            step += 1;
        }

        x = next_x;
        y = next_y;
    }

    println!("Part 1: {step}");
}

fn part_2(input: &ParsedInput) {}

fn parse_input(filename: &str) -> ParsedInput {
    let mut buf = String::new();
    let _ = fs::File::open(filename).unwrap().read_to_string(&mut buf);

    let floor_map = buf
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut guard_loc = (0, 0);
    for (i, row) in floor_map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let c = floor_map[i][j];
            if c != '.' && c != '#' {
                guard_loc = (i, j);
                break;
            }
        }
    }

    let dimension = (floor_map.len(), floor_map[0].len());
    let guard_dir = Direction::new(floor_map[guard_loc.0][guard_loc.1]);

    ParsedInput {
        floor_map,
        guard_loc,
        guard_dir,
        dimension,
    }
}
