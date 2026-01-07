use aoc2025_rust::day01::*;

#[test]
fn day01_part01_test() {
    let input = include_str!("../data/day01.txt");
    assert_eq!(part01(input), 992);
}

#[test]
fn day01_part02_test() {
    let input = include_str!("../data/day01.txt");
    assert_eq!(part02(input), 6133);
}
