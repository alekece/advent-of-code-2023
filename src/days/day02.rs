use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use strum::EnumString;

use crate::{Error, Result};

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

fn parse_input<R: Read>(reader: BufReader<R>) -> Result<Vec<Record>> {
    reader
        .lines()
        .map(|s| {
            let s = &s?[5..];

            let Some((id, s)) = s.split_once(':') else {
                return Err(Error::InvalidInput(s.to_string()));
            };

            let id = id.parse()?;
            let sets = s
                .split(';')
                .map(|s| {

                    let set = s
                        .split(',')
                        .map(|s| {
                            let Some((number, color)) = s.trim().split_once(' ') else {
                                return Err(Error::InvalidInput(s.to_string()));
                            };

                            let number = number.parse::<usize>()?;
                            let color = color.parse::<Color>()?;

                            Ok((color, number))
                        })
                        .collect::<Result<HashMap<_, _>>>()?;

                    Ok(set)
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(Record { id, sets })
        })
        .collect()
}

pub fn solve_part1<R: Read>(reader: BufReader<R>) -> Result<String> {
    let records = parse_input(reader)?;

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

    Ok(solution.to_string())
}

pub fn solve_part2<R: Read>(reader: BufReader<R>) -> Result<String> {
    let records = parse_input(reader)?;

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

    Ok(solution.to_string())
}
