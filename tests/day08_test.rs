use aoc2025_rust::day08::*;

#[test]
fn day08_part01_test() {
    let input = include_str!("../data/day08.txt");
    assert_eq!(part01(input), 8361881885);
}

#[test]
fn day08_part02_test() {
    let input = include_str!("../data/day08.txt");
    assert_eq!(part02(input), 244188);
}
