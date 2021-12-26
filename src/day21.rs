extern crate fxhash;

use fxhash::FxHashMap;

type Die = (u32, u64);

fn roll_d100(die: Die) -> Die {
    // println!("Rolling: {:?}", die);
    ((die.0 + 1) % 100, die.1 + 1)
}

type Puzzle = (u32, u32);

fn parse_input(input: &str) -> Puzzle {
    let mut positions = input
        .trim()
        .split('\n')
        .map(|l| l.rsplit(':').next().unwrap().trim().parse::<u32>().unwrap());
    (positions.next().unwrap(), positions.next().unwrap())
}

fn move_pawn(pos: u32, roll: u32) -> u32 {
    // println!("Moving: {}, {}", pos, roll);
    let mut nxt = pos + roll;
    while nxt > 10 {
        nxt -= 10;
    }
    nxt
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> u64 {
    let (mut p1, mut p2) = parse_input(input);

    let mut s1: u64 = 0;
    let mut s2: u64 = 0;

    let mut die: Die = (0, 0);

    loop {
        // p1 rolls 3x
        die = roll_d100(die);
        p1 = move_pawn(p1, die.0);
        die = roll_d100(die);
        p1 = move_pawn(p1, die.0);
        die = roll_d100(die);
        p1 = move_pawn(p1, die.0);

        s1 += p1 as u64;
        if s1 >= 1000 {
            break;
        }

        die = roll_d100(die);
        p2 = move_pawn(p2, die.0);
        die = roll_d100(die);
        p2 = move_pawn(p2, die.0);
        die = roll_d100(die);
        p2 = move_pawn(p2, die.0);
        s2 += p2 as u64;
        if s2 >= 1000 {
            break;
        }
    }

    // println!("Scores: {}, {}  Die: {:?}", s1, s2, die);

    if s1 > s2 {
        // s2 is the loser
        s2 * die.1
    } else {
        // s1 is the loser
        s1 * die.1
    }
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> u64 {
    let (p1_start, p2_start) = parse_input(input);

    // p1 pos, p1 score, p2 pos, p2 score -> count of multiverses
    let mut verse: FxHashMap<(u8, u8, u8, u8), u64> = FxHashMap::default();
    let mut alt: FxHashMap<(u8, u8, u8, u8), u64> = FxHashMap::default();

    let mut wins_p1: u64 = 0;
    let mut wins_p2: u64 = 0;

    let steps: [(u8, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    // init
    verse.insert((p1_start as u8 - 1, 0, p2_start as u8 - 1, 0), 1);

    while !verse.is_empty() {
        // println!("VERSES: {:?}", verse.len());
        // P1 moves
        for (game, count) in verse.drain() {
            for step in steps {
                let p1_next = (game.0 + step.0) % 10;
                let s1_next = game.1 + p1_next + 1;
                if s1_next >= 21 {
                    wins_p1 += count * step.1;
                } else {
                    *alt.entry((p1_next, s1_next, game.2, game.3)).or_insert(0) += count * step.1;
                }
            }
        }

        // P2 moves
        for (game, count) in alt.drain() {
            for step in steps {
                let p2_next = (game.2 + step.0) % 10;
                let s2_next = game.3 + p2_next + 1;
                if s2_next >= 21 {
                    wins_p2 += count * step.1;
                } else {
                    *verse.entry((game.0, game.1, p2_next, s2_next)).or_insert(0) += count * step.1;
                }
            }
        }
    }

    // println!("VERSE: {:?}", verse);
    // println!("Wins: {}, {}", wins_p1, wins_p2);

    if wins_p1 > wins_p2 {
        wins_p1
    } else {
        wins_p2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"Player 1 starting position: 4
Player 2 starting position: 8
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(EXAMPLE_INPUT), 739785)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(EXAMPLE_INPUT), 444356092776315)
    }
}
