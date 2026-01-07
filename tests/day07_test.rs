use aoc2025_rust::day07::*;

#[test]
fn day07_part01_test() {
    let input = include_str!("../data/day07.txt");
    assert_eq!(part01(input), 1566);
}

#[test]
fn day07_part02_test() {
    let input = include_str!("../data/day07.txt");
    assert_eq!(part02(input), 5921061943075);
}
