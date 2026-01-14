use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod day_one;
mod day_two;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    day: Day,
}

#[derive(Subcommand)]
enum Day {
    DayOne { input_file: PathBuf },
    DayTwo { input_file: PathBuf },
}

fn main() {
    let cli = Cli::parse();
    match cli.day {
        Day::DayOne { input_file } => day_one::main(input_file),
        Day::DayTwo { input_file } => day_two::main(input_file),
    }
}
