use std::collections::HashSet;

type Point = (i32, i32);
type Fold = (char, i32);

#[derive(Default)]
struct Manual {
    dots: Vec<Point>,
    folds: Vec<Fold>,
}

fn parse_input(input: &str) -> Manual {
    let mut man: Manual = Default::default();

    for line in input.lines() {
        let l = line.trim();
        if l.contains(',') {
            // It's a point
            let mut pl = l.split(',');
            let x = pl.next().unwrap().parse::<i32>().unwrap();
            let y = pl.next().unwrap().parse::<i32>().unwrap();
            man.dots.push((x, y));
        } else if l.contains('=') {
            // It's a fold
            let mut fl = l[11..].split('=');
            let axis = fl.next().unwrap().chars().next().unwrap();
            let val = fl.next().unwrap().parse::<i32>().unwrap();
            man.folds.push((axis, val));
        }
    }
    man
}

fn fold(dots: Vec<Point>, fold: Fold) -> Vec<Point> {
    let mut result: HashSet<Point> = HashSet::new();

    for (x, y) in dots {
        if fold.0 == 'x' {
            // Fold along X, so set x = fold - abs(x-fold)
            let dist = (x - fold.1).abs();
            result.insert((fold.1 - dist, y));
        } else if fold.0 == 'y' {
            // Fold along Y, so set y = fold - abs(y-fold)
            let dist = (y - fold.1).abs();
            result.insert((x, fold.1 - dist));
        }
    }

    result.drain().collect::<Vec<_>>()
}

fn render(dots: &[Point]) {
    let max_x = dots.iter().map(|p| p.0).max().unwrap();
    let max_y = dots.iter().map(|p| p.1).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let val = if dots.contains(&(x, y)) { '#' } else { ' ' };
            print!("{}", val);
        }
        println!();
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    let man = parse_input(input);
    let dots = fold(man.dots, man.folds[0]);
    dots.len()
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> usize {
    let mut man = parse_input(input);
    man.dots = man.folds.iter().fold(man.dots, |dots, &f| fold(dots, f));
    render(&man.dots);
    man.dots.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(EXAMPLE_INPUT), 17)
    }
}
