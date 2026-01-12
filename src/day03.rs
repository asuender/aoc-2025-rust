// Helpers

fn max_joltage_greedy(bank: &str, k: usize) -> u64 {
    let bank = bank.trim();
    let n = bank.len();

    if k >= n {
        return bank.parse().unwrap();
    }

    let bank: Vec<u32> = bank.chars().map(|d| d.to_digit(10).unwrap()).collect();

    let mut result: Vec<u32> = vec![];
    let mut to_skip = n - k;

    for digit in bank {
        while !result.is_empty() && to_skip > 0 && result[result.len() - 1] < digit {
            result.pop();
            to_skip -= 1;
        }

        result.push(digit);
    }

    result.truncate(k);
    result
        .iter()
        .map(|d| char::from_digit(*d, 10).unwrap())
        .collect::<String>()
        .parse()
        .unwrap()
}

// Solution

pub fn part01(input: &str) -> u64 {
    let mut res: u64 = 0;

    for line in input.lines() {
        let digits: Vec<u64> = line
            .chars()
            .filter_map(|d| d.to_digit(10))
            .map(|d| d as u64)
            .collect();

        let max_comb = digits
            .iter()
            .enumerate()
            .flat_map(|(i, dgt_i)| digits[i + 1..].iter().map(move |dgt_j| dgt_i * 10 + dgt_j))
            .max();

        if let Some(max) = max_comb {
            res += max;
        }
    }

    res
}

pub fn part02(input: &str) -> u64 {
    input.lines().map(|bank| max_joltage_greedy(bank, 12)).sum()
}
