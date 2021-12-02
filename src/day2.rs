use std::str::FromStr;

struct Point {
    x: i32,
    depth: i32,
}

struct Target {
    x: i32,
    depth: i32,
    aim: i32,
}

pub enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

pub type Instruction = (Direction, i32);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut line = l.trim().split(' ');
            (
                Direction::from_str(line.next().unwrap()).unwrap(),
                line.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    let mut loc = Point { x: 0, depth: 0 };

    for ins in input {
        match ins.0 {
            Direction::Up => {
                loc.depth -= ins.1;
            }
            Direction::Down => {
                loc.depth += ins.1;
            }
            Direction::Forward => {
                loc.x += ins.1;
            }
        }
    }

    loc.depth * loc.x
}

#[aoc(day2, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
    let mut tgt = Target {
        x: 0,
        depth: 0,
        aim: 0,
    };

    for ins in input {
        match ins.0 {
            Direction::Up => {
                tgt.aim -= ins.1;
            }
            Direction::Down => {
                tgt.aim += ins.1;
            }
            Direction::Forward => {
                tgt.x += ins.1;
                tgt.depth += tgt.aim * ins.1;
            }
        }
    }

    tgt.depth * tgt.x
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
