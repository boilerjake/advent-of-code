#![allow(dead_code)]

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");

fn part_one(input: &str) -> usize {
    input
        .split('\n')
        .filter_map(|s| match s.trim() {
            "" => None,
            _ => Some(s.trim()),
        })
        .count() // TODO
}

fn part_two(input: &str) -> usize {
    unimplemented!("{}", input)
}

fn main() {
    dbg!(part_one(SAMPLE));
    // dbg!(part_two(SAMPLE));
}
