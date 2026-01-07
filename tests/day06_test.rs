use aoc2025_rust::day06::*;

#[test]
fn day06_part01_test() {
    let input = include_str!("../data/day06.txt");
    assert_eq!(part01(input), 5524274308182);
}

#[test]
fn day06_part02_test() {
    let input = include_str!("../data/day06.txt");
    assert_eq!(part02(input), 8843673199391);
}
