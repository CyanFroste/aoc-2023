// https://adventofcode.com/2023/day/1

use std::fs::read_to_string;

pub fn run() {
    pt1();
    pt2();
    println!();
}

fn pt2() {
    const WORDED_NUMS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let parse_worded_num = |w: &str| (WORDED_NUMS.iter().position(|&x| x == w).unwrap() + 1) as u32;

    let input = read_to_string("src/day_1/input_pt2.txt").unwrap();

    let res: u32 = input
        .lines()
        .map(|line| {
            let pure_digits = line
                .chars()
                .enumerate()
                .filter_map(|(i, c)| c.to_digit(10).and_then(|d| Some((i, d))));

            let worded_digits =
                WORDED_NUMS
                    .iter()
                    .filter_map(|w| line.find(w).and_then(|i| Some((i, parse_worded_num(w)))))
                    .chain(WORDED_NUMS.iter().filter_map(|w| {
                        line.rfind(w).and_then(|i| Some((i, parse_worded_num(w))))
                    }));

            let mut v: Vec<_> = pure_digits.chain(worded_digits).collect();
            v.sort();

            let first = v.first();
            let last = v.last().or(first);
            first.unwrap().1 * 10 + last.unwrap().1
        })
        .sum();

    println!("Pt.2 -> {}", res);
}

fn pt1() {
    let input = read_to_string("src/day_1/input_pt1.txt").unwrap();

    let res: u32 = input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter_map(|c| c.to_digit(10));
            let first = chars.next();
            let last = chars.last().or(first);
            first.unwrap() * 10 + last.unwrap()
        })
        .sum();

    println!("Pt.1 -> {}", res);
}
