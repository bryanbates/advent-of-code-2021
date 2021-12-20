use std::collections::{HashMap, HashSet};

type Point = (i32, i32);
type Image = HashMap<Point, bool>;
type Algo = HashSet<u16>;

type Puzzle = (Algo, Image, usize, usize);

fn parse_input(input: &str) -> Puzzle {
    let mut algorithm: Algo = Algo::new();
    let mut image: Image = Image::new();

    let mut lines = input.trim().split_whitespace();

    for (i, ch) in lines.next().unwrap().bytes().enumerate() {
        if ch == b'#' {
            algorithm.insert(i as u16);
        }
    }

    let mut max_r: usize = 0;
    let mut max_c: usize = 0;

    for (r, row) in lines.enumerate() {
        max_r = r;
        for (c, ch) in row.trim().bytes().enumerate() {
            max_c = c;
            image.insert((c as i32, r as i32), ch == b'#');
        }
    }

    (algorithm, image, max_c + 1, max_r + 1)
}

fn point_to_num(p: Point, img: &Image, def: bool) -> u16 {
    let (x, y) = p;
    let mut val: u16 = 0;

    val += if *img.get(&(x - 1, y - 1)).unwrap_or(&def) {
        1 << 8
    } else {
        0
    };
    val += if *img.get(&(x, y - 1)).unwrap_or(&def) {
        1 << 7
    } else {
        0
    };
    val += if *img.get(&(x + 1, y - 1)).unwrap_or(&def) {
        1 << 6
    } else {
        0
    };
    val += if *img.get(&(x - 1, y)).unwrap_or(&def) {
        1 << 5
    } else {
        0
    };
    val += if *img.get(&(x, y)).unwrap_or(&def) {
        1 << 4
    } else {
        0
    };
    val += if *img.get(&(x + 1, y)).unwrap_or(&def) {
        1 << 3
    } else {
        0
    };
    val += if *img.get(&(x - 1, y + 1)).unwrap_or(&def) {
        1 << 2
    } else {
        0
    };
    val += if *img.get(&(x, y + 1)).unwrap_or(&def) {
        1 << 1
    } else {
        0
    };
    val += if *img.get(&(x + 1, y + 1)).unwrap_or(&def) {
        1 << 0
    } else {
        0
    };

    val
}

#[allow(dead_code)]
fn render(img: &Image, cols: i32, rows: i32) {
    for y in -6..rows + 6 {
        for x in -6..cols + 6 {
            let val = img.get(&(x, y)).unwrap_or(&false);
            let ch = if *val { '#' } else { '.' };
            print!("{}", ch);
        }
        println!();
    }
}

fn lit_after_enhance(puzzle: Puzzle, steps: i32) -> usize {
    let algo = puzzle.0;
    let mut image = puzzle.1;
    let orig_cols = puzzle.2 as i32;
    let orig_rows = puzzle.3 as i32;

    let blink = algo.contains(&0);

    for i in 1..=steps {
        let mut next_image: Image = Image::new();
        // Enhance
        for x in (-i)..(orig_cols + i) {
            for y in (-i)..(orig_rows + i) {
                let val = point_to_num((x, y), &image, blink && ((i % 2) == 0));
                let enh = algo.contains(&val);
                next_image.insert((x, y), enh);
            }
        }
        image = next_image;
    }

    image.iter().filter(|(_, &v)| v).count()
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
    let puzzle = parse_input(input);

    lit_after_enhance(puzzle, 2)
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
    let puzzle = parse_input(input);

    lit_after_enhance(puzzle, 50)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(EXAMPLE_INPUT), 35)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(EXAMPLE_INPUT), 3351)
    }
}
