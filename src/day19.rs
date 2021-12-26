use std::collections::{HashSet, VecDeque};

type Position = (i32, i32, i32);

type Scanner = HashSet<Position>;

fn parse_input(input: &str) -> VecDeque<Scanner> {
    let mut puzzle: VecDeque<Scanner> = VecDeque::new();

    let mut scanner: Scanner = Scanner::new();
    for line in input.trim().lines() {
        if line.is_empty() {
            puzzle.push_back(scanner);
            scanner = Scanner::new();
        } else {
            if line.starts_with("---") {
                // Just the name
            } else {
                let mut vals = line.split(',').map(|v| v.parse::<i32>().unwrap());
                scanner.insert((
                    vals.next().unwrap(),
                    vals.next().unwrap(),
                    vals.next().unwrap(),
                ));
            }
        }
    }
    // last scanner
    puzzle.push_back(scanner);

    puzzle
}

fn permute(s: &Scanner) -> Vec<Scanner> {
    let mut res: Vec<Scanner> = vec![s.clone()];

    // FORWARDS
    // XYZ --blue, green, purple
    // x,-z,y
    let mut x_nz_y: Scanner = Scanner::new();
    for p in s.clone() {
        x_nz_y.insert((p.0, -p.2, p.1));
    }
    res.push(x_nz_y);

    // x,-y,-z
    let mut x_ny_nz: Scanner = Scanner::new();
    for p in s.clone() {
        x_ny_nz.insert((p.0, -p.1, -p.2));
    }
    res.push(x_ny_nz);

    // x,z,-y
    let mut x_z_ny: Scanner = Scanner::new();
    for p in s.clone() {
        x_z_ny.insert((p.0, p.2, -p.1));
    }
    res.push(x_z_ny);

    // BACKWARDS
    // -x,y,-z
    let mut nx_y_nz: Scanner = Scanner::new();
    for p in s.clone() {
        nx_y_nz.insert((-p.0, p.1, -p.2));
    }
    res.push(nx_y_nz);

    // -x,-z,-y
    let mut nx_nz_ny: Scanner = Scanner::new();
    for p in s.clone() {
        nx_nz_ny.insert((-p.0, -p.2, -p.1));
    }
    res.push(nx_nz_ny);

    // -x,-y,z
    let mut nx_ny_z: Scanner = Scanner::new();
    for p in s.clone() {
        nx_ny_z.insert((-p.0, -p.1, p.2));
    }
    res.push(nx_ny_z);

    // -x,z,y
    let mut nx_z_y: Scanner = Scanner::new();
    for p in s.clone() {
        nx_z_y.insert((-p.0, p.2, p.1));
    }
    res.push(nx_z_y);

    // LEFT
    // y, -x, z
    let mut y_nx_z: Scanner = Scanner::new();
    for p in s.clone() {
        y_nx_z.insert((p.1, -p.0, p.2));
    }
    res.push(y_nx_z);

    // -z, -x, y
    let mut nz_nx_y: Scanner = Scanner::new();
    for p in s.clone() {
        nz_nx_y.insert((-p.2, -p.0, p.1));
    }
    res.push(nz_nx_y);

    // -y. -x, -z
    let mut ny_nx_nz: Scanner = Scanner::new();
    for p in s.clone() {
        ny_nx_nz.insert((-p.1, -p.0, -p.2));
    }
    res.push(ny_nx_nz);

    // z, -x, -y
    let mut z_nx_ny: Scanner = Scanner::new();
    for p in s.clone() {
        z_nx_ny.insert((p.2, -p.0, -p.1));
    }
    res.push(z_nx_ny);

    // RIGHT
    // -y, x, z
    let mut ny_x_z: Scanner = Scanner::new();
    for p in s.clone() {
        ny_x_z.insert((-p.1, p.0, p.2));
    }
    res.push(ny_x_z);

    // z, x, y
    let mut z_x_y: Scanner = Scanner::new();
    for p in s.clone() {
        z_x_y.insert((p.2, p.0, p.1));
    }
    res.push(z_x_y);

    // y, x, -z
    let mut y_x_nz: Scanner = Scanner::new();
    for p in s.clone() {
        y_x_nz.insert((p.1, p.0, -p.2));
    }
    res.push(y_x_nz);

    // -z, x, -y
    let mut nz_x_ny: Scanner = Scanner::new();
    for p in s.clone() {
        nz_x_ny.insert((-p.2, p.0, -p.1));
    }
    res.push(nz_x_ny);

    // UP
    // -z, y, x
    let mut nz_y_x: Scanner = Scanner::new();
    for p in s.clone() {
        nz_y_x.insert((-p.2, p.1, p.0));
    }
    res.push(nz_y_x);

    // -y, -z, x
    let mut ny_nz_x: Scanner = Scanner::new();
    for p in s.clone() {
        ny_nz_x.insert((-p.1, -p.2, p.0));
    }
    res.push(ny_nz_x);

    // z, -y, x
    let mut z_ny_x: Scanner = Scanner::new();
    for p in s.clone() {
        z_ny_x.insert((p.2, -p.1, p.0));
    }
    res.push(z_ny_x);

    // y, z, x
    let mut y_z_x: Scanner = Scanner::new();
    for p in s.clone() {
        y_z_x.insert((p.1, p.2, p.0));
    }
    res.push(y_z_x);

    // DOWN
    // z, y, -x
    let mut z_y_nx: Scanner = Scanner::new();
    for p in s.clone() {
        z_y_nx.insert((p.2, p.1, -p.0));
    }
    res.push(z_y_nx);

    // -y, z, -x
    let mut ny_z_nx: Scanner = Scanner::new();
    for p in s.clone() {
        ny_z_nx.insert((-p.1, p.2, -p.0));
    }
    res.push(ny_z_nx);

    // -z, -y, -x
    let mut nz_ny_nx: Scanner = Scanner::new();
    for p in s.clone() {
        nz_ny_nx.insert((-p.2, -p.1, -p.0));
    }
    res.push(nz_ny_nx);

    // y, -z, -x
    let mut y_nz_nx: Scanner = Scanner::new();
    for p in s.clone() {
        y_nz_nx.insert((p.1, -p.2, -p.0));
    }
    res.push(y_nz_nx);

    res
}

fn translate(scanner: &Scanner, zero: Position) -> Scanner {
    // Offset everything in scanner by delta
    let least = scanner.iter().min().unwrap();
    let delta = (least.0 - zero.0, least.1 - zero.1, least.2 - zero.2);
    scanner
        .iter()
        .copied()
        .map(|(x, y, z)| (x - delta.0, y - delta.1, z - delta.2))
        .collect::<Scanner>()
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let mut puzzle = parse_input(input);

    // Start w/ the first scanner's beacons as "known"
    let mut known_beacons: HashSet<Position> = puzzle
        .pop_front()
        .unwrap()
        .iter()
        .copied()
        .collect::<HashSet<_>>();
    let mut known_scanners: HashSet<Position> = HashSet::new();
    known_scanners.insert((0, 0, 0));

    let mut iters = 2;

    while !puzzle.is_empty() && iters > 0 {
        iters -= 1;
        // Is there a permutation / translation that gives us at least 12 points in the known set?
        let scanner = puzzle.pop_front().unwrap();
        let mut to_add: Option<Scanner> = None;
        let mut retry = true;
        for perm in permute(&scanner) {
            // Try translating relative to every known beacon
            println!("Known: {:?}", known_beacons);
            for beacon in &known_beacons {
                let xlat = translate(&perm, *beacon);
                println!("Translated: {:?}", xlat);
                let intersect = known_beacons.intersection(&xlat).count();
                println!("INTERSECTION w/ KNOWN: {}", intersect);
                if intersect >= 12 {
                    // Found a match!
                    // Add these (translated) points to the known beacon set

                    to_add = Some(xlat);
                    retry = false;
                    break;
                }
            }
            if let Some(beacons) = to_add {
                for b in beacons {
                    known_beacons.insert(b);
                }
                break;
            }
        }
        if retry {
            // Try this scanner again later, when we know more.
            puzzle.push_back(scanner);
        }
    }

    known_beacons.len()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> u32 {
    let puzzle = parse_input(input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(EXAMPLE_INPUT), 79)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(EXAMPLE_INPUT), 6)
    }
}
