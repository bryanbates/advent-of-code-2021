type Chunk = Vec<String>;

// Default implementation: Chunk on whitespace
#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input.lines().map(|l| {
        l.trim().split(' ').map(|s| s.to_owned()).collect::<Chunk>()
    }).collect()
}


#[aoc(day16, part1)]
pub fn part1(input: &[Chunk]) -> u32 {
    input.len() as u32
}

#[aoc(day16, part2)]
pub fn part2(input: &[Chunk]) -> u32 {
    input.iter().map(|c| c.len()).sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"a b c
d e f
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 2)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 6)
    }
}
