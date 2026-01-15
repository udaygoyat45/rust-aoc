use std::{fs, path::PathBuf};

struct Bank {
    batteries: Vec<u8>,
}

impl Bank {
    fn parse(raw_str: &str) -> Option<Self> {
        let batteries = raw_str
            .chars()
            .map(|c| c.to_digit(10).map(|d| d as u8))
            .collect::<Option<Vec<u8>>>()?;
        Some(Bank { batteries })
    }
}

fn maximum_voltage(bank: &Bank) -> u8 {
    let batteries = &bank.batteries;
    if batteries.len() < 2 {
        return 0;
    }
    let mut max_digit_from_right = *batteries.last().unwrap();
    let mut max_voltage = 0;
    for &digit in batteries.iter().rev().skip(1) {
        max_voltage = std::cmp::max(max_voltage, digit * 10 + max_digit_from_right);
        max_digit_from_right = std::cmp::max(max_digit_from_right, digit);
    }
    max_voltage
}

fn nested_vector(a: usize, b: usize) -> Vec<Vec<u64>> {
    (0..a).map(|_| vec![0; b]).collect()
}

fn maximum_voltage_2(bank: &Bank) -> u64 {
    let digits = 12;
    let mut dp = nested_vector(bank.batteries.len(), digits);
    dp[0][0] = bank.batteries[0] as u64;
    for i in 1..bank.batteries.len() {
        dp[i][0] = std::cmp::max(dp[i - 1][0], bank.batteries[i] as u64);
    }

    let mut answer = 0;
    for i in 1..bank.batteries.len() {
        for size in 1..digits {
            dp[i][size] = std::cmp::max(
                dp[i - 1][size],
                dp[i - 1][size - 1] * 10 + bank.batteries[i] as u64,
            );
            if size == digits - 1 {
                answer = std::cmp::max(answer, dp[i][size]);
            }
        }
    }
    answer
}

pub fn main(input_path: PathBuf) {
    let content = fs::read_to_string(input_path).expect("Unable to open {input_path}");
    let content_opt: Option<Vec<Bank>> =
        content.split('\n').map(|line| Bank::parse(line)).collect();
    let banks = content_opt.expect("Unable to parse from {input_path}");
    let mut cumulative: u64 = 0;
    let mut cumulative_2: u64 = 0;
    for bank in banks {
        cumulative += maximum_voltage(&bank) as u64;
        cumulative_2 += maximum_voltage_2(&bank);
    }
    println!("Part 1 Answer: {cumulative}");
    println!("Part 2 Answer: {cumulative_2}");
}
