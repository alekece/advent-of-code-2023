use clap::ValueEnum;

use days::*;

mod days;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Day {
    Day01,
    Day02,
    Day03,
}

pub fn solve(day: Day, second_part: bool) {
    match (day, second_part) {
        (Day::Day01, false) => day01::solve_part1(),
        (Day::Day01, true) => day01::solve_part2(),
        (Day::Day02, false) => day02::solve_part1(),
        (Day::Day02, true) => day02::solve_part2(),
        (Day::Day03, false) => day03::solve_part1(),
        (Day::Day03, true) => day03::solve_part2(),
    }
}
