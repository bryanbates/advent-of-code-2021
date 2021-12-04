const BITS: usize = 5;
//const BITS: usize = 12;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| u32::from_str_radix(l.trim(), 2).unwrap())
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[u32]) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..BITS {
        let mcb = most_common_bit_at_index(input, i);
        println!("{} {}", i, mcb);
        gamma += mcb << i;
        epsilon += (1 - mcb) << i;
    }

    println!("ga: {}, ep: {}", gamma, epsilon);

    epsilon * gamma
}

fn most_common_bit_at_index(input: &[u32], index: usize) -> u32 {
    let ones: usize = input
        .iter()
        .map(|n| -> u32 { (n & (1 << index) > 0).into() })
        .sum::<u32>() as usize;
    // println!("MCB @ {}: {}, {}", index, input.len(), ones);
    let threshold = (1 + input.len()) / 2;
    (ones >= threshold).into()
}

fn filter_most_common_at_index(input: &[u32], index: usize) -> Vec<u32> {
    let mcb = most_common_bit_at_index(input, index);
    // println!("MCB @ {} = {}", index, mcb);
    input
        .iter()
        .filter(|&v| ((v & (1 << index)) ^ (mcb << index)) == 0)
        .copied()
        .collect::<Vec<_>>()
}

fn filter_least_common_at_index(input: &[u32], index: usize) -> Vec<u32> {
    let lcb = 1 - most_common_bit_at_index(input, index);
    // println!("LCB @ {} = {}", index, lcb);
    input
        .iter()
        .filter(|&v| ((v & (1 << index)) ^ (lcb << index)) == 0)
        .copied()
        .collect::<Vec<_>>()
}

#[aoc(day3, part2)]
pub fn part2(input: &[u32]) -> u32 {
    let mut remaining = input.iter().copied().collect::<Vec<u32>>();
    let mut oxygen: u32 = 0;
    let mut co2: u32 = 0;
    for i in 1..=BITS {
        remaining = filter_most_common_at_index(&remaining, BITS - i);
        // println!("Remaining: {:?}", remaining);
        if remaining.len() == 1 {
            oxygen = remaining[0];
            break;
        }
    }

    remaining = input.iter().copied().collect::<Vec<u32>>();
    for i in 1..=BITS {
        remaining = filter_least_common_at_index(&remaining, BITS - i);
        // println!("Remaining: {:?}", remaining);
        if remaining.len() == 1 {
            co2 = remaining[0];
            break;
        }
    }

    println!("O2 {} CO2 {}", oxygen, co2);
    oxygen * co2
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;
    #[test]
    fn part1_ex1() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 198)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 230)
    }
}
