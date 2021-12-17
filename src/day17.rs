use std::collections::HashSet;

use std::cmp::{max, min};
use std::ops::RangeInclusive;

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
    let ddx = if dx > 0 { -1 } else { 0 };
    let vel = (dx + ddx, dy - 1);
    (pos, vel)
}

fn iterate(v0: Velocity, t: Target) -> Option<i32> {
    let mut p: Position = (0, 0);
    let mut v: Velocity = v0;

    let mut max_y = -1000;

    let mut on_target = false;

    let max_x = t.0.clone().max().unwrap();
    let min_y = t.1.clone().min().unwrap();

    // Are we past our target zone?
    while p.0 <= max_x && p.1 >= min_y {
        let res = step(p, v);
        p = res.0;
        v = res.1;
        max_y = max(max_y, p.1);
        if t.0.contains(&p.0) && t.1.contains(&p.1) {
            on_target = true;
            // println!("In target! {:?}", v0);
        }
    }

    if on_target {
        Some(max_y)
    } else {
        None
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> i32 {
    let (xrange, yrange) = parse_input(input);
    // println!("{:?}, {:?}", xrange, yrange);

    let mut max_y = -100;

    for dx in 0..=xrange.clone().max().unwrap() {
        for dy in -100..100 {
            if let Some(dv) = iterate((dx, dy), (xrange.clone(), yrange.clone())) {
                max_y = max(max_y, dv);
            }
        }
    }

    max_y
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let (xrange, yrange) = parse_input(input);
    // println!("{:?}, {:?}", xrange, yrange);

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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"target area: x=20..30, y=-10..-5
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(EXAMPLE_INPUT), 45)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(EXAMPLE_INPUT), 112)
    }
}
