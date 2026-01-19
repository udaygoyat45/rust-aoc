use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod day1;
mod day3;
mod day2;
mod day4;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    day: Day,
}

#[derive(Subcommand)]
enum Day {
    Day01 { input_file: PathBuf },
    Day02 { input_file: PathBuf },
    Day03 { input_file: PathBuf },
    Day04 { input_file: PathBuf },
}

fn main() {
    let cli = Cli::parse();
    match cli.day {
        Day::Day01 { input_file } => day1::main(input_file),
        Day::Day02 { input_file } => day2::main(input_file),
        Day::Day03 { input_file } => day3::main(input_file),
        Day::Day04 { input_file } => day4::main(input_file)
    }
}
