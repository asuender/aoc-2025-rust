use aoc2025_rust::day05::*;

#[test]
fn day05_part01_test() {
    let input = include_str!("../data/day05.txt");
    assert_eq!(part01(input), 821);
}

#[test]
fn day05_part02_test() {
    let input = include_str!("../data/day05.txt");
    assert_eq!(part02(input), 344771884978261);
}
