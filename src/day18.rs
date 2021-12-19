use itertools::Itertools;
use std::collections::VecDeque;

type SnailFish = Vec<u8>;

const LBRAK: u8 = 252;
const COMMA: u8 = 253;
const RBRAK: u8 = 254;

fn parse(s: &str) -> SnailFish {
    let mut result: SnailFish = SnailFish::with_capacity(s.len());

    for ch in s.trim().bytes() {
        match ch {
            b'[' => result.push(LBRAK),
            b']' => result.push(RBRAK),
            b',' => result.push(COMMA),
            b'0'..=b'9' => result.push(ch - b'0'),
            _ => unreachable!(),
        }
    }

    result
}

#[allow(dead_code)]
fn render(x: &[u8]) {
    for &ch in x {
        match ch {
            LBRAK => print!("["),
            COMMA => print!(","),
            RBRAK => print!("]"),
            _ => print!("{}", ch),
        }
    }
    println!();
}

fn add(x: SnailFish, y: SnailFish) -> SnailFish {
    let mut result: SnailFish = SnailFish::with_capacity(x.len() + y.len() + 3);

    result.push(LBRAK);
    result.extend_from_slice(&x);
    result.push(COMMA);
    result.extend_from_slice(&y);
    result.push(RBRAK);

    result
}

fn reduce(x: SnailFish) -> SnailFish {
    let mut val: SnailFish = x;

    loop {
        // println!("REDUCING...");
        // render(&val);
        if let Some(y) = explode(val.clone()) {
            // println!("EXPLODED");
            val = y;
            continue;
        } else if let Some(y) = split(val.clone()) {
            // println!("SPLIT");
            val = y;
            continue;
        } else {
            // println!("DONE");
            break;
        }
    }

    val
}

fn explode(x: SnailFish) -> Option<SnailFish> {
    let mut before: VecDeque<u8> = VecDeque::new();
    let mut after: VecDeque<u8> = x.iter().copied().collect::<VecDeque<_>>();

    let mut depth = 0;

    let mut exploded = false;

    while !after.is_empty() {
        // println!("BEFORE: {:?}", before);
        // println!("AFTER: {:?}", after);

        let cur = after.pop_front().unwrap();

        before.push_back(cur);

        match cur {
            LBRAK => {
                depth += 1;
                if depth > 4 {
                    // println!("EXPLOSION");
                    exploded = true;
                    // Explode!
                    let lbrak = before.pop_back().unwrap();
                    // sanity check
                    assert_eq!(lbrak, LBRAK);
                    // First, read in the rest of this pair.
                    let left = after.pop_front().unwrap();
                    let comma = after.pop_front().unwrap();
                    // sanity check
                    assert_eq!(comma, COMMA);
                    let right = after.pop_front().unwrap();
                    let rbrak = after.pop_front().unwrap();
                    // sanity check
                    assert_eq!(rbrak, RBRAK);

                    // Now rewind to the previous regular number
                    let mut no_left = true;
                    let mut stk: Vec<u8> = Vec::new();
                    while let Some(prev) = before.pop_back() {
                        match prev {
                            LBRAK | COMMA | RBRAK => {
                                stk.push(prev);
                            }
                            _ => {
                                no_left = false;
                                stk.push(prev);
                                break;
                            }
                        }
                    }
                    // prev reg num is now on stk.
                    // Add left, and pop remaining stack
                    if !no_left {
                        before.push_back(stk.pop().unwrap() + left);
                    }
                    while !stk.is_empty() {
                        before.push_back(stk.pop().unwrap());
                    }

                    // The pair becomes 0
                    before.push_back(0);

                    // Now, the right side. Read forward to the next regular number.
                    let mut no_right = true;
                    let mut q: VecDeque<u8> = VecDeque::new();

                    while let Some(next) = after.pop_front() {
                        match next {
                            LBRAK | COMMA | RBRAK => {
                                q.push_back(next);
                            }
                            _ => {
                                no_right = false;
                                q.push_back(next);
                                break;
                            }
                        }
                    }

                    // Next reg num is now at end of queue.
                    while !q.is_empty() {
                        before.push_back(q.pop_front().unwrap());
                    }

                    if !no_right {
                        // Add.
                        let next_right = before.pop_back().unwrap();
                        before.push_back(next_right + right);
                    }

                    break;
                }
            }
            RBRAK => {
                depth -= 1;
            }
            _ => {}
        }
    }

    while !after.is_empty() {
        before.push_back(after.pop_front().unwrap());
    }

    if exploded {
        Some(before.iter().copied().collect::<Vec<_>>())
    } else {
        None
    }
}

fn split(x: SnailFish) -> Option<SnailFish> {
    let mut before: VecDeque<u8> = VecDeque::new();
    let mut after: VecDeque<u8> = x.iter().copied().collect::<VecDeque<_>>();

    let mut did_split = false;

    while !after.is_empty() {
        // println!("BEFORE: {:?}", before);
        // println!("AFTER: {:?}", after);

        let cur = after.pop_front().unwrap();

        before.push_back(cur);

        if let 10..=251 = cur {
            // Too big, need to split into a pair.
            did_split = true;
            // Take the value off the queue
            before.pop_back();
            // Split it up
            let left = cur / 2;
            let right = left + (cur % 2);

            // println!("SPLIT {} -> {}, {}", cur, left, right);
            // and push the pair.
            before.push_back(LBRAK);
            before.push_back(left);
            before.push_back(COMMA);
            before.push_back(right);
            before.push_back(RBRAK);

            break;
        }
    }

    while !after.is_empty() {
        before.push_back(after.pop_front().unwrap());
    }

    if did_split {
        Some(before.iter().copied().collect::<Vec<_>>())
    } else {
        None
    }
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Value(u8),
    Pair(Box<(Node, Node)>),
}

fn tree_parse(s: &[u8]) -> Node {
    let mut stack: Vec<Node> = Vec::new();

    for &ch in s {
        match ch {
            LBRAK | COMMA => {
                // Do nothing, yet?
            }
            RBRAK => {
                // POP 2, push pair
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let this = Node::Pair(Box::new((left, right)));
                stack.push(this);
            }
            _ => stack.push(Node::Value(ch)),
        }
    }

    stack.pop().unwrap()
}

fn value(n: Node) -> u64 {
    match n {
        Node::Value(x) => x as u64,
        Node::Pair(b) => {
            let (left, right) = *b;
            3 * value(left) + 2 * value(right)
        }
    }
}

fn magnitude(x: SnailFish) -> u64 {
    let root = tree_parse(&x);
    value(root)
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> u64 {
    let mut fishes = input
        .trim()
        .split_whitespace()
        .map(parse)
        .collect::<VecDeque<_>>();

    let mut val = fishes.pop_front().unwrap();

    while !fishes.is_empty() {
        val = add(val, fishes.pop_front().unwrap());
        val = reduce(val);
    }

    magnitude(val)
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> u64 {
    let fishes = input
        .trim()
        .split_whitespace()
        .map(parse)
        .collect::<Vec<_>>();

    let nfish = fishes.len();

    (0..nfish)
        .cartesian_product(0..nfish)
        .filter(|(a, b)| a != b)
        .map(|(a, b)| {
            let fish_a = fishes[a].clone();
            let fish_b = fishes[b].clone();
            let mut x = add(fish_a, fish_b);
            x = reduce(x);
            magnitude(x)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"#;

    #[test]
    fn snail_explode() {
        assert_eq!(
            explode(parse("[[[[[9,8],1],2],3],4]")),
            Some(parse("[[[[0,9],2],3],4]"))
        );
        assert_eq!(
            explode(parse("[7,[6,[5,[4,[3,2]]]]]")),
            Some(parse("[7,[6,[5,[7,0]]]]"))
        );
        assert_eq!(
            explode(parse("[[6,[5,[4,[3,2]]]],1]")),
            Some(parse("[[6,[5,[7,0]]],3]"))
        );
        assert_eq!(
            explode(parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")),
            Some(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
        );
        assert_eq!(
            explode(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")),
            Some(parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"))
        );
    }

    #[test]
    fn snail_reduce() {
        assert_eq!(
            reduce(add(parse("[[[[4,3],4],4],[7,[[8,4],9]]]"), parse("[1,1]"))),
            parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn snail_magnitude() {
        assert_eq!(magnitude(parse("[9,1]")), 29);
        assert_eq!(magnitude(parse("[[1,2],[[3,4],5]]")), 143);
        assert_eq!(magnitude(parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")), 1384);
        assert_eq!(magnitude(parse("[[[[1,1],[2,2]],[3,3]],[4,4]]")), 445);
        assert_eq!(magnitude(parse("[[[[3,0],[5,3]],[4,4]],[5,5]]")), 791);
        assert_eq!(magnitude(parse("[[[[5,0],[7,4]],[5,5]],[6,6]]")), 1137);
        assert_eq!(
            magnitude(parse(
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            )),
            3488
        );
        assert_eq!(
            magnitude(parse(
                "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
            )),
            4140
        );
    }

    #[test]
    fn part1_ex1() {
        // [[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]
        assert_eq!(part1(EXAMPLE_INPUT), 4140);
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(EXAMPLE_INPUT), 3993)
    }
}
