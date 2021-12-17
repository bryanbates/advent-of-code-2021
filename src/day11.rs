use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::RangeInclusive;

const ROWS: usize = 10;
const COLS: usize = 10;

type Grid = [[u8; ROWS]; COLS];
type Point = (usize, usize);
type GridMap = HashMap<Point, u8>;

fn parse_input(input: &str) -> Grid {
    let mut grid = [[0_u8; ROWS]; COLS];
    for (r, row) in input.trim().split_whitespace().enumerate() {
        // println!("{}, {:?}", r, row);
        for (c, ch) in row.trim().bytes().enumerate() {
            // println!("{}, {} = {}", r, c, ch);
            grid[c][r] = ch - b'0';
        }
    }
    grid
}

fn parse_input_map(input: &str) -> GridMap {
    let mut grid: GridMap = HashMap::new();
    for (r, row) in input.trim().split_whitespace().enumerate() {
        // println!("{}, {:?}", r, row);
        for (c, ch) in row.trim().bytes().enumerate() {
            // println!("{}, {} = {}", r, c, ch);
            grid.insert((c, r), ch - b'0');
        }
    }

    grid
}

fn axis_range(n: usize, extent: usize) -> RangeInclusive<usize> {
    max(0, n.saturating_sub(1))..=min(extent.saturating_sub(1), n + 1)
}

fn neighbors((x, y): Point) -> Vec<Point> {
    let mut ns: Vec<Point> = vec![];
    for xs in axis_range(x, COLS) {
        for ys in axis_range(y, ROWS) {
            if (xs, ys) != (x, y) {
                ns.push((xs, ys));
            }
        }
    }
    ns
}

fn neighbors_of(p: Point) -> Vec<Point> {
    let mut neighbors: Vec<Point> = vec![];

    let (x, y) = p;

    if x > 0 {
        // W
        neighbors.push((x - 1, y));
        if y > 0 {
            // NW
            neighbors.push((x - 1, y - 1));
        }
        if y < ROWS - 1 {
            // SW
            neighbors.push((x - 1, y + 1));
        }
    }
    if x < COLS - 1 {
        // E
        neighbors.push((x + 1, y));
        if y > 0 {
            // NE
            neighbors.push((x + 1, y - 1));
        }
        if y < ROWS - 1 {
            // SE
            neighbors.push((x + 1, y + 1));
        }
    }
    if y > 0 {
        // N
        neighbors.push((x, y - 1));
    }
    if y < ROWS - 1 {
        // S
        neighbors.push((x, y + 1));
    }
    neighbors
}

fn step(grid: &mut Grid) -> usize {
    // energy++
    #[allow(clippy::needless_range_loop)]
    for x in 0..COLS {
        for y in 0..ROWS {
            grid[x][y] += 1;
        }
    }

    let mut flashed: HashSet<Point> = HashSet::new();

    // FLASH
    loop {
        // println!("FLASHES THIS TURN: {:?}", flashed);
        // println!("GRID: {:?}", grid);
        // Stop if there's no octopus with enough energy left to flash
        let mut again = false;
        #[allow(clippy::needless_range_loop)]
        for x in 0..COLS {
            for y in 0..ROWS {
                if grid[x][y] > 9 {
                    again = true;
                }
            }
        }
        if !again {
            // println!("NONE LEFT: {:?}", grid);
            break;
        }

        for x in 0..COLS {
            for y in 0..ROWS {
                if grid[x][y] > 9 {
                    // println!("FLASHING: {}, {} = {}", x, y, grid[x][y]);
                    flashed.insert((x, y));
                    grid[x][y] = 0;
                    // FLASH!
                    neighbors_of((x, y)).iter().for_each(|(nx, ny)| {
                        // If it didn't flash this turn, it gets energy
                        if !flashed.contains(&(*nx, *ny)) {
                            grid[*nx][*ny] += 1;
                        }
                    });
                }
            }
        }
    }

    flashed.len()
}

#[allow(dead_code)]
fn render_map(grid: &GridMap) {
    for y in 0..ROWS {
        for x in 0..COLS {
            let val = grid.get(&(x, y)).unwrap();
            let ch = (*val + b'0') as char;
            print!("{}", ch);
        }
        println!();
    }
}

fn step_map(grid: &mut GridMap) -> usize {
    // energy++
    for val in grid.values_mut() {
        *val += 1;
    }

    let mut flashed: HashSet<Point> = HashSet::new();
    let mut to_flash: VecDeque<Point> = VecDeque::new();

    for ((x, y), v) in grid.iter() {
        if *v > 9 {
            to_flash.push_back((*x, *y));
        }
    }

    while !to_flash.is_empty() {
        // println!("To flash: {:?}", to_flash);
        let (x, y) = to_flash.pop_front().unwrap();
        if flashed.contains(&(x, y)) {
            continue;
        }

        flashed.insert((x, y));

        // Increment neighbors
        for (nx, ny) in neighbors((x, y)) {
            // println!("Incrementing neighbors of {}, {}", nx, ny);
            let val = grid.get_mut(&(nx, ny)).unwrap();
            *val += 1;
            if *val > 9 {
                to_flash.push_back((nx, ny));
            }
        }
    }

    for (x, y) in &flashed {
        *grid.get_mut(&(*x, *y)).unwrap() = 0;
    }

    flashed.len()
}

#[aoc(day11, part1, twodee)]
pub fn part1(input: &str) -> usize {
    let mut grid = parse_input(input);
    (0..100).map(|_| step(&mut grid)).sum::<usize>()
}

#[aoc(day11, part1, map)]
pub fn part1_map(input: &str) -> usize {
    let mut grid = parse_input_map(input);
    (0..100).map(|_| step_map(&mut grid)).sum::<usize>()
}

#[aoc(day11, part2, twodee)]
pub fn part2(input: &str) -> usize {
    let mut grid = parse_input(input);
    for i in 1..1000 {
        let flashes = step(&mut grid);
        if flashes >= 100 {
            return i;
        }
    }

    0
}

#[aoc(day11, part2, map)]
pub fn part2_map(input: &str) -> usize {
    let mut grid = parse_input_map(input);
    for i in 1..1000 {
        let flashes = step_map(&mut grid);
        if flashes >= 100 {
            return i;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;

    const EXAMPLE_STEP1: &str = r#"6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637
"#;
    const EXAMPLE_STEP2: &str = r#"8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848
"#;

    const EXAMPLE_STEP10: &str = r#"0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000
"#;

    #[test]
    fn part1_ex1_1step() {
        let mut grid = parse_input(EXAMPLE_INPUT);
        let expected_step1 = parse_input(EXAMPLE_STEP1);

        let flashes = step(&mut grid);

        assert_eq!(flashes, 0);
        assert_eq!(grid, expected_step1);
    }

    #[test]
    fn part1_ex1_2steps() {
        let mut grid = parse_input(EXAMPLE_INPUT);
        let expected_step1 = parse_input(EXAMPLE_STEP1);
        let expected_step2 = parse_input(EXAMPLE_STEP2);

        let flashes_1 = step(&mut grid);
        assert_eq!(grid, expected_step1);
        assert_eq!(flashes_1, 0);

        let flashes_2 = step(&mut grid);
        assert_eq!(grid, expected_step2);
        assert_eq!(flashes_2, 35);
    }

    #[test]
    fn part1_ex1_10steps() {
        let mut grid = parse_input(EXAMPLE_INPUT);
        let expected_step10 = parse_input(EXAMPLE_STEP10);

        let mut flashes = 0;
        for _ in 0..10 {
            flashes += step(&mut grid);
        }

        assert_eq!(grid, expected_step10);
        assert_eq!(flashes, 204);
    }

    #[test]
    fn part1_ex1_1step_map() {
        let mut grid = parse_input_map(EXAMPLE_INPUT);
        let expected_step1 = parse_input_map(EXAMPLE_STEP1);

        let flashes = step_map(&mut grid);

        assert_eq!(flashes, 0);
        assert_eq!(grid, expected_step1);
    }

    #[test]
    fn part1_ex1_2steps_map() {
        let mut grid = parse_input_map(EXAMPLE_INPUT);
        let expected_step1 = parse_input_map(EXAMPLE_STEP1);
        let expected_step2 = parse_input_map(EXAMPLE_STEP2);

        let flashes_1 = step_map(&mut grid);
        assert_eq!(grid, expected_step1);
        assert_eq!(flashes_1, 0);

        let flashes_2 = step_map(&mut grid);
        render_map(&grid);
        assert_eq!(grid, expected_step2);
        assert_eq!(flashes_2, 35);
    }

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(EXAMPLE_INPUT), 1656);
        assert_eq!(part1_map(EXAMPLE_INPUT), 1656);
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(EXAMPLE_INPUT), 195);
        assert_eq!(part2_map(EXAMPLE_INPUT), 195);
    }

    #[test]
    fn neighbor_range() {
        assert_eq!(neighbors((0, 0)), vec![(0, 1), (1, 0), (1, 1)]);
        assert_eq!(neighbors((9, 9)), vec![(8, 8), (8, 9), (9, 8)]);
        assert_eq!(neighbors((0, 9)), vec![(0, 8), (1, 8), (1, 9)]);
        assert_eq!(neighbors((9, 0)), vec![(8, 0), (8, 1), (9, 1)]);

        assert_eq!(
            neighbors((5, 5)),
            vec![
                (4, 4),
                (4, 5),
                (4, 6),
                (5, 4),
                (5, 6),
                (6, 4),
                (6, 5),
                (6, 6)
            ]
        );
    }
}
