use aoc2025_rust::day04::*;

#[test]
fn day04_part01_test() {
    let input = include_str!("../data/day04.txt");
    assert_eq!(part01(input), 1502);
}

#[test]
fn day04_part02_test() {
    let input = include_str!("../data/day04.txt");
    assert_eq!(part02(input), 9083);
}
