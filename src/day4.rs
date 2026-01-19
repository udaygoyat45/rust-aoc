use std::path::PathBuf;
use std::{fs, usize};

fn parse_paper_rolls(raw_str: &str) -> Vec<bool> {
    let rolls: Vec<bool> = raw_str
        .chars()
        .map(|c| if c == '@' { true } else { false })
        .collect();
    rolls
}

fn count_neighbors(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> u8 {
    let mut count = 0;
    let m = grid.len();
    let n = grid[0].len();
    let deltas: [(i8, i8); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for (dr, dc) in deltas {
        let cr = row as isize + dr as isize;
        let cc = col as isize + dc as isize;
        if cr >= 0 && cr < n as isize && cc >= 0 && cc < m as isize {
            count += grid[cr as usize][cc as usize] as u8;
        }
    }
    count
}

fn remove_toilet_paper(grid: &mut Vec<Vec<bool>>) -> usize {
    let mut removed_indices = Vec::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] && count_neighbors(grid, row, col) < 4 {
                removed_indices.push((row, col));
            }
        }
    }

    let removed_count = removed_indices.len();
    for (row, col) in removed_indices {
        grid[row][col] = false;
    }
    removed_count
}

pub fn main(input_path: PathBuf) {
    let content = fs::read_to_string(input_path).expect("Unable to open {input_path}");
    let mut content: Vec<Vec<bool>> = content
        .split('\n')
        .map(|line| parse_paper_rolls(line))
        .collect();

    // ensure the grid is n x m
    let first_line_len = content.first().unwrap().len();
    for line in content.iter().skip(1) {
        assert_eq!(line.len(), first_line_len);
    }

    let answer_part1 = remove_toilet_paper(&mut content);
    println!("Answer to part 1: {}", answer_part1);

    let mut answer_part2 = answer_part1;
    let mut removed = answer_part1;
    while removed > 0 {
        removed = remove_toilet_paper(&mut content);
        answer_part2 += removed;
    }

    println!("Answer to part 2: {answer_part2}");
}
