use aoc2025_rust::day02::*;

#[test]
fn day02_part01_test() {
    let input = include_str!("../data/day02.txt");
    assert_eq!(part01(input), 24043483400);
}

#[test]
fn day02_part02_test() {
    let input = include_str!("../data/day02.txt");
    assert_eq!(part02(input), 38262920235);
}
