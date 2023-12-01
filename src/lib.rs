use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

use clap::ValueEnum;
use eyre::Context;
use thiserror::Error;

use days::*;

mod days;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    EyreReport(#[from] eyre::ErrReport),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("No solution found: {0}")]
    NoSolution(String),
    #[error("Empty input")]
    EmptyInput,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Day {
    Day01,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PuzzlePart {
    Part1,
    Part2,
}

pub fn solve(path: &Path, day: Day, puzzle_part: PuzzlePart) -> Result<String> {
    let file = File::open(path).wrap_err_with(|| format!("Cannot open file '{}'", path.display()))?;
    let reader = BufReader::new(file);

    match (day, puzzle_part) {
        (Day::Day01, PuzzlePart::Part1) => day01::solve_part1(reader),
        (Day::Day01, PuzzlePart::Part2) => day01::solve_part2(reader),
    }
}
