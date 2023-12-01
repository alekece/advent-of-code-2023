use std::path::PathBuf;

use advent_of_code_2023 as aoc;
use advent_of_code_2023::{Day, PuzzlePart};

use clap::Parser;
use eyre::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opt {
    day: Day,
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long, default_value = "part1")]
    puzzle_part: PuzzlePart,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let opt = Opt::parse();
    let solution = aoc::solve(&opt.input, opt.day, opt.puzzle_part)?;

    println!("{solution}");

    Ok(())
}
