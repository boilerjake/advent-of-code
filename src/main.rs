#![allow(dead_code)]

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");

fn part_one(input: &[&str]) -> usize {
    unimplemented!("{:#?}", input)
}

fn part_two(input: &[&str]) -> usize {
    unimplemented!("{:#?}", input)
}

fn main() {
    let input: Vec<_> = SAMPLE
        .lines()
        .filter_map(|s| match s.trim() {
            "" => None,
            _ => Some(s.trim()),
        })
        .collect();

    dbg!(part_one(&input));
    // dbg!(part_two(&input));
}
