use fancy_regex::Regex;

// Helpers

fn solve(input: &str, repeated: bool) -> u64 {
    let re = match repeated {
        true => Regex::new(r"^([1-9]\d*)\1+$").unwrap(),
        false => Regex::new(r"^([1-9]\d*)\1$").unwrap(),
    };
    let mut invalid: u64 = 0;

    for range in input.split(",") {
        if let Some((lower, upper)) = range.split_once("-") {
            let lower: u64 = lower.trim().parse().unwrap();
            let upper: u64 = upper.trim().parse().unwrap();

            for i in lower..=upper {
                if re.is_match(&i.to_string()).unwrap() {
                    invalid += i;
                }
            }
        }
    }

    invalid
}

// Solution

pub fn part01(input: &str) -> u64 {
    solve(input, false)
}

pub fn part02(input: &str) -> u64 {
    solve(input, true)
}
