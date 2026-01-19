use std::cmp::max;
use std::fs;
use std::path::PathBuf;

#[derive(PartialEq, PartialOrd, Eq)]
struct Interval {
    left: u64,
    right: u64,
}

impl Interval {
    fn parse(raw_str: &str) -> Option<Interval> {
        let mut split = raw_str.split('-');
        let left = split.next()?.parse::<u64>().ok()?;
        let right = split.next()?.parse::<u64>().ok()?;
        Some(Interval { left, right })
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.left.cmp(&other.left)
    }
}

fn merge_interval(intervals: &Vec<Interval>) -> Vec<Interval> {
    let mut merged_intervals = Vec::new();
    let mut idx = 0;
    while idx < intervals.len() {
        let left = intervals[idx].left;
        let mut right = intervals[idx].right;
        let mut jdx = idx;
        while jdx + 1 < intervals.len() && right >= intervals[jdx + 1].left {
            jdx += 1;
            right = max(right, intervals[jdx].right);
        }
        merged_intervals.push(Interval { left, right });
        idx = jdx + 1;
    }
    merged_intervals
}

fn is_invalid(num: u64) -> bool {
    let num_str = num.to_string();
    let len = num_str.len();
    if num_str.len() % 2 == 1 {
        return false;
    }
    num_str[..len / 2] == num_str[len / 2..]
}

fn is_invalid_2(num: u64) -> bool {
    let num_str = num.to_string();
    let len = num_str.len();
    for step in 1..len {
        if len % step != 0 {
            continue;
        }
        let first = &num_str[..step];
        let mut unbroken = true;
        for i in (0..len).step_by(step) {
            if &num_str[i..i + step] != first {
                unbroken = false;
                break;
            }
        }
        if unbroken {
            return true;
        }
    }

    false
}

pub fn main(input_path: PathBuf) {
    let content = fs::read_to_string(input_path).expect("Unable to open {input_path}");
    let content_opt: Option<Vec<Interval>> = content
        .split(',')
        .map(|line| Interval::parse(line))
        .collect();
    let mut intervals = content_opt.expect("Unable to open {input_path}");
    intervals.sort();
    let merged_intervals = merge_interval(&intervals);

    let intervals_len = intervals.len();
    let merged_intervals_len = merged_intervals.len();
    println!(
        "I wanted to make sure the ranges didn't have any overlap, 
so ran a simple interval merging algorithm

intervals length: {intervals_len}
merged_intervals_len: {merged_intervals_len}"
    );

    let mut ans = 0;
    let mut ans_2 = 0;
    for Interval { left, right } in merged_intervals {
        for number in left..right + 1 {
            if is_invalid(number) {
                ans += number;
            }
            if is_invalid_2(number) {
                ans_2 += number;
            }
        }
    }

    println!("First Rule: {ans}");
    println!("Second Rule: {ans_2}")
}
