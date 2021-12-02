pub type Instruction = (char, i32);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut line = l.trim().split(' ');
            (
                line.next().unwrap().chars().next().unwrap(),
                line.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    let mut x: i32 = 0;
    let mut depth: i32 = 0;

    for ins in input {
        match ins.0 {
            'u' => {
                depth -= ins.1;
            }
            'd' => {
                depth += ins.1;
            }
            'f' => {
                x += ins.1;
            }
            _ => {}
        }
    }

    depth * x
}

#[aoc(day2, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
    let mut x: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for ins in input {
        match ins.0 {
            'u' => {
                aim -= ins.1;
            }
            'd' => {
                aim += ins.1;
            }
            'f' => {
                x += ins.1;
                depth += aim * ins.1;
            }
            _ => {}
        }
    }

    depth * x
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;
    #[test]
    fn part1_ex1() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 150)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 900)
    }
}
