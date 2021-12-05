use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Line {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

// Default implementation: Chunk on whitespace
#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let mut toks = l.trim().split(' ');
            // 2 coords separated by ->
            let coord1 = toks
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            // skip the ->
            toks.next();
            let coord2 = toks
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            Line {
                x0: coord1[0],
                y0: coord1[1],
                x1: coord2[0],
                y1: coord2[1],
            }
        })
        .collect()
}

fn step_from(x0: i32, x1: i32) -> i32 {
    match x0.cmp(&x1) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

fn draw(input: &[&Line]) -> HashMap<(i32, i32), i32> {
    let mut dwg = HashMap::new();
    for &line in input {
        let xstep = step_from(line.x0, line.x1);
        let ystep = step_from(line.y0, line.y1);

        let mut x = line.x0;
        let mut y = line.y0;

        println!(
            "Drawing {:?} starting at ({}, {}) by steps {}, {}",
            line, x, y, xstep, ystep
        );

        loop {
            // Draw at point
            *dwg.entry((x, y)).or_insert(0) += 1;

            // If we drew the last point, break
            if x == line.x1 && y == line.y1 {
                break;
            }

            // And step
            x += xstep;
            y += ystep;
        }
    }
    dwg
}

#[aoc(day5, part1)]
pub fn part1(input: &[Line]) -> usize {
    let lines = input
        .iter()
        .filter(|&l| l.x0 == l.x1 || l.y0 == l.y1)
        .collect::<Vec<_>>();
    let dwg = draw(&lines);
    dwg.values().filter(|&&v| v >= 2).count()
}

#[aoc(day5, part2)]
pub fn part2(input: &[Line]) -> usize {
    let lines = input
        .iter()
        .filter(|&l| l.x0 == l.x1 || l.y0 == l.y1 || ((l.y1 - l.y0).abs() == (l.x1 - l.x0).abs()))
        .collect::<Vec<_>>();
    let dwg = draw(&lines);
    dwg.values().filter(|&&v| v >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 5)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 12)
    }
}
