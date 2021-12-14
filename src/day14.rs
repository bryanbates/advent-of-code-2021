use std::collections::HashMap;

#[derive(Debug, Default)]
struct Formula {
    template: String,
    rules: HashMap<[char; 2], char>,
}

fn parse_input(input: &str) -> Formula {
    let mut formula: Formula = Default::default();
    let mut lines = input.lines();

    // First line is template
    formula.template = lines.next().unwrap().trim().to_owned();
    // Then a blank
    lines.next();
    // Then the rules
    for line in lines {
        let mut l = line.trim().split_whitespace();
        let mut from = l.next().unwrap().chars();
        // discard the ->
        l.next();
        let to = l.next().unwrap().chars().next().unwrap();
        formula
            .rules
            .insert([from.next().unwrap(), from.next().unwrap()], to);
    }

    formula
}

#[derive(Debug, Default)]
struct FormulaBytes {
    template: Vec<u8>,
    rules: HashMap<[u8; 2], u8>,
}

fn parse_input_bytes(input: &str) -> FormulaBytes {
    let mut formula: FormulaBytes = Default::default();
    let mut lines = input.lines();

    // First line is template
    formula.template = lines.next().unwrap().trim().bytes().collect::<Vec<_>>();
    // Then a blank
    lines.next();
    // Then the rules
    for line in lines {
        let mut l = line.trim().split_whitespace();
        let mut from = l.next().unwrap().bytes();
        // discard the ->
        l.next();
        let to = l.next().unwrap().bytes().next().unwrap();
        formula
            .rules
            .insert([from.next().unwrap(), from.next().unwrap()], to);
    }

    formula
}

fn apply_str(formula: &Formula, polymer: String) -> String {
    let pchars: Vec<char> = polymer.chars().collect();

    let inserts = pchars
        .windows(2)
        .flat_map(|pair| {
            let insert = formula.rules.get(&[pair[0], pair[1]]).unwrap();
            [pair[0], *insert]
        })
        .collect::<String>();

    format!("{}{}", inserts, pchars.last().unwrap())
}

fn frequencies(polymer: &str) -> Vec<(char, u64)> {
    let mut freqs: HashMap<char, u64> = HashMap::new();
    for ch in polymer.chars() {
        *freqs.entry(ch).or_insert(0) += 1;
    }
    let mut freqvec: Vec<(char, u64)> = freqs.iter().map(|(&k, &v)| (k, v)).collect();
    freqvec.sort_by(|a, b| b.1.cmp(&a.1));
    freqvec
}

#[aoc(day14, part1, brute)]
pub fn part1_brute(input: &str) -> u64 {
    let formula = parse_input(input);
    let mut polymer = formula.template.clone();

    for _ in 0..10 {
        polymer = apply_str(&formula, polymer);
    }

    // println!("Polymer: {:?}", polymer);

    let freqs = frequencies(&polymer);

    // println!("Freqs: {:?}", freqs);

    freqs.first().unwrap().1 - freqs.last().unwrap().1
}

fn apply_bytes(formula: &FormulaBytes, polymer: Vec<u8>) -> Vec<u8> {
    let mut inserts = polymer
        .windows(2)
        .flat_map(|pair| {
            let insert = formula.rules.get(&[pair[0], pair[1]]).unwrap();
            [pair[0], *insert]
        })
        .collect::<Vec<_>>();

    inserts.push(*polymer.last().unwrap());

    inserts
}

fn frequencies_bytes(polymer: &[u8]) -> Vec<(u8, u64)> {
    let mut freqs: HashMap<u8, u64> = HashMap::new();
    for ch in polymer {
        *freqs.entry(*ch).or_insert(0) += 1;
    }
    let mut freqvec: Vec<(u8, u64)> = freqs.iter().map(|(&k, &v)| (k, v)).collect();
    freqvec.sort_by(|a, b| b.1.cmp(&a.1));
    freqvec
}

#[aoc(day14, part1, bytes)]
pub fn part1_bytes(input: &str) -> u64 {
    let formula = parse_input_bytes(input);
    let mut polymer = formula.template.clone();

    for _ in 0..10 {
        polymer = apply_bytes(&formula, polymer);
    }

    // println!("Polymer: {:?}", polymer);

    let freqs = frequencies_bytes(&polymer);

    // println!("Freqs: {:?}", freqs);

    freqs.first().unwrap().1 - freqs.last().unwrap().1
}

fn step_maps(formula: &FormulaBytes, current: HashMap<[u8; 2], u64>) -> HashMap<[u8; 2], u64> {
    let mut pair_counts: HashMap<[u8; 2], u64> = HashMap::new();

    for (k, v) in &current {
        let insert = *formula.rules.get(k).unwrap();

        let p1 = [k[0], insert];
        let p2 = [insert, k[1]];

        *pair_counts.entry(p1).or_insert(0) += v;
        *pair_counts.entry(p2).or_insert(0) += v;
    }

    pair_counts
}

#[aoc(day14, part1, pairs)]
pub fn part1_pairs(input: &str) -> u64 {
    let formula = parse_input_bytes(input);

    let mut pair_counts: HashMap<[u8; 2], u64> = HashMap::new();

    // Initialize Step 0 from template
    for ws in formula.template.windows(2) {
        let mut wsi = ws.iter();
        *pair_counts
            .entry([*wsi.next().unwrap(), *wsi.next().unwrap()])
            .or_insert(0) += 1;
    }

    for _ in 0..10 {
        // println!("{:?}", pair_counts);
        pair_counts = step_maps(&formula, pair_counts);
    }

    let mut counters: HashMap<u8, u64> = HashMap::new();

    for (pair, v) in pair_counts {
        *counters.entry(pair[0]).or_insert(0) += v;
    }
    // Also count the last char
    *counters
        .entry(*formula.template.last().unwrap())
        .or_insert(0) += 1;

    // println!("Counters: {:?}", counters);

    let mut freqs: Vec<(u8, u64)> = counters.iter().map(|(&k, &v)| (k, v)).collect();
    freqs.sort_by(|a, b| b.1.cmp(&a.1));

    freqs.first().unwrap().1 - freqs.last().unwrap().1
}

#[aoc(day14, part2, pairs)]
pub fn part2_pairs(input: &str) -> u64 {
    let formula = parse_input_bytes(input);

    let mut pair_counts: HashMap<[u8; 2], u64> = HashMap::new();

    // Initialize Step 0 from template
    for ws in formula.template.windows(2) {
        let mut wsi = ws.iter();
        *pair_counts
            .entry([*wsi.next().unwrap(), *wsi.next().unwrap()])
            .or_insert(0) += 1;
    }

    for _ in 0..40 {
        // println!("{:?}", pair_counts);
        pair_counts = step_maps(&formula, pair_counts);
    }

    let mut counters: HashMap<u8, u64> = HashMap::new();

    for (pair, v) in pair_counts {
        *counters.entry(pair[0]).or_insert(0) += v;
    }
    // Also count the last char
    *counters
        .entry(*formula.template.last().unwrap())
        .or_insert(0) += 1;

    // println!("Counters: {:?}", counters);

    let mut freqs: Vec<(u8, u64)> = counters.iter().map(|(&k, &v)| (k, v)).collect();
    freqs.sort_by(|a, b| b.1.cmp(&a.1));

    freqs.first().unwrap().1 - freqs.last().unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1_brute(EXAMPLE_INPUT), 1588);
        assert_eq!(part1_bytes(EXAMPLE_INPUT), 1588);
        assert_eq!(part1_pairs(EXAMPLE_INPUT), 1588);
    }

    #[test]
    fn part1_ex1_step_1() {
        let formula = parse_input(EXAMPLE_INPUT);
        let mut polymer: String = formula.template.clone();

        assert_eq!(polymer, "NNCB");

        polymer = apply_str(&formula, polymer);

        assert_eq!(polymer, "NCNBCHB");
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2_pairs(EXAMPLE_INPUT), 2188189693529)
    }
}
