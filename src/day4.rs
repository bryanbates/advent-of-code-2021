use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Board {
    cells: Vec<u8>,
    id: usize,
}

impl Board {
    pub fn new(id: usize) -> Self {
        Board {
            id,
            cells: Vec::with_capacity(25),
        }
    }

    fn winp(&self, marked: &[u8]) -> bool {
        // Rows
        for r in 0..5 {
            let offset = r * 5;
            let row = [
                self.cells[offset],
                self.cells[offset + 1],
                self.cells[offset + 2],
                self.cells[offset + 3],
                self.cells[offset + 4],
            ];
            if row.iter().all(|el| marked.contains(el)) {
                println!("Found a row: {:?}", row);
                return true;
            }
        }
        // Columns
        for c in 0..5 {
            let col = [
                self.cells[c],
                self.cells[c + 5],
                self.cells[c + 10],
                self.cells[c + 15],
                self.cells[c + 20],
            ];
            if col.iter().all(|el| marked.contains(el)) {
                println!("Found a col: {:?}", col);
                return true;
            }
        }
        // // Diagonals DON'T COUNT...
        // let nwse = [self.cells[0], self.cells[6], self.cells[12], self.cells[18], self.cells[24]];
        // if nwse.iter().all(|el| marked.contains(el)) {
        //     println!("Found a nwse: {:?}", nwse);
        //     return true;
        // }
        // let nesw = [self.cells[4], self.cells[8], self.cells[12], self.cells[16], self.cells[20]];
        // if nesw.iter().all(|el| marked.contains(el)) {
        //     println!("Found a nesw: {:?}", nesw);
        //     return true;
        // }
        false
    }

    fn score(&self, marked: &[u8]) -> u32 {
        // Sum of all unmarked cells
        self.cells
            .iter()
            .copied()
            .filter(|el| !marked.contains(el))
            .map(|el| el as u32)
            .sum()
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    call: Vec<u8>,
    boards: Vec<Board>,
}

// Default implementation: Chunk on whitespace
#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Game> {
    let mut lines = input.lines();

    // Read calls
    let call = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    debug!("Found calls: {:?}", call);

    let mut game: Game = Game {
        call,
        boards: Vec::new(),
    };

    let mut board = Board::new(0);

    let mut board_id: usize = 0;

    // Read boards
    for line in lines {
        if line.trim().is_empty() {
            if !board.cells.is_empty() {
                game.boards.push(board);
            }
            board_id += 1;
            board = Board::new(board_id);
        } else {
            debug!("Parsing line: {}", line);
            board.cells.append(
                &mut line
                    .trim()
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
    }
    // Catch the last board
    if !board.cells.is_empty() {
        game.boards.push(board);
    }

    info!("GAME: {:?}", game);

    vec![game]
}

#[aoc(day4, part1)]
pub fn part1(input: &[Game]) -> u32 {
    // Bit of a hack, we just read 1 "game"
    let game = &input[0];
    for i in 0..game.call.len() {
        let this_call = game.call[i] as u32;
        debug!("ROUND: {:?}", &game.call[0..=i]);
        for board in &game.boards {
            // See if the current set of calls results in a win for any board
            let win = board.winp(&game.call[0..=i]);
            if win {
                debug!("Board: {:?}", board);
                return board.score(&game.call[0..=i]) * this_call;
            }
        }
    }
    0
}

#[aoc(day4, part2)]
pub fn part2(input: &[Game]) -> u32 {
    // Bit of a hack, we just read 1 "game"
    let game = &input[0];
    let mut won_boards: HashSet<usize> = HashSet::new();

    for i in 0..game.call.len() {
        let this_call = game.call[i] as u32;
        debug!("ROUND: {:?}", &game.call[0..=i]);
        let remaining = game
            .boards
            .iter()
            .filter(|&b| !won_boards.contains(&b.id))
            .collect::<Vec<_>>();
        for board in remaining {
            // See if the current set of calls results in a win for any board
            if board.winp(&game.call[0..=i]) {
                debug!("Board Won on this round: {:?}", board);
                let score = board.score(&game.call[0..=i]) * this_call;
                won_boards.insert(board.id);
                if won_boards.len() == game.boards.len() {
                    // This was the last board to win.
                    return score;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 4512)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 1924)
    }
}
