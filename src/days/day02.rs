use std::collections::HashMap;

use strum::EnumString;


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, EnumString)]
#[strum(serialize_all = "lowercase")]
enum Color {
    Red,
    Green,
    Blue,
}

struct Record {
    id: usize,
    sets: Vec<HashMap<Color, usize>>,
}

fn get_input() -> &'static str {
    include_str!("../../data/day02.txt")
}

fn parse_input(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|s| {
            let s = &s[5..];

            let (id, s) = s.split_once(':').unwrap();

            let id = id.parse().unwrap();
            let sets = s
                .split(';')
                .map(|s| {
                    s.split(',')
                        .map(|s| {
                            let (number, color) = s.trim().split_once(' ').unwrap();

                            (color.parse::<Color>().unwrap(), number.parse::<usize>().unwrap())
                        })
                        .collect()
                })
                .collect();

            Record { id, sets }
        })
        .collect()
}

pub fn solve_part1() {
    let input = get_input();
    let records = parse_input(input);

    let solution = records
        .iter()
        .filter_map(|Record { id, sets }| {
            sets.iter()
                .all(|set| {
                    set.get(&Color::Red).unwrap_or(&0) <= &12
                        && set.get(&Color::Green).unwrap_or(&0) <= &13
                        && set.get(&Color::Blue).unwrap_or(&0) <= &14
                })
                .then_some(*id)
        })
        .sum::<usize>();

    println!("{solution}");
}

pub fn solve_part2() {
    let input = get_input();
    let records = parse_input(input);

    let solution = records
        .iter()
        .map(|Record { sets, .. }| {
            let (red, green, blue) = sets.iter().fold((0, 0, 0), |acc, set| {
                (
                    *set.get(&Color::Red).unwrap_or(&0).max(&acc.0),
                    *set.get(&Color::Green).unwrap_or(&0).max(&acc.1),
                    *set.get(&Color::Blue).unwrap_or(&0).max(&acc.2),
                )
            });

            red * green * blue
        })
        .sum::<usize>();

    println!("{solution}");
}
