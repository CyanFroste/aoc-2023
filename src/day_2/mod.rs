// https://adventofcode.com/2023/day/2

use std::{fs::read_to_string, str::FromStr};

pub fn run() {
    pt1();
    pt2();
    println!();
}

fn pt2() {
    let input = read_to_string("src/day_2/input_pt2.txt").unwrap();

    let games: Vec<Game> = input.lines().map(|x| x.parse().unwrap()).collect();

    let res: u32 = games.iter().map(|game| game.required_set().power()).sum();

    println!("Pt.2 -> {}", res);
}

fn pt1() {
    let input = read_to_string("src/day_2/input_pt1.txt").unwrap();

    let games: Vec<Game> = input.lines().map(|x| x.parse().unwrap()).collect();

    let max_set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    let res: u32 = games
        .iter()
        .filter_map(|x| {
            if x.is_possible_with(&max_set) {
                Some(x.id)
            } else {
                None
            }
        })
        .sum();

    println!("Pt.1 -> {}", res);
}

impl Game {
    fn required_set(&self) -> Set {
        let mut target = Set { ..self.sets[0] };

        for set in self.sets.iter().skip(1) {
            target.red = target.red.max(set.red);
            target.green = target.green.max(set.green);
            target.blue = target.blue.max(set.blue);
        }

        target
    }

    fn is_possible_with(&self, max: &Set) -> bool {
        self.sets
            .iter()
            .all(|set| set.red <= max.red && set.green <= max.green && set.blue <= max.blue)
    }
}

impl Set {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl FromStr for Game {
    type Err = ();

    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = s.strip_prefix("Game ").unwrap();
        let mut splits = stripped.split(":");

        let id = splits.next().and_then(|x| x.parse().ok()).unwrap();

        let sets = splits
            .next()
            .unwrap()
            .split(";")
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .unwrap();

        Ok(Game { id, sets })
    }
}

impl FromStr for Set {
    type Err = ();

    //  1 blue, 2 green
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for c in s.split(",") {
            let mut ds = c.split_ascii_whitespace();
            let d = ds.next().unwrap().trim().parse().unwrap();
            let name = ds.next().unwrap();

            if name.contains("red") {
                red = d;
            } else if name.contains("green") {
                green = d;
            } else if name.contains("blue") {
                blue = d;
            }
        }

        Ok(Set { red, green, blue })
    }
}
