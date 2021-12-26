use fxhash::FxHashSet;

type Point = (u8, u8);

#[derive(Debug, PartialEq)]
struct Puzzle {
    east: FxHashSet<Point>,
    south: FxHashSet<Point>,
    cols: u8,
    rows: u8,
}

fn parse_input(input: &str) -> Puzzle {
    let mut east: FxHashSet<Point> = FxHashSet::default();
    let mut south: FxHashSet<Point> = FxHashSet::default();
    let mut max_r: usize = 0;
    let mut max_c: usize = 0;
    for (r, row) in input.trim().split_whitespace().enumerate() {
        // println!("{}, {:?}", r, row);
        max_r = r;
        for (c, ch) in row.trim().bytes().enumerate() {
            // println!("{}, {} = {}", r, c, ch);
            max_c = c;
            match ch {
                b'>' => {
                    east.insert((c as u8, r as u8));
                }
                b'v' => {
                    south.insert((c as u8, r as u8));
                }
                _ => {}
            }
        }
    }

    Puzzle {
        east,
        south,
        cols: (max_c + 1) as u8,
        rows: (max_r + 1) as u8,
    }
}

fn step(puzzle: &Puzzle) -> (Puzzle, u32) {
    let mut east1: FxHashSet<Point> = FxHashSet::default();
    let mut south1: FxHashSet<Point> = FxHashSet::default();

    let mut moves = 0_u32;
    // Move East
    for x in (0..puzzle.cols).rev() {
        for y in (0..puzzle.rows).rev() {
            if puzzle.east.contains(&(x, y)) {
                let loc = ((x + 1) % puzzle.cols, y);
                // This snail moves if there's an open spot to move into, in the trench
                if !puzzle.east.contains(&loc) && !puzzle.south.contains(&loc) {
                    moves += 1;
                    east1.insert(loc);
                } else {
                    // This snail doesn't move.
                    east1.insert((x, y));
                }
            }
        }
    }
    // Move South
    for x in (0..puzzle.cols).rev() {
        for y in (0..puzzle.rows).rev() {
            if puzzle.south.contains(&(x, y)) {
                let loc = (x, (y + 1) % puzzle.rows);
                // This snail moves if there's an open spot to move into, in the trench
                if !east1.contains(&loc) && !puzzle.south.contains(&loc) {
                    moves += 1;
                    south1.insert(loc);
                } else {
                    // This snail doesn't move.
                    south1.insert((x, y));
                }
            }
        }
    }

    (
        Puzzle {
            east: east1,
            south: south1,
            cols: puzzle.cols,
            rows: puzzle.rows,
        },
        moves,
    )
}

#[allow(dead_code)]
fn render_map(puzzle: &Puzzle) {
    for y in 0..puzzle.rows {
        for x in 0..puzzle.cols {
            let ch = if puzzle.east.contains(&(x, y)) {
                '>'
            } else if puzzle.south.contains(&(x, y)) {
                'v'
            } else {
                '.'
            };
            print!("{}", ch);
        }
        println!();
    }
}

#[aoc(day25, part1)]
pub fn part1(input: &str) -> u32 {
    let mut puzzle = parse_input(input);

    // render_map(&puzzle);
    // println!("--------------------------------");

    let mut steps = 0_u32;
    loop {
        steps += 1;
        let (nxt, moves) = step(&puzzle);
        puzzle = nxt;
        // println!("STEP {}", steps);
        // render_map(&puzzle);
        // println!("--------------------------------");
        if moves == 0 {
            break;
        }
    }
    steps
}

// #[aoc(day25, part2)]
// pub fn part2(input: &str) -> u32 {
//     let puzzle = parse_input(input);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
"#;

    const EX_STEP_1: &str = r#"....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(EXAMPLE_INPUT), 58)
    }

    #[test]
    fn step_1() {
        let p = parse_input(EXAMPLE_INPUT);
        let expected = parse_input(EX_STEP_1);
        let (nxt, _) = step(&p);
        assert_eq!(nxt, expected);
    }

    // #[test]
    // fn part2_ex1() {
    //     assert_eq!(part2(EXAMPLE_INPUT), 6)
    // }
}
