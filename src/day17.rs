use std::collections::HashSet;

use std::cmp::max;
use std::ops::RangeInclusive;

use itertools::Itertools;

use rayon::prelude::*;

type Position = (i32, i32);
type Velocity = (i32, i32);
type Target = (RangeInclusive<i32>, RangeInclusive<i32>);

fn parse_input(input: &str) -> Target {
    let mut xrange: RangeInclusive<i32> = 0..=0;
    let mut yrange: RangeInclusive<i32> = 0..=0;
    for chunk in input.trim().split_whitespace() {
        match chunk.chars().next().unwrap() {
            'x' => {
                let mut vals = chunk[2..].split("..");
                let start = vals
                    .next()
                    .unwrap()
                    .trim_matches(',')
                    .parse::<i32>()
                    .unwrap();
                let end = vals
                    .next()
                    .unwrap()
                    .trim_matches(',')
                    .parse::<i32>()
                    .unwrap();
                xrange = start..=end;
            }
            'y' => {
                let mut vals = chunk[2..].split("..");
                let start = vals
                    .next()
                    .unwrap()
                    .trim_matches(',')
                    .parse::<i32>()
                    .unwrap();
                let end = vals
                    .next()
                    .unwrap()
                    .trim_matches(',')
                    .parse::<i32>()
                    .unwrap();
                yrange = start..=end;
            }
            _ => {}
        }
    }
    (xrange, yrange)
}

fn step((x, y): Position, (dx, dy): Velocity) -> (Position, Velocity) {
    let pos = (x + dx, y + dy);
    let vel = (dx - dx.signum(), dy - 1);
    (pos, vel)
}

fn iterate(v0: Velocity, (tx, ty): Target) -> Option<(Velocity, i32)> {
    let mut p: Position = (0, 0);
    let mut v: Velocity = v0;

    let mut max_y = -1000;

    let max_x = tx.clone().max().unwrap();
    let min_y = ty.clone().min().unwrap();

    // Are we past our target zone?
    while p.0 <= max_x && p.1 >= min_y {
        // For some reason, can't use a destructuring bind here...
        let res = step(p, v);
        p = res.0;
        v = res.1;
        max_y = max(max_y, p.1);
        if tx.contains(&p.0) && ty.contains(&p.1) {
            // println!("In target! {:?}", v0);
            return Some((v0, max_y));
        }
    }

    None
}

#[aoc(day17, part1, loop)]
pub fn part1(input: &str) -> i32 {
    let (xrange, yrange) = parse_input(input);

    let mut max_y = -100;

    for dx in 0..=xrange.clone().max().unwrap() {
        for dy in -100..100 {
            if let Some(dv) = iterate((dx, dy), (xrange.clone(), yrange.clone())) {
                max_y = max(max_y, dv.1);
            }
        }
    }

    max_y
}

#[aoc(day17, part1, iter)]
pub fn part1_iter(input: &str) -> i32 {
    let (xrange, yrange) = parse_input(input);
    (0..=xrange.clone().max().unwrap())
        .cartesian_product(-100..100)
        .filter_map(|v| iterate(v, (xrange.clone(), yrange.clone())))
        .map(|(_, max_y)| max_y)
        .max()
        .unwrap()
}

#[aoc(day17, part1, parallel)]
pub fn part1_parallel(input: &str) -> i32 {
    let (xrange, yrange) = parse_input(input);
    (0..=xrange.clone().max().unwrap())
        .cartesian_product(-100..100)
        .par_bridge()
        .filter_map(|v| iterate(v, (xrange.clone(), yrange.clone())))
        .map(|(_, max_y)| max_y)
        .max()
        .unwrap()
}

#[aoc(day17, part1, geometry)]
pub fn part1_geometry(input: &str) -> i32 {
    let (_, yrange) = parse_input(input);
    let y_min = yrange.min().unwrap().abs();
    (y_min * (y_min - 1)) / 2
}

#[aoc(day17, part2, loop)]
pub fn part2(input: &str) -> usize {
    let (xrange, yrange) = parse_input(input);

    let mut ds: HashSet<Velocity> = HashSet::new();

    for dx in 0..=xrange.clone().max().unwrap() {
        for dy in -100..100 {
            if iterate((dx, dy), (xrange.clone(), yrange.clone())).is_some() {
                ds.insert((dx, dy));
            }
        }
    }

    ds.len()
}

#[aoc(day17, part2, iter)]
pub fn part2_iter(input: &str) -> usize {
    let (xrange, yrange) = parse_input(input);
    (0..=xrange.clone().max().unwrap())
        .cartesian_product(-100..100)
        .filter_map(|v| iterate(v, (xrange.clone(), yrange.clone())))
        .map(|(v, _)| v)
        .collect::<HashSet<_>>()
        .len()
}

#[aoc(day17, part2, parallel)]
pub fn part2_parallel(input: &str) -> usize {
    let (xrange, yrange) = parse_input(input);
    (0..=xrange.clone().max().unwrap())
        .cartesian_product(-100..100)
        .par_bridge()
        .filter_map(|v| iterate(v, (xrange.clone(), yrange.clone())))
        .map(|(v, _)| v)
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"target area: x=20..30, y=-10..-5
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(EXAMPLE_INPUT), 45);
        assert_eq!(part1_iter(EXAMPLE_INPUT), 45);
        assert_eq!(part1_parallel(EXAMPLE_INPUT), 45);
        assert_eq!(part1_geometry(EXAMPLE_INPUT), 45);
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(EXAMPLE_INPUT), 112);
        assert_eq!(part2_iter(EXAMPLE_INPUT), 112);
        assert_eq!(part2_parallel(EXAMPLE_INPUT), 112);
    }
}
