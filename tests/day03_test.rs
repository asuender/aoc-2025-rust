use aoc2025_rust::day03::*;

#[test]
fn day03_part01_test() {
    let input = include_str!("../data/day03.txt");
    assert_eq!(part01(input), 17263);
}

#[test]
fn day03_part02_test() {
    let input = include_str!("../data/day03.txt");
    assert_eq!(part02(input), 170731717900423);
}
