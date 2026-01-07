// Helpers

fn parse_instructions(line: &str) -> (char, i32) {
    let dir = line.chars().next().unwrap();
    let by: i32 = line[1..].parse().unwrap();

    (dir, by)
}

fn get_sign_from(dir: char) -> i32 {
    match dir {
        'L' => -1,
        'R' => 1,
        _ => panic!("Direction should be either L or R."),
    }
}

fn zeros_during_move(start_dial: i32, dir: char, by: i32) -> u64 {
    let sign = get_sign_from(dir);
    let k0 = (-sign * start_dial).rem_euclid(100);
    let first_zero = if k0 == 0 { 100 } else { k0 };

    if first_zero > by {
        return 0;
    }

    (1 + (by - first_zero) / 100) as u64
}

// Solutions

pub fn part01(input: &str) -> u64 {
    let mut dial: i32 = 50;
    let mut count_dial_zero = 0;

    for line in input.lines() {
        let (dir, by) = parse_instructions(line);

        dial = if dir == 'R' { dial + by } else { dial - by };
        dial = dial.rem_euclid(100);

        if dial == 0 {
            count_dial_zero += 1;
        }
    }

    count_dial_zero
}

pub fn part02(input: &str) -> u64 {
    let mut dial = 50;
    let mut count_dial_zero = 0;

    for line in input.lines() {
        let (dir, by) = parse_instructions(line);

        count_dial_zero += zeros_during_move(dial, dir, by);

        let sign = get_sign_from(dir);
        dial = (dial + sign * by).rem_euclid(100);
    }

    count_dial_zero
}
