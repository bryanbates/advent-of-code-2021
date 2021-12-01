type Depth = usize;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Depth> {
    input
        .lines()
        .map(|l| l.trim().parse::<Depth>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Depth]) -> usize {
    input
        .windows(2)
        .map(|ds| -> usize { (ds[1] > ds[0]).into() }) // 1 if increasing, else 0
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Depth]) -> usize {
    input
        .windows(3)
        .map(|ds| ds[0] + ds[1] + ds[2]) // Sum 3d window
        .collect::<Vec<Depth>>()
        .windows(2)
        .map(|ds| -> usize { (ds[1] > ds[0]).into() }) // 1 if increasing, else 0
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"199
200
208
210
200
207
240
269
260
263
"#;

    #[test]
    fn example1() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE_INPUT)), 7)
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part2(&input_generator(EXAMPLE_INPUT)), 5)
    }
}
