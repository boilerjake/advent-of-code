fn part_one(input: &str) -> usize {
    input.lines().count()
}

fn part_two(input: &str) -> usize {
    input.lines().count()
}

fn main() {
    #[allow(unused_variables)]
    let [sample, input] = ["sample.txt", "input.txt"]
        .map(|filename| std::fs::read_to_string(filename).expect("Failed to open file."));

    dbg!(part_one(&sample));
    dbg!(part_two(&sample));
}
