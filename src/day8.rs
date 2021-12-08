use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Chunk = (Vec<String>, Vec<String>);

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Chunk> {
    input
        .lines()
        .map(|l| {
            let mut digits = l.trim().split('|').map(|s| {
                // Sort the patterns for ease of lookup later
                s.trim()
                    .split(' ')
                    .map(|d| d.chars().sorted().collect::<String>())
                    .collect::<Vec<_>>()
            });
            (digits.next().unwrap(), digits.next().unwrap())
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &[Chunk]) -> u32 {
    input
        .iter()
        .map(|(_, after)| {
            after
                .iter()
                .map(|digit| match digit.len() {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn decode(input: &[String]) -> HashMap<String, u32> {
    // Final Wiring
    let mut wiring: HashMap<String, u32> = HashMap::new();

    assert_eq!(input.len(), 10);

    // What we want to know is what letters correspond to ABCDEFG.
    // We know:
    // 0: ABCEFG (D)
    // 1: CF
    // 2: ACDEG (BF)
    // 3: ACDFG (BE)
    // 4: BCDF
    // 5: ABDFG (CE)
    // 6: ABDEFG (C)
    // 7: ACF
    // 8: ABCDEFG
    // 9: ABCDFG (E)

    // Use the unique length patterns to find:
    // CF (1)
    // ACF (7)
    // BCDF (4)
    // BCEF (chars not used by the 3 len=5 patterns: 2, 3, 5)
    // CDE (chars not used by the 3 len=6 patterns: 0, 6, 9)
    let mut cf: HashSet<char> = HashSet::new();
    let mut acf: HashSet<char> = HashSet::new();
    let mut bcdf: HashSet<char> = HashSet::new();
    let mut bcef: HashSet<char> = HashSet::new();
    let mut cde: HashSet<char> = HashSet::new();

    let abcdefg: HashSet<char> = "abcdefg".chars().collect();

    for digit in input {
        let digit_set: HashSet<char> = digit.chars().collect();
        match digit.len() {
            2 => {
                // 1: "CF"
                for ch in digit.chars() {
                    cf.insert(ch);
                }
            }
            3 => {
                // 7: "ACF"
                for ch in digit.chars() {
                    acf.insert(ch);
                }
            }
            4 => {
                // 4: "BCDF"
                for ch in digit.chars() {
                    bcdf.insert(ch);
                }
            }
            5 => {
                // BCEF: elements missing from the 3 len5 patterns (2, 3, 5)
                for &ch in abcdefg.difference(&digit_set) {
                    bcef.insert(ch);
                }
            }
            6 => {
                // CDE: element missing from the 3 len6 patterns (0, 6, 9)
                for &ch in abcdefg.difference(&digit_set) {
                    cde.insert(ch);
                }
            }
            _ => {}
        }
    }

    // Then use set ops to find...
    // A: ACF - CF
    // BD: BCDF - CF
    // BE: BCEF - CF
    // B: intersection of BD, BE
    // C: intersection of CDE,CF
    // D: intersection of BD, CDE
    // E: CDE - C,D
    // F: CF - C
    // G: ABCDEFG - A,B,C,D,E,F

    let a = *acf.difference(&cf).next().unwrap();
    let bd: HashSet<char> = bcdf.difference(&cf).cloned().collect();
    let be: HashSet<char> = bcef.difference(&cf).cloned().collect();
    let b = *bd.intersection(&be).next().unwrap();
    let c = *cde.intersection(&cf).next().unwrap();
    let d = *cde.intersection(&bd).next().unwrap();
    let e = *cde
        .difference(&[c, d].iter().cloned().collect())
        .next()
        .unwrap();
    let f = *cf
        .difference(&[c].iter().cloned().collect())
        .next()
        .unwrap();
    let g = *abcdefg
        .difference(&[a, b, c, d, e, f].iter().cloned().collect())
        .next()
        .unwrap();

    // Now we need to normalize back to "sorted" patterns.
    // 0: ABCEFG
    wiring.insert([a, b, c, e, f, g].iter().sorted().collect::<String>(), 0);
    // 1: CF
    wiring.insert([c, f].iter().sorted().collect::<String>(), 1);
    // 2: ACDEG
    wiring.insert([a, c, d, e, g].iter().sorted().collect::<String>(), 2);
    // 3: ACDFG
    wiring.insert([a, c, d, f, g].iter().sorted().collect::<String>(), 3);
    // 4: BCDF
    wiring.insert([b, c, d, f].iter().sorted().collect::<String>(), 4);
    // 5: ABDFG
    wiring.insert([a, b, d, f, g].iter().sorted().collect::<String>(), 5);
    // 6: ABDEFG
    wiring.insert([a, b, d, e, f, g].iter().sorted().collect::<String>(), 6);
    // 7: ACF
    wiring.insert([a, c, f].iter().sorted().collect::<String>(), 7);
    // 8: ABCDEFG
    wiring.insert([a, b, c, d, e, f, g].iter().sorted().collect::<String>(), 8);
    // 9: ABCDFG
    wiring.insert([a, b, c, d, f, g].iter().sorted().collect::<String>(), 9);

    // println!("{:?}", wiring);

    wiring
}

#[aoc(day8, part2)]
pub fn part2(input: &[Chunk]) -> u32 {
    input
        .iter()
        .map(|(before, after)| {
            // Figure out pattern mappings
            let wiring: HashMap<String, u32> = decode(before);

            // Now convert from patterns to a number
            after
                .iter()
                .map(|digit| wiring.get(digit).unwrap())
                .zip([1000, 100, 10, 1])
                .map(|(&digit, place)| digit * place)
                .sum::<u32>()
        })
        .sum()
}

////////////////////////////////////////////////////////////////////////
// Much more compact solution adapted from the reddit megathread...
//
fn to_hashset(chars: &str) -> HashSet<u8> {
    HashSet::from_iter(chars.bytes())
}

#[aoc(day8, part2, intersect)]
pub fn part2_intersect(input: &[Chunk]) -> u32 {
    input.iter().map(|(signal, output)| {
        let splits: Vec<HashSet<u8>> = signal.iter().map(|s| to_hashset(s)).collect();
        let one = splits.iter().find(|&x| x.len() == 2).unwrap().to_owned();
        let four = splits.iter().find(|&x| x.len() == 4).unwrap().to_owned();

        let res =
            output
                .iter()
            .map(|o| to_hashset(o))
            .fold(Vec::with_capacity(4), |mut res, digit| {
                match (
                    digit.len(),
                    one.intersection(&digit).count(),
                    four.intersection(&digit).count(),
                ) {
                    (2, _, _) => res.push(1),
                    (3, _, _) => res.push(7),
                    (4, _, _) => res.push(4),
                    (5, 2, _) => res.push(3),
                    (5, _, 2) => res.push(2),
                    (5, _, _) => res.push(5),
                    (6, 1, _) => res.push(6),
                    (6, _, 4) => res.push(9),
                    (6, _, _) => res.push(0),
                    (7, _, _) => res.push(8),
                    _ => unreachable!(),
                }
                res
            });
        res[0] * 1000 + res[1] * 100 + res[2] * 10 + res[3]
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT)), 26)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 61229);
        assert_eq!(part2_intersect(&input_generator(EXAMPLE_INPUT)), 61229)
    }
}
