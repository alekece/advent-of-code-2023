use std::io::{BufRead, BufReader, Read};

use crate::{Error, Result};

pub fn solve_part1<R: Read>(reader: BufReader<R>) -> Result<String> {
    let digits = reader
        .lines()
        .map(|line| {
            let line = line?;

            let radix = 10;
            let mut digits = line.chars().filter_map(|c| c.to_digit(radix));

            let first = digits.next().ok_or(Error::InvalidInput(line.clone()))?;
            let last = digits.last().unwrap_or(first);

            Ok((first * radix + last) as usize)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(digits.iter().sum::<usize>().to_string())
}

pub fn solve_part2<R: Read>(reader: BufReader<R>) -> Result<String> {
    let digits = reader
        .lines()
        .map(|line| {
            let line = line?;

            let radix = 10;
            let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

            let mut digits = line.chars().enumerate().filter_map(|(i, c)| {
                c.to_digit(radix).or_else(|| {
                    numbers
                        .iter()
                        .enumerate()
                        .find_map(|(j, number)| line[i..].starts_with(number).then_some((j + 1) as u32))
                })
            });

            let first = digits.next().ok_or(Error::InvalidInput(line.clone()))?;
            let last = digits.last().unwrap_or(first);

            Ok((first * radix + last) as usize)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(digits.iter().sum::<usize>().to_string())
}
