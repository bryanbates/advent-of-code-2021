use std::collections::VecDeque;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

pub fn input_helper(input: &[u64]) -> [u64; 9] {
    // INITIALIZE
    let mut fish: [u64; 9] = [0; 9];

    for &i in input {
        fish[i as usize] += 1;
    }
    fish
}

fn sim_3(init: [u64; 9], days: usize) -> u64 {
    let mut fish = init;

    for _ in 0..days {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }

    fish.iter().sum()
}

fn sim_2(init: [u64; 9], days: usize) -> u64 {
    let mut fish = init;

    for _ in 0..days {
        let v = fish[0];
        for i in 1..=8 {
            fish[i - 1] = fish[i];
        }
        fish[6] += v;
        fish[8] = v;
    }

    fish.iter().sum()
}

fn sim_4(init: [u64; 9], days: usize) -> u64 {
    let mut fish = VecDeque::from_iter(init);

    for _ in 0..days {
        let spawning = fish.pop_front().unwrap();
        fish[6] += spawning;
        fish.push_back(spawning);
    }

    fish.iter().sum()
}

fn sim(init: [u64; 9], days: usize) -> u64 {
    let mut fish = init;

    for _ in 0..days {
        let mut next_fish: [u64; 9] = [0; 9];

        next_fish[8] = fish[0];
        next_fish[7] = fish[8];
        next_fish[6] = fish[7] + fish[0];
        next_fish[5] = fish[6];
        next_fish[4] = fish[5];
        next_fish[3] = fish[4];
        next_fish[2] = fish[3];
        next_fish[1] = fish[2];
        next_fish[0] = fish[1];

        fish = next_fish;
    }

    fish.iter().sum()
}

#[aoc(day6, part1, unroll)]
pub fn part1(input: &[u64]) -> u64 {
    let fish = input_helper(input);
    sim(fish, 80)
}

#[aoc(day6, part1, looper)]
pub fn part1_loop(input: &[u64]) -> u64 {
    let fish = input_helper(input);
    sim_2(fish, 80)
}

#[aoc(day6, part1, rotate)]
pub fn part1_rotate(input: &[u64]) -> u64 {
    let fish = input_helper(input);
    sim_3(fish, 80)
}

#[aoc(day6, part1, vecdeque)]
pub fn part1_vecdeque(input: &[u64]) -> u64 {
    let fish = input_helper(input);
    sim_4(fish, 80)
}

#[aoc(day6, part2, unroll)]
pub fn part2(input: &[u64]) -> u64 {
    let fish = input_helper(input);
    sim(fish, 256)
}

#[aoc(day6, part2, looper)]
pub fn part2_loop(input: &[u64]) -> u64 {
    let fish = input_helper(input);
    sim_2(fish, 256)
}

#[aoc(day6, part2, rotate)]
pub fn part2_rotate(input: &[u64]) -> u64 {
    let fish = input_helper(input);
    sim_3(fish, 256)
}

#[aoc(day6, part2, vecdeque)]
pub fn part2_vecdeque(input: &[u64]) -> u64 {
    let fish = input_helper(input);
    sim_4(fish, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"3,4,3,1,2
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 5934)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 26984457539)
    }
}
