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
    let mut wrong = Vec::new();

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
                    wrong.push(update);
                    return;
                }
            }
            printed.push(*page);
        }
    });

    let mut result: u64 = 0;

    wrong.into_iter().for_each(|updates| {
        let mut printed = Vec::new();

        for (index, page) in updates.into_iter().enumerate() {
            if printed.len() == 0 {
                printed.push(page);
                continue;
            }

            let printed_after = &updates[index + 1..];
            let rules = match input.rules.get(page) {
                None => {
                    printed.push(page);
                    println!("\tno rules for {page}");
                    continue;
                }
                Some(rules) => rules,
            };

            let mut right_ord = printed.clone();
            let mut violate = false;
            for (i, &p) in (&printed).into_iter().enumerate() {
                violate = rules.contains(&p);
                if violate {
                    right_ord = printed[..i].to_vec();
                    right_ord.push(page);
                    right_ord = [right_ord, printed[i..].to_vec()].concat();
                    break;
                }
            }

            if !violate {
                right_ord.push(page);
            }

            printed = right_ord.clone();
        }

        result += printed[printed.len() / 2];
    });

    println!("Part 2: {result}");
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
