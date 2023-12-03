use advent_of_code_2023::Day;

use clap::Parser;
use eyre::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opt {
    day: Day,
    #[arg(short, long)]
    second_part: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let opt = Opt::parse();

    advent_of_code_2023::solve(opt.day, opt.second_part);

    Ok(())
}
