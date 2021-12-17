type Puzzle = &str;

fn parse_input(input: &str) -> Puzzle {
    input
}


#[aoc(day25, part1)]
pub fn part1(input: &str) -> u32 {
    let puzzle = parse_input(input);
    0
}

#[aoc(day25, part2)]
pub fn part2(input: &str) -> u32 {
    let puzzle = parse_input(input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"a b c
d e f
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(EXAMPLE_INPUT), 2)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(EXAMPLE_INPUT), 6)
    }
}
