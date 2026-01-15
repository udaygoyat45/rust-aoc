use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod day_one;
mod day_two;
mod day_three;

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
    Day03 {input_file: PathBuf}
}

fn main() {
    let cli = Cli::parse();
    match cli.day {
        Day::Day01 { input_file } => day_one::main(input_file),
        Day::Day02 { input_file } => day_two::main(input_file),
        Day::Day03 { input_file } => day_three::main(input_file)
    }
}
