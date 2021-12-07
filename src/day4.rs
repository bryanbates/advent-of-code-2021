#[derive(Debug, Clone)]
pub struct Board {
    cells: Vec<u8>,
    score: u32,
}

impl Board {
    pub fn new() -> Self {
        Board {
            score: 0,
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
                debug!("Found a row: {:?}", row);
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
                debug!("Found a col: {:?}", col);
                return true;
            }
        }
        // // Diagonals DON'T COUNT...
        // let nwse = [self.cells[0], self.cells[6], self.cells[12], self.cells[18], self.cells[24]];
        // if nwse.iter().all(|el| marked.contains(el)) {
        //     debug!("Found a nwse: {:?}", nwse);
        //     return true;
        // }
        // let nesw = [self.cells[4], self.cells[8], self.cells[12], self.cells[16], self.cells[20]];
        // if nesw.iter().all(|el| marked.contains(el)) {
        //     debug!("Found a nesw: {:?}", nesw);
        //     return true;
        // }
        false
    }

    fn solve(&mut self, calls: &[u8]) {
        let mut score: u32 = self.cells.iter().map(|&el| el as u32).sum();
        let mut marks = Vec::<u8>::new();

        for &round in calls {
            score -= round as u32;
            marks.push(round);
            if self.winp(&marks) {
                self.score = score * (round as u32);
                break;
            }
        }
    }
}

// Default implementation: Chunk on whitespace
#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Board> {
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

    let mut boards: Vec<Board> = Vec::new();

    let mut board = Board::new();

    // Read boards
    for line in lines {
        if line.trim().is_empty() {
            if !board.cells.is_empty() {
                // Go ahead and solve it
                board.solve(&call);
                boards.push(board);
            }
            board = Board::new();
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
        // Go ahead and solve it
        board.solve(&call);
        boards.push(board);
    }

    // We want the first one to be the highest score
    boards.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    boards
}

#[aoc(day4, part1)]
pub fn part1(input: &[Board]) -> u32 {
    input.first().unwrap().score
}

#[aoc(day4, part2)]
pub fn part2(input: &[Board]) -> u32 {
    input.last().unwrap().score
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
