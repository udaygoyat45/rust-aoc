use std::fs;
use std::path::PathBuf;

enum Direction { Left, Right }

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

fn adjust_dial(dial: &mut i64, instruction: &Instruction) {
    let distance = instruction.distance;
    match instruction.direction {
        Direction::Left => *dial = (*dial - distance).rem_euclid(100),
        Direction::Right => *dial = (*dial + distance).rem_euclid(100),
    }
}

pub fn main(input_path: PathBuf) {
    let content = fs::read_to_string(input_path).expect("Unable to open {input_path}");
    let content_opt: Option<Vec<Instruction>> = content.split('\n').map(|line| Instruction::create(line)).collect();
    let contents = content_opt.expect("Problem parsing {input_path}");

    let mut dial_part1: i64 = 50;
    let mut part1_counter: i64 = 0;

    for instruction in &contents {
        adjust_dial(&mut dial_part1, instruction);
        if dial_part1 == 0 {
            part1_counter += 1;
        }
    }
    println!("Part One Answer: {part1_counter}");

    let mut dial_part2: i64 = 50;
    let mut part2_counter: i64 = 0;
    for instruction in contents {
        let distance = instruction.distance;
        part2_counter += distance / 100;
        let remaining_dist = distance % 100;

        match instruction.direction {
            Direction::Left => {
                if dial_part2 > 0 && remaining_dist >= dial_part2 {
                    part2_counter += 1;
                }
            }
            Direction::Right => {
                if dial_part2 + remaining_dist >= 100 {
                    part2_counter += 1;
                }
            }
        }
        adjust_dial(&mut dial_part2, &instruction);
    }

    println!("Part Two Answer: {part2_counter}")
}