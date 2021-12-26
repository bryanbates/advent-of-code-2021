use std::collections::VecDeque;

use rayon::prelude::*;

#[derive(Copy, Clone, Debug)]
enum Reg {
    W,
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, Debug)]
enum IS {
    Inp(Reg),
    AddC(Reg, i16),
    AddR(Reg, Reg),
    MulC(Reg, i16),
    MulR(Reg, Reg),
    DivC(Reg, i16),
    DivR(Reg, Reg),
    ModC(Reg, i16),
    ModR(Reg, Reg),
    EqlC(Reg, i16),
    EqlR(Reg, Reg),
}

#[derive(Clone, Debug, Default)]
struct Machine {
    w: i16,
    x: i16,
    y: i16,
    z: i16,
    inp: VecDeque<i16>,
    prog: VecDeque<IS>,
}

impl Machine {

    fn run(&mut self) {
        while !self.prog.is_empty() {
            let ins = self.prog.pop_front().unwrap();
            self.execute(ins);
        }
    }

    fn execute(&mut self, pc: IS) {
        match pc {
            IS::Inp(a) => {
                match a {
                    Reg::W => self.w = self.inp.pop_front().unwrap(),
                    Reg::X => self.x = self.inp.pop_front().unwrap(),
                    Reg::Y => self.y = self.inp.pop_front().unwrap(),
                    Reg::Z => self.z = self.inp.pop_front().unwrap(),
                }
            },
            IS::AddC(a, b) => {
                match a {
                    Reg::W => self.w += b,
                    Reg::X => self.x += b,
                    Reg::Y => self.y += b,
                    Reg::Z => self.z += b,
                }
            },
            IS::AddR(a, b) => {
                let val = match b {
                    Reg::W => self.w,
                    Reg::X => self.x,
                    Reg::Y => self.y,
                    Reg::Z => self.z,
                };
                match a {
                    Reg::W => self.w += val,
                    Reg::X => self.x += val,
                    Reg::Y => self.y += val,
                    Reg::Z => self.z += val,
                }
            },
            IS::MulC(a, b) => {
                match a {
                    Reg::W => self.w *= b,
                    Reg::X => self.x *= b,
                    Reg::Y => self.y *= b,
                    Reg::Z => self.z *= b,
                }
            },
            IS::MulR(a, b) => {
                let val = match b {
                    Reg::W => self.w,
                    Reg::X => self.x,
                    Reg::Y => self.y,
                    Reg::Z => self.z,
                };
                match a {
                    Reg::W => self.w *= val,
                    Reg::X => self.x *= val,
                    Reg::Y => self.y *= val,
                    Reg::Z => self.z *= val,
                }
            },
            IS::DivC(a, b) => {
                match a {
                    Reg::W => self.w /= b,
                    Reg::X => self.x /= b,
                    Reg::Y => self.y /= b,
                    Reg::Z => self.z /= b,
                }
            },
            IS::DivR(a, b) => {
                let val = match b {
                    Reg::W => self.w,
                    Reg::X => self.x,
                    Reg::Y => self.y,
                    Reg::Z => self.z,
                };
                match a {
                    Reg::W => self.w /= val,
                    Reg::X => self.x /= val,
                    Reg::Y => self.y /= val,
                    Reg::Z => self.z /= val,
                }
            },
            IS::ModC(a, b) => {
                match a {
                    Reg::W => self.w %= b,
                    Reg::X => self.x %= b,
                    Reg::Y => self.y %= b,
                    Reg::Z => self.z %= b,
                }
            },
            IS::ModR(a, b) => {
                let val = match b {
                    Reg::W => self.w,
                    Reg::X => self.x,
                    Reg::Y => self.y,
                    Reg::Z => self.z,
                };
                match a {
                    Reg::W => self.w %= val,
                    Reg::X => self.x %= val,
                    Reg::Y => self.y %= val,
                    Reg::Z => self.z %= val,
                }
            },
            IS::EqlC(a, b) => {
                match a {
                    Reg::W => self.w = if self.w == b { 1 } else { 0 },
                    Reg::X => self.x = if self.x == b { 1 } else { 0 },
                    Reg::Y => self.y = if self.y == b { 1 } else { 0 },
                    Reg::Z => self.z = if self.z == b { 1 } else { 0 },
                }
            },
            IS::EqlR(a, b) => {
                let val = match b {
                    Reg::W => self.w,
                    Reg::X => self.x,
                    Reg::Y => self.y,
                    Reg::Z => self.z,
                };
                match a {
                    Reg::W => self.w = if self.w == val { 1 } else { 0 },
                    Reg::X => self.x = if self.x == val { 1 } else { 0 },
                    Reg::Y => self.y = if self.y == val { 1 } else { 0 },
                    Reg::Z => self.z = if self.z == val { 1 } else { 0 },
                }
            },
        }
    }
}

fn parse_input(input: &str) -> Vec<IS> {
    let mut prog: Vec<IS> = Vec::new();

    for line in input.trim().lines() {
        let mut cmd = line.split_whitespace();
        match cmd.next().unwrap() {
            "inp" => {
                match cmd.next().unwrap() {
                    "w" => prog.push(IS::Inp(Reg::W)),
                    "x" => prog.push(IS::Inp(Reg::X)),
                    "y" => prog.push(IS::Inp(Reg::Y)),
                    "z" => prog.push(IS::Inp(Reg::Z)),
                    _ => unreachable!()
                }
            },
            "add" => {
                let dst = match cmd.next().unwrap() {
                    "w" => Reg::W,
                    "x" => Reg::X,
                    "y" => Reg::Y,
                    "z" => Reg::Z,
                    _ => unreachable!()
                };
                match cmd.next().unwrap() {
                    "w" => prog.push(IS::AddR(dst, Reg::W)),
                    "x" => prog.push(IS::AddR(dst, Reg::X)),
                    "y" => prog.push(IS::AddR(dst, Reg::Y)),
                    "z" => prog.push(IS::AddR(dst, Reg::Z)),
                    v => prog.push(IS::AddC(dst, v.parse::<i16>().unwrap())),
                }
            },
            "mul" => {
                let dst = match cmd.next().unwrap() {
                    "w" => Reg::W,
                    "x" => Reg::X,
                    "y" => Reg::Y,
                    "z" => Reg::Z,
                    _ => unreachable!()
                };
                match cmd.next().unwrap() {
                    "w" => prog.push(IS::MulR(dst, Reg::W)),
                    "x" => prog.push(IS::MulR(dst, Reg::X)),
                    "y" => prog.push(IS::MulR(dst, Reg::Y)),
                    "z" => prog.push(IS::MulR(dst, Reg::Z)),
                    v => prog.push(IS::MulC(dst, v.parse::<i16>().unwrap())),
                }
            },
            "div" => {
                let dst = match cmd.next().unwrap() {
                    "w" => Reg::W,
                    "x" => Reg::X,
                    "y" => Reg::Y,
                    "z" => Reg::Z,
                    _ => unreachable!()
                };
                match cmd.next().unwrap() {
                    "w" => prog.push(IS::DivR(dst, Reg::W)),
                    "x" => prog.push(IS::DivR(dst, Reg::X)),
                    "y" => prog.push(IS::DivR(dst, Reg::Y)),
                    "z" => prog.push(IS::DivR(dst, Reg::Z)),
                    v => prog.push(IS::DivC(dst, v.parse::<i16>().unwrap())),
                }
            },
            "mod" => {
                let dst = match cmd.next().unwrap() {
                    "w" => Reg::W,
                    "x" => Reg::X,
                    "y" => Reg::Y,
                    "z" => Reg::Z,
                    _ => unreachable!()
                };
                match cmd.next().unwrap() {
                    "w" => prog.push(IS::ModR(dst, Reg::W)),
                    "x" => prog.push(IS::ModR(dst, Reg::X)),
                    "y" => prog.push(IS::ModR(dst, Reg::Y)),
                    "z" => prog.push(IS::ModR(dst, Reg::Z)),
                    v => prog.push(IS::ModC(dst, v.parse::<i16>().unwrap())),
                }
            },
            "eql" => {
                let dst = match cmd.next().unwrap() {
                    "w" => Reg::W,
                    "x" => Reg::X,
                    "y" => Reg::Y,
                    "z" => Reg::Z,
                    _ => unreachable!()
                };
                match cmd.next().unwrap() {
                    "w" => prog.push(IS::EqlR(dst, Reg::W)),
                    "x" => prog.push(IS::EqlR(dst, Reg::X)),
                    "y" => prog.push(IS::EqlR(dst, Reg::Y)),
                    "z" => prog.push(IS::EqlR(dst, Reg::Z)),
                    v => prog.push(IS::EqlC(dst, v.parse::<i16>().unwrap())),
                }
            },
            _ => unreachable!()
        }
    }
    prog
}

fn monad(prog: &[IS], input: &[i16]) -> Option<u64> {
    let mut machine = Machine::default();
    machine.prog.extend(prog.iter());
    machine.inp.extend(input.iter());

    machine.run();

    if machine.z == 0 {
        Some(from_digits(input))
    } else {
        None
    }
}

fn to_digits(x: u64) -> Option<[i16; 14]> {
    let mut res: [i16; 14] = [1; 14];

    for d in 0..14 {
        let digit = ((x / 10_u64.pow(13-d) ) % 10) as i16;
        // println!("Digit {} = {}", 13-d, digit);
        if digit == 0 {
            return None;
        }
        res[d as usize] = digit;
    }

    Some(res)
}

fn from_digits(x: &[i16]) -> u64 {
    let mut res = 0_u64;

    for d in 0..x.len() {
        res += x[d] as u64 * 10_u64.pow(((x.len()-1)-d) as u32)
    }

    res
}

fn translated(inp: [i16; 14]) -> i16 {
    let mut w: i16 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut z: i16 = 0;

    let mut ictr: usize = 0;

    w = inp[ictr];
    ictr += 1;

    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 11;
    x = (x==w).into();
    x = (x==0).into();
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 7;
    y *= x;
    z += y;

    w = inp[ictr];
    ictr += 1;

    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 14;
    x = (x==w).into();
    x = (x==0).into();
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 8;
    y *= x;
    z += y;

    w = inp[ictr];
    ictr += 1;

    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 10;
    x = (x==w).into();
    x = (x==0).into();
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 16;
    y *= x;
    z += y;

    w = inp[ictr];
    ictr += 1;

    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 14;
    x = (x==w).into();
    x = (x==0).into();
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 8;
    y *= x;
    z += y;

    w = inp[ictr];
    ictr += 1;

    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -8;
    x = (x==w).into();
    x = (x==0).into();
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 3;
    y *= x;
    z += y;

    w = inp[ictr];
    ictr += 1;

    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 14;
    x = (x==w).into();
    x = (x==0).into();
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 12;
    y *= x;
    z += y;

    w = inp[ictr];
    ictr += 1;

    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -11;
    x = (x==w).into();
    x = (x==0).into();
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 1;
    y *= x;
    z += y;

    w = inp[ictr];
    ictr += 1;

    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 10;
    x = (x==w).into();
    x = (x==0).into();
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 8;
    y *= x;
    z += y;

    w = inp[ictr];
    ictr += 1;

    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -6;
    x = (x==w).into();
    x = (x==0).into();
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 8;
    y *= x;
    z += y;

    w = inp[ictr];
    ictr += 1;

    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -9;
    x = (x==w).into();
    x = (x==0).into();
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 14;
    y *= x;
    z += y;

    w = inp[ictr];
    ictr += 1;

    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 12;
    x = (x==w).into();
    x = (x==0).into();
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 4;
    y *= x;
    z += y;

    w = inp[ictr];
    ictr += 1;

    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -5;
    x = (x==w).into();
    x = (x==0).into();
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 14;
    y *= x;
    z += y;

    w = inp[ictr];
    ictr += 1;

    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -4;
    x = (x==w).into();
    x = (x==0).into();
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 15;
    y *= x;
    z += y;

    w = inp[ictr];
    ictr += 1;

    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -9;
    x = (x==w).into();
    x = (x==0).into();
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 6;
    y *= x;
    z += y;

    z
}


#[aoc(day24, part1)]
pub fn part1(input: &str) -> u64 {
    let prog = parse_input(input);
    println!("{:#?}", prog);

    let result = (99988888888888_u64..=99999999999999).into_par_iter().find_map_last(|v| {
        if let Some(digits) = to_digits(v) {
            if translated(digits) == 0 {
                Some(v)
            } else {
                None
            }
        } else {
            None
        }
    });

    println!("result: {:?}", result);

    result.unwrap_or(0)
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> u32 {
    let prog = parse_input(input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"inp z
inp x
mul z 3
eql z x
"#;
    const INPUT_1: &str = r#"inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y
"#;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(INPUT_1), 1);
    }

    #[test]
    fn to_digits_test() {
        let result = (99999999999989..=99999999999991).rev().map(to_digits).collect::<Vec<_>>();
        let expected = vec![Some([9,9,9,9,9,9,9,9,9,9,9,9,9,1]), None, Some([9,9,9,9,9,9,9,9,9,9,9,9,8,9])];
        assert_eq!(result, expected);
    }

    #[test]
    fn from_digits_test() {
        assert_eq!(from_digits(&[9,9,9,9,9,9,9,9,9,9,9,9,9,1]), 99999999999991);
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(EXAMPLE_INPUT), 6)
    }
}
