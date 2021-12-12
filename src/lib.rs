use cached::proc_macro::cached;

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    0
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    0
}

#[cached]
fn parse_input(input: &'static str) -> Vec<&'static str> {
    input
        .lines()
        .filter_map(|s| match s.trim() {
            "" => None,
            _ => Some(s.trim()),
        })
        .collect()
}
