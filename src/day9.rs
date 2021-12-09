use std::collections::HashSet;
use std::collections::VecDeque;

const ROWS: usize = 100;
const COLS: usize = 100;

// const ROWS: usize = 5;
// const COLS: usize = 10;

type Grid = [[u8; ROWS]; COLS];
type Point = (usize, usize);

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

fn lowcations(grid: Grid) -> Vec<Point> {
    let mut lows: Vec<Point> = vec![];

    for x in 0..COLS {
        for y in 0..ROWS {
            let val = grid[x][y];

            if neighbors_of((x, y))
                .iter()
                .all(|(xx, yy)| val < grid[*xx][*yy])
            {
                lows.push((x, y));
            }
        }
    }

    lows
}

fn neighbors_of(p: Point) -> Vec<Point> {
    let mut neighbors: Vec<Point> = vec![];

    let (x, y) = p;

    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < COLS - 1 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < ROWS - 1 {
        neighbors.push((x, y + 1));
    }
    neighbors
}

fn basin_size(grid: Grid, low: Point) -> u32 {
    let mut marked: HashSet<Point> = HashSet::new();
    let mut todo: VecDeque<Point> = VecDeque::new();

    todo.push_back(low);

    while !todo.is_empty() {
        let (x, y) = todo.pop_front().unwrap();
        marked.insert((x, y));
        // Not 9 and not marked? add it to the size of this basin
        for (nx, ny) in neighbors_of((x, y))
            .iter()
            .filter(|&p| !marked.contains(p))
            .filter(|(xx, yy)| grid[*xx][*yy] != 9)
        {
            todo.push_back((*nx, *ny));
        }
    }

    marked.len() as u32
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u32 {
    let grid = parse_input(input);

    lowcations(grid)
        .iter()
        .map(|(x, y)| grid[*x][*y] + 1)
        .fold(0_u32, |acc, v| acc + v as u32)
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u32 {
    let grid = parse_input(input);
    let mut basins = lowcations(grid)
        .iter()
        .map(|p| basin_size(grid, *p))
        .collect::<Vec<u32>>();
    basins.sort_unstable();
    // println!("{:?}", basins);
    basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(EXAMPLE_INPUT), 15)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(EXAMPLE_INPUT), 1134)
    }
}
