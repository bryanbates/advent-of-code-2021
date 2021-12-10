#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.trim().lines().map(|l| l.trim().to_owned()).collect()
}

fn score_illegal_char(ch: char) -> u64 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn score_completion_char(ch: char) -> u64 {
    match ch {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}

fn char_pair(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => ch,
    }
}

fn score_corrupt_line(line: &str) -> u64 {
    let mut stack: Vec<char> = Vec::new();
    // println!("Line: {:?}", line);
    for ch in line.chars() {
        // println!("{}, Stack: {:?}", ch, stack);
        match ch {
            '{' | '(' | '[' | '<' => {
                stack.push(ch);
            }
            '}' | ')' | ']' | '>' => {
                let expect = char_pair(stack.pop().unwrap());
                if ch != expect {
                    // println!("Corrupt!  Expected {}, but found {} instead.", expect, ch);
                    return score_illegal_char(ch);
                }
            }
            _ => unreachable!(),
        }
    }

    // if !stack.is_empty() {
    //     println!("Incomplete, but that's ok for now?");
    // }
    0
}

fn score_incomplete_line(line: &str) -> u64 {
    let mut stack: Vec<char> = Vec::new();
    // println!("Line: {:?}", line);
    for ch in line.chars() {
        // println!("{}, Stack: {:?}", ch, stack);
        match ch {
            '{' | '(' | '[' | '<' => {
                stack.push(ch);
            }
            '}' | ')' | ']' | '>' => {
                let expect = char_pair(stack.pop().unwrap());
                if ch != expect {
                    // println!("Corrupt!  Expected {}, but found {} instead.", expect, ch);
                    return 0;
                }
            }
            _ => unreachable!(),
        }
    }

    if !stack.is_empty() {
        // println!("Incomplete, stack: {:?}", stack);
        let mut score: u64 = 0;

        while let Some(ch) = stack.pop() {
            score *= 5;
            score += score_completion_char(ch);
        }
        return score;
    }
    0
}

#[aoc(day10, part1)]
pub fn part1(input: &[String]) -> u64 {
    input
        .iter()
        .map(|line| score_corrupt_line(line))
        .filter(|&s| s != 0)
        .sum::<u64>()
}

#[aoc(day10, part2)]
pub fn part2(input: &[String]) -> u64 {
    statistical::median(
        &input
            .iter()
            .map(|line| score_incomplete_line(line))
            .filter(|&s| s != 0)
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 26397)
    }

    #[test]
    fn part1_ex1a() {
        assert_eq!(score_corrupt_line("{([(<{}[<>[]}>{[]{[(<()>"), 1197)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 288957)
    }

    #[test]
    fn part2_ex1a() {
        assert_eq!(score_incomplete_line("[({(<(())[]>[[{[]{<()<>>"), 288957);
    }

    #[test]
    fn part2_ex1b() {
        assert_eq!(score_incomplete_line("[(()[<>])]({[<{<<[]>>("), 5566);
    }
    #[test]
    fn part2_ex1c() {
        assert_eq!(score_incomplete_line("(((({<>}<{<{<>}{[]{[]{}"), 1480781);
    }
    #[test]
    fn part2_ex1d() {
        assert_eq!(score_incomplete_line("{<[[]]>}<{[{[{[]{()[[[]"), 995444);
    }
    #[test]
    fn part2_ex1e() {
        assert_eq!(score_incomplete_line("<{([{{}}[<[[[<>{}]]]>[]]"), 294);
    }
}
