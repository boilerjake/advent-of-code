use {{crate_name}}::{part_one, part_two};

const SAMPLE: &str = include_str!("../sample.txt");

#[test]
fn test_part_one() {
    assert_eq!(part_one(SAMPLE), 0);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(SAMPLE), 0);
}
