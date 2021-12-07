use rayon::prelude::*;
use memoize::memoize;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn const_consumption(input: &[i32], target: i32) -> i32 {
    input.iter().map(|x| (target-x).abs()).sum()
}

#[memoize]
fn mem_const_consumption(input: Vec<i32>, target: i32) -> i32 {
    input.iter().map(|x| (target-x).abs()).sum()
}

fn par_const_consumption(input: &[i32], target: i32) -> i32 {
    input.par_iter().map(|x| (target-x).abs()).sum()
}

#[aoc(day7, part1, brute)]
pub fn part1(input: &[i32]) -> i32 {
    let min_pos: i32 = *input.iter().min().unwrap();
    let max_pos: i32 = *input.iter().max().unwrap();

    let fuel = (min_pos..=max_pos).map(|pos| {
        const_consumption(input, pos)
    }).collect::<Vec<i32>>();
    *fuel.iter().min().unwrap()
}

#[aoc(day7, part1, parallel)]
pub fn part1_parallel(input: &[i32]) -> i32 {
    let min_pos: i32 = *input.iter().min().unwrap();
    let max_pos: i32 = *input.iter().max().unwrap();

    let fuel = (min_pos..=max_pos).into_par_iter().map(|pos| {
        par_const_consumption(input, pos)
    }).collect::<Vec<i32>>();
    *fuel.iter().min().unwrap()
}

#[aoc(day7, part1, memo)]
pub fn part1_memo(input: &[i32]) -> i32 {
    let min_pos: i32 = *input.iter().min().unwrap();
    let max_pos: i32 = *input.iter().max().unwrap();

    let ivec = input.to_vec();

    let fuel = (min_pos..=max_pos).map(|pos| {
        mem_const_consumption(ivec.clone(), pos)
    }).collect::<Vec<i32>>();
    *fuel.iter().min().unwrap()
}

#[aoc(day7, part1, stats)]
pub fn part1_stats(input: &[i32]) -> i32 {
    let median: i32 = statistical::median(input) as i32;
    const_consumption(input, median)
}

fn cumulative_consumption(input: &[i32], target: i32) -> i32 {
    input.iter()
        .map(|x| {
            let delta = (target - x).abs();
            delta * (delta + 1) / 2
        })
        .sum()
}

#[memoize]
fn mem_cumulative_consumption(input: Vec<i32>, target: i32) -> i32 {
    input.iter()
        .map(|x| {
            let delta = (target - x).abs();
            delta * (delta + 1) / 2
        })
        .sum()
}

fn par_cumulative_consumption(input: &[i32], target: i32) -> i32 {
    input.par_iter()
        .map(|x| {
            let delta = (target - x).abs();
            delta * (delta + 1) / 2
        })
        .sum()
}

#[aoc(day7, part2, brute)]
pub fn part2(input: &[i32]) -> i32 {
    let min_pos: i32 = *input.iter().min().unwrap();
    let max_pos: i32 = *input.iter().max().unwrap();

    let fuel = (min_pos..=max_pos).map(|pos| {
        cumulative_consumption(input, pos)
    }).collect::<Vec<i32>>();

    *fuel.iter().min().unwrap()
}

#[aoc(day7, part2, parallel)]
pub fn part2_parallel(input: &[i32]) -> i32 {
    let min_pos: i32 = *input.iter().min().unwrap();
    let max_pos: i32 = *input.iter().max().unwrap();

    let fuel = (min_pos..=max_pos).into_par_iter().map(|pos| {
        par_cumulative_consumption(input, pos)
    }).collect::<Vec<i32>>();

    *fuel.iter().min().unwrap()
}

#[aoc(day7, part2, memo)]
pub fn part2_memo(input: &[i32]) -> i32 {
    let min_pos: i32 = *input.iter().min().unwrap();
    let max_pos: i32 = *input.iter().max().unwrap();

    let ivec = input.to_vec();

    let fuel = (min_pos..=max_pos).map(|pos| {
        mem_cumulative_consumption(ivec.clone(), pos)
    }).collect::<Vec<i32>>();

    *fuel.iter().min().unwrap()
}

#[aoc(day7, part2, stats)]
pub fn part2_stats(input: &[i32]) -> i32 {
    // Search around the mean
    let mean: f64 = input.iter().sum::<i32>() as f64 / input.len() as f64;
    let fuel_floor: i32 = cumulative_consumption(input, mean.floor() as i32);
    let fuel_ceil: i32 = cumulative_consumption(input, mean.ceil() as i32);
    std::cmp::min(fuel_floor, fuel_ceil)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"16,1,2,0,4,2,7,1,2,14
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 37);
        assert_eq!(part1_parallel(&input_generator(EXAMPLE_INPUT)), 37);
        assert_eq!(part1_memo(&input_generator(EXAMPLE_INPUT)), 37);
        assert_eq!(part1_stats(&input_generator(EXAMPLE_INPUT)), 37);
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 168);
        assert_eq!(part2_parallel(&input_generator(EXAMPLE_INPUT)), 168);
        assert_eq!(part2_memo(&input_generator(EXAMPLE_INPUT)), 168);
        assert_eq!(part2_stats(&input_generator(EXAMPLE_INPUT)), 168);
    }
}
