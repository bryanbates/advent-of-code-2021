#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.trim().lines().map(|l| l.trim().to_owned()).collect()
}

fn score_char(ch: char) -> u64 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn score_char_2(ch: char) -> u64 {
    match ch {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}

fn score_line(line: &str) -> u64 {
    let mut stack: Vec<char> = Vec::new();
    println!("Line: {:?}", line);
    for ch in line.chars() {
        // println!("{}, Stack: {:?}", ch, stack);
        match ch {
            '{' | '(' | '[' | '<' => {
                stack.push(ch);
            }
            '}' | ')' | ']' | '>' => {
                match stack.pop() {
                    Some(x) => {
                        match x {
                            '(' => {
                                if ch != ')' {
                                    // Corrupt!
                                    println!("Expected ), but found {} instead.", ch);
                                    return score_char(ch);
                                }
                            }
                            '[' => {
                                if ch != ']' {
                                    // Corrupt!
                                    println!("Expected ], but found {} instead.", ch);
                                    return score_char(ch);
                                }
                            }
                            '{' => {
                                if ch != '}' {
                                    // Corrupt!
                                    println!("Expected }}, but found {} instead.", ch);
                                    return score_char(ch);
                                }
                            }
                            '<' => {
                                if ch != '>' {
                                    // Corrupt!
                                    println!("Expected >, but found {} instead.", ch);
                                    return score_char(ch);
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    None => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    // if !stack.is_empty() {
    //     // println!("Incomplete, but that's ok for now?");
    // }
    0
}

fn score_line_2(line: &str) -> u64 {
    let mut stack: Vec<char> = Vec::new();
    println!("Line: {:?}", line);
    for ch in line.chars() {
        // println!("{}, Stack: {:?}", ch, stack);
        match ch {
            '{' | '(' | '[' | '<' => {
                stack.push(ch);
            }
            '}' | ')' | ']' | '>' => {
                match stack.pop() {
                    Some(x) => {
                        match x {
                            '(' => {
                                if ch != ')' {
                                    // Corrupt!
                                    return 0;
                                }
                            }
                            '[' => {
                                if ch != ']' {
                                    // Corrupt!
                                    return 0;
                                }
                            }
                            '{' => {
                                if ch != '}' {
                                    // Corrupt!
                                    return 0;
                                }
                            }
                            '<' => {
                                if ch != '>' {
                                    // Corrupt!
                                    return 0;
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    None => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    if !stack.is_empty() {
        println!("Incomplete, stack: {:?}", stack);
        let mut score: u64 = 0;

        while let Some(ch) = stack.pop() {
            score *= 5;
            score += score_char_2(ch);
        }
        return score;
    }
    0
}

#[aoc(day10, part1)]
pub fn part1(input: &[String]) -> u64 {
    let corrupt = input
        .iter()
        .map(|line| score_line(line))
        .filter(|&s| s != 0)
        .collect::<Vec<_>>();
    corrupt.iter().sum::<u64>()
}

#[aoc(day10, part2)]
pub fn part2(input: &[String]) -> u64 {
    statistical::median(
        &input
            .iter()
            .map(|line| score_line_2(line))
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
        assert_eq!(score_line("{([(<{}[<>[]}>{[]{[(<()>"), 1197)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 288957)
    }

    #[test]
    fn part2_ex1a() {
        assert_eq!(score_line_2("[({(<(())[]>[[{[]{<()<>>"), 288957);
    }

    #[test]
    fn part2_ex1b() {
        assert_eq!(score_line_2("[(()[<>])]({[<{<<[]>>("), 5566);
    }
    #[test]
    fn part2_ex1c() {
        assert_eq!(score_line_2("(((({<>}<{<{<>}{[]{[]{}"), 1480781);
    }
    #[test]
    fn part2_ex1d() {
        assert_eq!(score_line_2("{<[[]]>}<{[{[{[]{()[[[]"), 995444);
    }
    #[test]
    fn part2_ex1e() {
        assert_eq!(score_line_2("<{([{{}}[<[[[<>{}]]]>[]]"), 294);
    }
}
