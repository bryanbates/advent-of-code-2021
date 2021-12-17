use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::ops::RangeInclusive;

type Point = (usize, usize);
type Grid = HashMap<Point, u32>;

fn parse_input(input: &str) -> (Grid, usize, usize) {
    let mut grid: Grid = HashMap::new();
    let mut max_r: usize = 0;
    let mut max_c: usize = 0;
    for (r, row) in input.trim().split_whitespace().enumerate() {
        // println!("{}, {:?}", r, row);
        max_r = r;
        for (c, ch) in row.trim().bytes().enumerate() {
            // println!("{}, {} = {}", r, c, ch);
            max_c = c;
            grid.insert((c, r), (ch - b'0') as u32);
        }
    }

    (grid, max_c + 1, max_r + 1)
}

fn axis_range(n: usize, extent: usize) -> RangeInclusive<usize> {
    max(0, n.saturating_sub(1))..=min(extent.saturating_sub(1), n + 1)
}

fn neighbors((x, y): Point, cols: usize, rows: usize) -> Vec<Point> {
    let mut ns: Vec<Point> = vec![];
    for xs in axis_range(x, cols) {
        for ys in axis_range(y, rows) {
            if (xs == x) ^ (ys == y) {
                ns.push((xs, ys));
            }
        }
    }
    ns
}

fn dijkstra(grid: Grid, start: Point, end: Point, cols: usize, rows: usize) -> u32 {
    // TEXTBOOK?

    let mut dist: HashMap<Point, u32> = HashMap::new();
    let mut prev: HashMap<Point, Point> = HashMap::new();

    for p in grid.keys() {
        dist.insert(*p, 10 * 500 * 500);
    }

    dist.insert(start, 0);

    // Reverse because we need a min-heap
    let mut pq: BinaryHeap<(Reverse<u32>, Point)> =
        dist.iter().map(|(&k, &v)| (Reverse(v), k)).collect();

    while !pq.is_empty() {
        let (_, p) = pq.pop().unwrap();
        let dist_to_p = *dist.get(&p).unwrap();
        for n in neighbors(p, cols, rows) {
            let dist_from_p_to_n = *grid.get(&n).unwrap();
            let dist_to_n = *dist.get(&n).unwrap();
            let dist_to_n_via_p = dist_to_p + dist_from_p_to_n;
            if dist_to_n > dist_to_n_via_p {
                // Found a shorter path to n via p
                dist.insert(n, dist_to_n_via_p);
                prev.insert(n, p);
                // Need to update the entry for n in the heap,
                // because we found a shorter path...
                pq.push((Reverse(dist_to_n_via_p), n));
            }
        }
    }

    // Now read off the prev, starting with the destination
    let mut cur = end;
    let mut cost: u32 = 0;

    while cur != start {
        // println!("Path includes {:?}", cur);
        cost += *grid.get(&cur).unwrap();
        cur = *prev.get(&cur).unwrap();
    }

    cost
}

#[allow(dead_code)]
fn render_map(grid: &Grid, cols: usize, rows: usize) {
    for y in 0..rows {
        for x in 0..cols {
            let val = grid.get(&(x, y)).unwrap();
            let ch = (*val as u8 + b'0') as char;
            print!("{}", ch);
        }
        println!();
    }
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> u32 {
    let (grid, cols, rows) = parse_input(input);
    dijkstra(grid, (0, 0), (cols - 1, rows - 1), cols, rows)
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> u32 {
    let (grid, cols, rows) = parse_input(input);
    let mut expanded: Grid = HashMap::new();

    for x in 0..cols * 5 {
        for y in 0..rows * 5 {
            let orig_x = x % cols;
            let orig_y = y % rows;
            let copy_x = x / cols;
            let copy_y = y / rows;

            let orig_val = *grid.get(&(orig_x, orig_y)).unwrap();
            let mut val = orig_val + copy_x as u32 + copy_y as u32;
            while val > 9 {
                val -= 9;
            }

            expanded.insert((x, y), val);
        }
    }

    // render_map(&expanded, cols*5, rows*5);

    dijkstra(
        expanded,
        (0, 0),
        (cols * 5 - 1, rows * 5 - 1),
        cols * 5,
        rows * 5,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(EXAMPLE_INPUT), 40)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(EXAMPLE_INPUT), 315)
    }
}
