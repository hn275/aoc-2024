#![allow(unused_variables, unused_imports)]

use std::{
    collections::HashMap,
    env, fs,
    io::{self, BufRead, Read},
};

type Rule = HashMap<u64, Vec<u64>>;

#[derive(Debug)]
struct ParsedInput {
    rules: Rule,
    print_order: Vec<Vec<u64>>,
}

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let input = parse_input(&args[1]);

    part_1(&input);
    part_2(&input);
    Ok(())
}

fn part_1(input: &ParsedInput) {
    let mut result: u64 = 0;

    (&input.print_order).into_iter().for_each(|update| {
        let mut printed: Vec<u64> = Vec::new();

        for (index, page) in update.into_iter().enumerate() {
            let printed_after = &update[index + 1..];
            let rules = input.rules.get(page);
            let rules = match rules {
                None => Vec::new(),
                Some(rules) => rules.to_owned(),
            };

            for p in (&printed).into_iter() {
                if rules.contains(&p) {
                    return;
                }
            }
            printed.push(*page);
        }

        let mid = update[update.len() / 2];

        result += mid;
    });

    println!("Part 1: {result}");
}

fn part_2(input: &ParsedInput) {
}

fn parse_input(filename: &str) -> ParsedInput {
    let mut buf = String::new();
    let _ = fs::File::open(filename).unwrap().read_to_string(&mut buf);

    let a = buf.split("\n\n").collect::<Vec<_>>();
    assert!(a.len() == 2);

    let mut rules = Rule::new();
    a[0].split("\n").into_iter().for_each(|line| {
        let vals = line
            .split("|")
            .into_iter()
            .map(|val| val.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        rules
            .entry(vals[0])
            .or_insert_with(|| Vec::new())
            .push(vals[1]);
    });

    let print_order = a[1]
        .split("\n")
        .into_iter()
        .filter(|l| l.len() > 0)
        .map(|line| {
            line.split(",")
                .into_iter()
                .map(|val| val.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<_>>();

    ParsedInput { rules, print_order }
}
