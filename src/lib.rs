pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    0
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    0
}

fn parse_input(input: &'static str) -> Vec<&'static str> {
    input
        .lines()
        .filter_map(|s| match s.trim() {
            "" => None,
            _ => Some(s.trim()),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 0);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 0);
    }
}
