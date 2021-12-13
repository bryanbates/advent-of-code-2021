use std::collections::HashMap;

type Edge = (String, String);

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Edge> {
    input
        .lines()
        .map(|l| {
            let mut line = l.trim().split('-').map(|s| s.to_owned());
            (line.next().unwrap(), line.next().unwrap())
        })
        .collect()
}

type Graph = HashMap<String, Vec<String>>;

fn to_map(edges: &[Edge]) -> Graph {
    let mut graph: Graph = HashMap::new();
    for (from, to) in edges {
        graph
            .entry(from.clone())
            .or_insert_with(Vec::new)
            .push(to.clone());
        graph
            .entry(to.clone())
            .or_insert_with(Vec::new)
            .push(from.clone());
    }
    graph
}

fn is_small(room: &str) -> bool {
    room == room.to_lowercase()
}

fn explore(graph: Graph, node: String, cpath: Vec<String>) -> u32 {
    if node == "end" {
        // println!("PATH: {:?}, {}", cpath, node);
        return 1;
    }

    let mut paths_to_end: u32 = 0;

    if let Some(children) = graph.get(&node) {
        // println!("NEXT: {:?}", children);

        let mut next_path = cpath.clone();
        next_path.push(node);

        for child in children {
            if is_small(child) && cpath.contains(child) {
                // small room already visited on this path, skip
                // println!("Path already contains small room {}", child);
            } else {
                paths_to_end += explore(graph.clone(), child.clone(), next_path.clone());
            }
        }
    }

    paths_to_end
}

fn explore_bonus(graph: Graph, node: String, cpath: Vec<String>, bonus: Option<String>) -> u32 {
    if node == "end" {
        // println!("PATH: {:?}, {}, BONUS: {:?}", cpath, node, bonus);
        return 1;
    }

    let mut paths_to_end: u32 = 0;

    if let Some(children) = graph.get(&node) {
        // println!("NEXT: {:?}", children);

        let mut next_path = cpath.clone();
        next_path.push(node);

        for child in children {
            if is_small(child) {
                match cpath.iter().filter(|&x| x == child).count() {
                    0 => {
                        // Continue, we haven't visited this child yet
                        paths_to_end += explore_bonus(
                            graph.clone(),
                            child.clone(),
                            next_path.clone(),
                            bonus.clone(),
                        );
                    }
                    1 => {
                        if let Some(ref s) = bonus {
                            if s == child {
                                // We're the bonus, keep going
                                paths_to_end += explore_bonus(
                                    graph.clone(),
                                    child.clone(),
                                    next_path.clone(),
                                    bonus.clone(),
                                );
                            } else {
                                // Another node is the bonus, we're done.
                            }
                        } else {
                            if child == "start" || child == "end" {
                                // Can't be bonus, done
                            } else {
                                // No bonus picked yet, try this one
                                paths_to_end += explore_bonus(
                                    graph.clone(),
                                    child.clone(),
                                    next_path.clone(),
                                    Some(child.clone()),
                                );
                            }
                        }
                    }
                    _ => {
                        // Already visited this child the max # of times
                    }
                }
            } else {
                paths_to_end += explore_bonus(
                    graph.clone(),
                    child.clone(),
                    next_path.clone(),
                    bonus.clone(),
                );
            }
        }
    }

    paths_to_end
}

#[aoc(day12, part1)]
pub fn part1(input: &[Edge]) -> u32 {
    let graph = to_map(input);
    // println!("Graph: {:?}", graph);
    explore(graph, "start".to_string(), vec![])
}

#[aoc(day12, part2)]
pub fn part2(input: &[Edge]) -> u32 {
    let graph = to_map(input);
    // println!("Graph: {:?}", graph);
    explore_bonus(graph, "start".to_string(), vec![], None)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;

    const EXAMPLE_INPUT_2: &str = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"#;

    const EXAMPLE_INPUT_3: &str = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT_1)), 10);
    }

    #[test]
    fn part1_ex2() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT_2)), 19);
    }

    #[test]
    fn part1_ex3() {
        assert_eq!(part1(&input_generator(EXAMPLE_INPUT_3)), 226);
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT_2)), 103);
    }

    #[test]
    fn part2_ex2() {
        assert_eq!(part2(&input_generator(EXAMPLE_INPUT_3)), 3509);
    }
}
