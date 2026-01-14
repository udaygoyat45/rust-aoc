use std::fs;
use std::path::PathBuf;

enum Direction {
    Left,
    Right
}

struct Instruction {
    direction: Direction,
    distance: i64
}

impl Instruction {
    fn create(raw: &str) -> Option<Self> {
        let mut chars = raw.chars();
        let first_char = chars.next()?;
        let direction = match first_char {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => return None
        };
        let distance = chars.as_str().parse::<i64>().ok()?;
        let instruction = Instruction {direction, distance};
        Some(instruction)
    }
}

pub fn adjust_dial(dial: i64) -> i64 {
    let mut adjusted = dial;
    while adjusted < 0 {
        adjusted += 100;
    }
    adjusted % 100
}

pub fn main(input_path: PathBuf) {
    let content = fs::read_to_string(input_path).expect("Unable to open {input_path}");
    let content_opt: Option<Vec<Instruction>> = content.split('\n').map(|line| Instruction::create(line)).collect();
    let contents = content_opt.expect("Problem parsing {input_path}");

    let mut dial: i64 = 50;
    let mut counter = 0;
    for Instruction {direction, distance} in contents {
        match direction {
            Direction::Left => dial -= distance,
            Direction::Right => dial += distance
        } 
        dial = adjust_dial(dial);
        if dial == 0 {
            counter += 1;
        }
    }
    println!("Answer: {counter}")
}