use aoc2025_rust::day09::*;

#[test]
fn day09_part01_test() {
    let input = include_str!("../data/day09.txt");
    assert_eq!(part01(input), 4754955192);
}

#[test]
fn day09_part02_test() {
    let input = include_str!("../data/day09.txt");
    assert_eq!(part02(input), 1568849600);
}
